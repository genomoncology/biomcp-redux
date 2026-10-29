use biodata::Text;
use biomcp_fetch::{Arg, BadReply, Leg, RequestError};
use biomcp_types::{FileKind, SupplementFile};
/// Source-confirmed supplement with a confined bucket location.
#[derive(Debug)]
pub struct Supplement {
    /// Public file metadata.
    pub file: SupplementFile,
    /// Public provider URL returned for unsupported files.
    pub url: Text,
    location: super::location::Location,
}
pub(super) fn select(
    jats: &[crate::jats::Supplement],
    urls: &[String],
    prefix: &str,
) -> Result<Vec<Supplement>, RequestError> {
    let mut out = Vec::new();
    for item in jats {
        // Matching the exact href is intentional; no path basename or URL guessing.
        for url in urls {
            let Ok(location) = super::location::Location::checked(url, prefix) else {
                continue;
            };
            let Some(name) = location.path.strip_prefix(&format!("/{prefix}")) else {
                return Err(super::bad(BadReply));
            };
            if name != item.href
                || out
                    .iter()
                    .any(|s: &Supplement| s.file.name.as_str() == name)
            {
                continue;
            }
            let kind = match name
                .rsplit('.')
                .next()
                .unwrap_or("")
                .to_ascii_lowercase()
                .as_str()
            {
                "xlsx" => FileKind::Xlsx,
                "csv" => FileKind::Csv,
                "tsv" => FileKind::Tsv,
                "txt" => FileKind::Txt,
                "pdf" => FileKind::Pdf,
                _ => FileKind::Other,
            };
            let kind = item.kind.unwrap_or(kind);
            let public = format!(
                "{}{}{}",
                super::SPEC.base,
                location.path,
                location
                    .md5
                    .as_ref()
                    .map(|v| format!("?md5={v}"))
                    .unwrap_or_default()
            );
            out.push(Supplement {
                file: SupplementFile {
                    name: Text::from_source(name).map_err(super::bad)?,
                    kind,
                    label: item.label.clone(),
                    caption: item.caption.clone(),
                },
                url: Text::new(&public).map_err(super::bad)?,
                location,
            });
        }
    }
    Ok(out)
}
/// Original bytes and one checked parse share the same remaining local budget.
#[derive(Debug)]
pub struct Download {
    /// Original source bytes for hashing.
    pub bytes: Vec<u8>,
    /// Parsed original sheets.
    pub sheets: Vec<crate::sheet::Sheet>,
    /// Deadline retained through row selection after parsing.
    pub deadline: biomcp_fetch::LocalDeadline,
}
/// Download only a previously intersected readable bucket file.
#[expect(
    clippy::disallowed_types,
    reason = "Retain one parsed binary reply across its checked cache admission callback"
)]
pub fn download(leg: &Leg<'_>, file: &Supplement) -> Result<Download, RequestError> {
    if !file.file.kind.readable() {
        return Err(super::bad(BadReply));
    }
    let (ep, params) = request(file)?;
    let local_end = leg.local_deadline();
    let parsed = std::cell::RefCell::new(None);
    let bytes = leg.bytes_checked(&ep, &params, |bytes| {
        let end = local_end.earlier(leg.local_deadline());
        let sheets = crate::sheet::read(bytes, file.file.kind, end.clone())?;
        *parsed.borrow_mut() = Some((sheets, end));
        Ok(())
    })?;
    let (sheets, deadline) = parsed.into_inner().ok_or_else(|| super::bad(BadReply))?;
    Ok(Download {
        bytes,
        sheets,
        deadline,
    })
}

/// Download original bytes from the same checked JATS/record intersection without parsing.
/// Used only by explicit saves, including formats the row reader cannot parse.
pub fn download_bytes(leg: &Leg<'_>, file: &Supplement) -> Result<Vec<u8>, RequestError> {
    let (ep, params) = request(file)?;
    // Explicit raw saves never read or admit source file bodies in the reply cache.
    let ep = ep.fresh(biomcp_fetch::Freshness::Never);
    leg.bytes(&ep, &params)
}
type RequestParts<'a> = (biomcp_fetch::Endpoint, Vec<(&'static str, Arg<'a>)>);
fn request(file: &Supplement) -> Result<RequestParts<'_>, RequestError> {
    let ep = super::FILE
        .clone()
        .with_path(file.location.path.clone())
        .map_err(super::bad)?
        .bytes(50 * 1024 * 1024);
    let params: Vec<_> = file
        .location
        .md5
        .as_ref()
        .map(|v| ("md5", Arg::Text(v)))
        .into_iter()
        .collect();
    Ok((ep, params))
}
