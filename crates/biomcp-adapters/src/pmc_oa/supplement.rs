use biodata::{FileKind, SupplementFile, Text};
use biomcp_fetch::{Arg, BadReply, Leg, RequestError};
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
            let location = super::location::Location::checked(url, prefix).map_err(super::bad)?;
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
                    name: Text::new(name).map_err(super::bad)?,
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
/// Download only a previously intersected readable bucket file.
pub fn download(leg: &Leg<'_>, file: &Supplement) -> Result<Vec<u8>, RequestError> {
    if !file.file.kind.readable() {
        return Err(super::bad(BadReply));
    }
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
    leg.bytes(&ep, &params)
}
