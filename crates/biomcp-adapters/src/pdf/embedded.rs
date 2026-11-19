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
    /// Why a partial read may miss text.
    pub missing: Option<PdfMissing>,
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
        missing: None,
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
    match super::scan::loadable(bytes, expired) {
        Ok(()) => {}
        Err(super::scan::Refusal::TooLarge) => {
            result.state = PdfTextState::TooLarge;
            return Ok(result);
        }
        Err(super::scan::Refusal::Expired) => return Err(Reason::DeadlineExceeded),
        Err(super::scan::Refusal::Irregular) => {
            result.state = PdfTextState::Damaged;
            return Ok(result);
        }
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
        unsound.extend(
            part.unsound
                .iter()
                .map(|(start, end)| (at + start, at + end)),
        );
        text.push_str(&part.text);
        skipped |= part.skipped;
    }
    check()?;
    Ok(settle(result, text, &unsound, skipped))
}

/// The read's state and text once every page is read.
fn settle(
    mut result: Embedded,
    mut text: String,
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
        let kept = without(&text, unsound);
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
    }
    result
}

/// `text` without the byte `spans`, which are in order and do not overlap.
fn without(text: &str, spans: &[(usize, usize)]) -> String {
    let mut kept = String::with_capacity(text.len());
    let mut from = 0;
    for (start, end) in spans {
        kept.push_str(text.get(from..*start).unwrap_or_default());
        kept.push(' ');
        from = *end;
    }
    kept.push_str(text.get(from..).unwrap_or_default());
    kept
}
