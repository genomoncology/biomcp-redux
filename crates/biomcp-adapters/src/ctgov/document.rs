//! Registry documents reuse the credential-free public download boundary.
use biodata::TrialDocument;
use biomcp_fetch::Input;
use biomcp_fetch::{Leg, RequestError, SourceError};
use biomcp_types::Reason;
use std::num::NonZeroU32;
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
    let bytes = leg
        .call(&endpoint, Input::Params(&[]), crate::pdf::pdf())
        .map(|received| received.value.0)?;
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
        tls_cause: None,
        retry_after: None,
    }
    .into()
}

/// Read a checked registry manifest entry with the bounded, isolated PDF reader.
pub fn read_text(
    leg: &Leg<'_>,
    document: &TrialDocument,
    first: NonZeroU32,
) -> Result<crate::pdf::DownloadedText, RequestError> {
    crate::pdf::from_download(
        leg,
        super::SPEC.source,
        &document.url,
        crate::pdf::Pages::Window(first),
        || download(leg, document),
    )
}
