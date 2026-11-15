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
/// The HTTP status of a download the source refused (4xx), with its standard name.
pub fn refused(error: &biomcp_fetch::RequestError) -> Option<(u16, &'static str)> {
    let biomcp_fetch::RequestError::Source(source) = error else {
        return None;
    };
    let status = source.status.filter(|status| {
        source.reason == biomcp_types::Reason::Rejected && (400..500).contains(status)
    })?;
    let meaning = match status {
        401 => " Unauthorized",
        403 => " Forbidden",
        404 => " Not Found",
        406 => " Not Acceptable",
        410 => " Gone",
        429 => " Too Many Requests",
        _ => "",
    };
    Some((status, meaning))
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
