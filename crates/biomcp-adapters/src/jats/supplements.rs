use biodata::{Markdown, Text};
use biomcp_fetch::BadReply;
/// Explicit JATS supplement declaration before the bucket intersection.
#[derive(Debug)]
pub struct Supplement {
    /// Source filename reference.
    pub href: String,
    /// Explicit source MIME declaration, when present.
    pub kind: Option<biodata::FileKind>,
    /// Optional source label.
    pub label: Option<Text>,
    /// Optional source caption.
    pub caption: Option<Markdown>,
}
pub(super) fn read(xml: &str) -> Result<Vec<Supplement>, BadReply> {
    let mut out = Vec::new();
    for item in super::blocks::descendants(xml, "supplementary-material")? {
        let label = item
            .value(&["label"])?
            .filter(|v| !v.trim().is_empty())
            .map(|v| Text::new(&v).map_err(|_| BadReply))
            .transpose()?;
        let caption = item
            .child("caption")?
            .map(|v| super::markdown(v.0))
            .transpose()?;
        let mut href = item.attr("xlink:href")?;
        let mut kind = mime(item)?;
        if href.is_none() {
            for media in super::blocks::descendants(item.0, "media")? {
                if let Some(value) = media.attr("xlink:href")? {
                    href = Some(value);
                    kind = mime(media)?.or(kind);
                    break;
                }
            }
        }
        if let Some(href) = href {
            out.push(Supplement {
                href,
                kind,
                label,
                caption,
            });
        }
    }
    Ok(out)
}

fn mime(item: crate::xml_element::Element<'_>) -> Result<Option<biodata::FileKind>, BadReply> {
    use biodata::FileKind;
    let Some(top) = item.attr("mimetype")? else { return Ok(None); };
    let Some(sub) = item.attr("mime-subtype")? else { return Ok(Some(FileKind::Other)); };
    let kind = match format!("{top}/{sub}").to_ascii_lowercase().as_str() {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => FileKind::Xlsx,
        "text/csv" => FileKind::Csv,
        "text/tab-separated-values" => FileKind::Tsv,
        "text/plain" => FileKind::Txt,
        "application/pdf" => FileKind::Pdf,
        "application/octet-stream" | "binary/octet-stream" => return Ok(None),
        _ => FileKind::Other,
    };
    Ok(Some(kind))
}
