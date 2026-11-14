//! FDA-published PDF text; article downloads retain their existing behavior.
use biomcp_fetch::{Leg, RequestError, SourceSpec};
use biomcp_types::{PdfText, PdfTextState, Reason, Source, SourceNotice};
/// Maximum pages admitted before extracting text.
pub const MAX_PAGES: usize = 100;
/// Maximum expanded stream or total extracted text size.
pub const MAX_EXPANDED_BYTES: usize = 4 * 1024 * 1024;
/// Escaped JSON text bytes returned in one continuable fragment.
pub const FRAGMENT_BYTES: usize = 6144;
fn error(source: Source, reason: Reason) -> RequestError {
    biomcp_fetch::SourceError {
        source,
        reason,
        status: Some(200),
        tls_cause: None,
        retry_after: None,
    }
    .into()
}
/// Say that FDA refused one document download, naming its URL and FDA's status.
///
/// FDA has answered automated PDF requests with 404 (its abuse-detection apology
/// page, pm jobs j38804 and j38812) and 406 (pm jobs j40615 and j40668). The status
/// is the only reason BioMCP reads, so the notice reports it without guessing why.
pub fn refusal(url: &biodata::Text, error: &RequestError) -> Option<SourceNotice> {
    let RequestError::Source(source) = error else {
        return None;
    };
    let status = source
        .status
        .filter(|status| source.reason == Reason::Rejected && (400..500).contains(status))?;
    let meaning = match status {
        401 => " Unauthorized",
        403 => " Forbidden",
        404 => " Not Found",
        406 => " Not Acceptable",
        410 => " Gone",
        429 => " Too Many Requests",
        _ => "",
    };
    Some(SourceNotice::RequestGuidance {
        data_date: None,
        message: format!(
            "FDA refused the download of {} with HTTP {status}{meaning} when BioMCP asked \
             for the PDF. FDA has refused PDFs that open in a browser, so this does not \
             show that the document is missing; open the URL to check.",
            url.as_str()
        )
        .into(),
        retry_after_seconds: None,
    })
}
/// Download only the FDA source's admitted origin, without credentials, and read embedded text.
pub fn read(
    leg: &Leg<'_>,
    spec: &'static SourceSpec,
    url: &biodata::Text,
    offset: u32,
) -> Result<PdfText, RequestError> {
    let (mut result, text) = whole(leg, spec, url)?;
    result.offset = offset;
    result.text = crate::text_parts::split(&text, FRAGMENT_BYTES)
        .into_iter()
        .nth(offset as usize)
        .map(Into::into);
    Ok(result)
}
/// Byte positions where each continuable fragment of the whole extracted text starts.
pub fn fragment_starts(text: &str) -> Vec<usize> {
    let mut start = 0;
    crate::text_parts::split(text, FRAGMENT_BYTES)
        .iter()
        .map(|part| {
            let at = start;
            start += part.len();
            at
        })
        .collect()
}
/// Read the whole embedded text once; fragment fields stay unset and the text is empty
/// unless the state is text.
pub fn whole(
    leg: &Leg<'_>,
    spec: &'static SourceSpec,
    url: &biodata::Text,
) -> Result<(PdfText, String), RequestError> {
    let mut result = PdfText {
        url: url.clone(),
        pages: None,
        state: PdfTextState::NoText,
        offset: 0,
        total: 0,
        text: None,
    };
    // FDA's historical document links use HTTP; retain that published spelling in the reply.
    let https = url
        .as_str()
        .strip_prefix("http://www.accessdata.fda.gov/")
        .map(|path| format!("https://www.accessdata.fda.gov/{path}"));
    let bytes =
        match super::download::from_source(leg, spec, https.as_deref().unwrap_or(url.as_str())) {
            Ok(bytes) => bytes,
            Err(RequestError::Source(source)) if source.reason == Reason::TooLarge => {
                result.state = PdfTextState::TooLarge;
                return Ok((result, String::new()));
            }
            Err(error) => return Err(error),
        };
    if !bytes.starts_with(b"%PDF-") {
        return Err(error(spec.source, Reason::NotPdf));
    }
    let deadline = leg.local_deadline();
    let check = || {
        if deadline.expired() {
            Err(error(spec.source, Reason::DeadlineExceeded))
        } else {
            Ok(())
        }
    };
    check()?;
    let document = lopdf::Document::load_mem_with_options(
        &bytes,
        lopdf::LoadOptions::with_max_decompressed_size(MAX_EXPANDED_BYTES),
    )
    .map_err(|issue| match issue {
        lopdf::Error::Decompress(lopdf::DecompressError::MemoryLimitExceeded { .. }) => {
            error(spec.source, Reason::TooLarge)
        }
        _ => error(spec.source, Reason::BadReply),
    })?;
    check()?;
    let pages = document.get_pages();
    result.pages = Some(pages.len() as u32);
    if pages.len() > MAX_PAGES {
        result.state = PdfTextState::TooLarge;
        return Ok((result, String::new()));
    }
    let mut text = String::new();
    for page in pages.keys() {
        check()?;
        let part = match document.extract_text_with_limit(&[*page], MAX_EXPANDED_BYTES) {
            Ok(part) => part,
            Err(lopdf::Error::Decompress(lopdf::DecompressError::MemoryLimitExceeded {
                ..
            })) => {
                result.state = PdfTextState::TooLarge;
                return Ok((result, String::new()));
            }
            Err(_) => return Err(error(spec.source, Reason::BadReply)),
        };
        if text.len().saturating_add(part.len()) > MAX_EXPANDED_BYTES {
            result.state = PdfTextState::TooLarge;
            return Ok((result, String::new()));
        }
        text.push_str(&part);
    }
    check()?;
    if text.trim().is_empty() {
        return Ok((result, String::new()));
    }
    result.state = PdfTextState::Text;
    result.total = fragment_starts(&text).len() as u32;
    Ok((result, text))
}
