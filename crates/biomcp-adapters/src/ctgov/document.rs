//! Registry documents reuse the credential-free public download boundary.
use biodata::TrialDocument;
use biomcp_fetch::{Leg, RequestError, SourceError};
use biomcp_types::Reason;
/// Download only a source manifest entry produced by the checked registry projection.
pub fn download(leg: &Leg<'_>, document: &TrialDocument) -> Result<Vec<u8>, RequestError> {
    if !document
        .url
        .as_str()
        .starts_with(&format!("{}/large-docs/", super::DOCUMENT_ORIGIN))
    {
        return Err(document_error(Reason::Rejected));
    }
    let endpoint = super::SPEC
        .download(document.url.as_str())?
        .bytes(crate::pdf::MAX_BYTES);
    let bytes = leg.bytes(&endpoint, &[])?;
    if !bytes.starts_with(b"%PDF-") {
        return Err(document_error(Reason::NotPdf));
    }
    Ok(bytes)
}
fn document_error(reason: Reason) -> RequestError {
    SourceError {
        source: biomcp_types::Source::ClinicalTrialsGov,
        reason,
        status: None,
        retry_after: None,
    }
    .into()
}
