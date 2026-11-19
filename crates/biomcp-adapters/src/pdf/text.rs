//! FDA-published PDF text; article downloads retain their existing behavior.
use biomcp_fetch::{Leg, RequestError, SourceSpec};
use biomcp_types::{PdfMissing, PdfText, PdfTextState, Reason, Source, SourceNotice};
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
    let (status, meaning) = super::download::refused(error)?;
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
        missing: None,
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
    let deadline = leg.local_deadline();
    let read = super::embedded(&bytes, MAX_PAGES, &|| deadline.expired())
        .map_err(|reason| error(spec.source, reason))?;
    result.pages = read.pages;
    result.state = read.state;
    result.missing = read.missing;
    if !result.state.readable() {
        return Ok((result, String::new()));
    }
    let text = read.text;
    result.total = fragment_starts(&text).len() as u32;
    Ok((result, text))
}

/// The reply notice for a partial read: what was cut or skipped, so text may be missing.
pub fn missing_notice(text: &PdfText) -> Option<SourceNotice> {
    if text.state != PdfTextState::Partial {
        return None;
    }
    let message = match text.missing {
        Some(PdfMissing::Gibberish) => {
            "Text this document draws in a font without a usable character map read as gibberish and was cut out, so some of its text is missing."
        }
        Some(PdfMissing::Skipped) => {
            "A part of a page in this document could not be read and was skipped, so some of its text may be missing."
        }
        Some(PdfMissing::Both) => {
            "Text in a font without a usable character map read as gibberish and was cut out, and a part of a page could not be read, so some of this document's text is missing."
        }
        None => "Some of this document's text could not be read, so text may be missing.",
    };
    Some(SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    })
}
