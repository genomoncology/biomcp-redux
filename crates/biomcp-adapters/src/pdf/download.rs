//! Credential-free bounded publisher downloads; PDF content is never parsed.
use biomcp_fetch::Input;
/// Maximum publisher bytes independent of ordinary API response limits.
pub const MAX_BYTES: u64 = 25 * 1024 * 1024;
/// Read a publisher file without caching or forwarding an API credential.
pub fn download(
    leg: &biomcp_fetch::Leg<'_>,
    url: &str,
) -> Result<Vec<u8>, biomcp_fetch::RequestError> {
    from_source(leg, &crate::semantic_scholar::PUBLISHER, url)
}

/// Ask for a PDF by name: FDA answered `application/octet-stream` with HTTP 406
/// from the M5 while serving `application/pdf` (pm job j40668).
pub(crate) fn pdf() -> biomcp_fetch::Read<'static, biomcp_fetch::Bytes> {
    biomcp_fetch::Read::bytes().document_media(biomcp_fetch::DocumentMedia::Pdf)
}
pub(super) fn from_source(
    leg: &biomcp_fetch::Leg<'_>,
    spec: &'static biomcp_fetch::SourceSpec,
    url: &str,
) -> Result<Vec<u8>, biomcp_fetch::RequestError> {
    let endpoint = spec.download(url)?.bytes(MAX_BYTES);
    leg.call(&endpoint, Input::Params(&[]), pdf())
        .map(|received| received.value.0)
        .map_err(|error| match error {
            biomcp_fetch::RequestError::Source(mut source)
                if source.status.is_some_and(|s| (400..500).contains(&s)) =>
            {
                source.reason = biomcp_types::Reason::Rejected;
                source.into()
            }
            error => error,
        })
}
#[cfg(test)]
mod tests {
    #[test]
    fn pdf_downloads_ask_for_application_pdf() {
        let read = format!("{:?}", super::pdf());
        assert!(read.contains("\"application/pdf\""), "{read}");
    }
}
