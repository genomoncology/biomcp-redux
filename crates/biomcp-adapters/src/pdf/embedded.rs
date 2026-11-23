//! Embedded text of PDF bytes already in hand, bounded by pages and expanded size.
use super::page::{Halt, too_large};
use biomcp_types::{PdfMissing, PdfTextState, Reason};

/// The text read from one PDF; `text` is empty unless `state` is text, so gibberish is never
/// returned as text.
#[derive(Debug)]
pub struct Embedded {
    /// Pages in the document, when it opened.
    pub pages: Option<u32>,
    /// Whether text was read, absent, or over a bound.
    pub state: PdfTextState,
    /// The whole embedded text, pages in order.
    pub text: String,
    /// UTF-8 byte offset of each physical page, including pages without text.
    pub page_starts: Vec<usize>,
    /// Why a partial read may miss text.
    pub missing: Option<PdfMissing>,
    /// A private-use Symbol character was kept in the text.
    pub private_symbol: bool,
}

/// Read every page's embedded text from `bytes`, refusing more than `max_pages` pages or
/// [`super::MAX_EXPANDED_BYTES`] of expanded content; `expired` stops the work between pages.
pub fn embedded(
    bytes: &[u8],
    max_pages: usize,
    expired: &dyn Fn() -> bool,
) -> Result<Embedded, Reason> {
    let limit = super::MAX_EXPANDED_BYTES;
    let mut result = Embedded {
        pages: None,
        state: PdfTextState::NoText,
        text: String::new(),
        page_starts: Vec::new(),
        missing: None,
        private_symbol: false,
    };
    if !bytes.starts_with(b"%PDF-") {
        return Err(Reason::NotPdf);
    }
    let check = || {
        if expired() {
            Err(Reason::DeadlineExceeded)
        } else {
            Ok(())
        }
    };
    check()?;
    if let Some(state) = refused(bytes, expired)? {
        result.state = state;
        return Ok(result);
    }
    let mut options =
        lopdf::LoadOptions::with_max_decompressed_size(super::scan::LOAD_STREAM_BYTES);
    options.strict = true;
    let mut document = match lopdf::Document::load_mem_with_options(bytes, options) {
        Ok(document) => document,
        Err(issue) if too_large(&issue) => return Err(Reason::TooLarge),
        Err(_) => {
            // Loading is strict, so lopdf repairs nothing: a file it refuses is damaged.
            result.state = PdfTextState::Damaged;
            return Ok(result);
        }
    };
    check()?;
    super::cmap::bound(&mut document, expired)?;
    check()?;
    let pages = document.get_pages();
    result.pages = Some(u32::try_from(pages.len()).unwrap_or(u32::MAX));
    if pages.len() > max_pages {
        result.state = PdfTextState::TooLarge;
        return Ok(result);
    }
    let (mut text, mut unsound, mut skipped) = (String::new(), Vec::new(), false);
    let mut page_starts = Vec::with_capacity(pages.len());
    let mut fonts = super::cmap::Cache::default();
    for page in pages.values() {
        check()?;
        let part = match super::page::page(&document, *page, limit, &mut fonts, expired) {
            Ok(part) => part,
            Err(Halt::Expired) => return Err(Reason::DeadlineExceeded),
            Err(Halt::Pdf(issue)) if too_large(&issue) => {
                result.state = PdfTextState::TooLarge;
                return Ok(result);
            }
            Err(Halt::Pdf(_)) => {
                result.state = PdfTextState::Damaged;
                return Ok(result);
            }
        };
        if text.len().saturating_add(part.text.len()) > limit {
            result.state = PdfTextState::TooLarge;
            return Ok(result);
        }
        let at = text.len();
        page_starts.push(at);
        unsound.extend(
            part.unsound
                .iter()
                .map(|(start, end)| (at + start, at + end)),
        );
        text.push_str(&part.text);
        skipped |= part.skipped;
        result.private_symbol |= part.private_symbol;
    }
    check()?;
    Ok(settle(result, text, page_starts, &unsound, skipped))
}

/// The read's state and text once every page is read.
/// The state of a document the measuring refuses before loading, if it does.
fn refused(bytes: &[u8], expired: &dyn Fn() -> bool) -> Result<Option<PdfTextState>, Reason> {
    match super::scan::loadable(bytes, expired) {
        Ok(()) => Ok(None),
        Err(super::scan::Refusal::TooLarge) => Ok(Some(PdfTextState::TooLarge)),
        Err(super::scan::Refusal::Irregular) => Ok(Some(PdfTextState::Damaged)),
        Err(super::scan::Refusal::Expired) => Err(Reason::DeadlineExceeded),
    }
}

fn settle(
    mut result: Embedded,
    mut text: String,
    mut page_starts: Vec<usize>,
    unsound: &[(usize, usize)],
    skipped: bool,
) -> Embedded {
    let mut cut = false;
    // Only text shown in a composite font without a character map is judged; text read
    // through a map or a declared one-byte encoding is not, since tables, part numbers and
    // other languages are not gibberish. Gibberish is cut out and the rest is partial, unless
    // gibberish was most of the text.
    let shown: Vec<&str> = unsound
        .iter()
        .filter_map(|(start, end)| text.get(*start..*end))
        .collect();
    let judged = shown.join(" ");
    if super::garbled::garbled(&judged) {
        let kept = without(&text, unsound, &mut page_starts);
        if kept.trim().len() < judged.len() {
            result.state = PdfTextState::Garbled;
            return result;
        }
        text = kept;
        cut = true;
    }
    // A skipped part may have held all the text, so it is partial, never no-text.
    if !text.trim().is_empty() || skipped {
        result.missing = match (cut, skipped) {
            (true, true) => Some(PdfMissing::Both),
            (true, false) => Some(PdfMissing::Gibberish),
            (false, true) => Some(PdfMissing::Skipped),
            (false, false) => None,
        };
        result.state = if result.missing.is_some() {
            PdfTextState::Partial
        } else {
            PdfTextState::Text
        };
        result.text = text;
        result.page_starts = page_starts;
    }
    result
}

/// `text` without the byte `spans`, which are in order and do not overlap.
fn without(text: &str, spans: &[(usize, usize)], page_starts: &mut [usize]) -> String {
    let mut kept = String::with_capacity(text.len());
    let mut from = 0;
    let mut pages = page_starts.iter_mut().peekable();
    for (start, end) in spans {
        while pages.peek().is_some_and(|page| **page <= *start) {
            if let Some(page) = pages.next() {
                *page = kept.len() + (*page - from);
            }
        }
        kept.push_str(text.get(from..*start).unwrap_or_default());
        kept.push(' ');
        from = *end;
    }
    for page in pages {
        *page = kept.len() + (*page - from);
    }
    kept.push_str(text.get(from..).unwrap_or_default());
    kept
}
