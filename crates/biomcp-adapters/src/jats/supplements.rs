use biodata::{Text, Markdown};
use biomcp_fetch::BadReply;
/// Explicit JATS supplement declaration before the bucket intersection.
#[derive(Debug)]
pub struct Supplement {
    /// Source filename reference.
    pub href: String,
    /// Optional source label.
    pub label: Option<Text>,
    /// Optional source caption.
    pub caption: Option<Markdown>,
}
pub(super) fn read(xml: &str) -> Result<Vec<Supplement>, BadReply> {
    let mut out = Vec::new();
    for item in super::blocks::descendants(xml, "supplementary-material")? {
        let label = item.value(&["label"])?.filter(|v| !v.trim().is_empty()).map(|v| Text::new(&v).map_err(|_| BadReply)).transpose()?;
        let caption = item.child("caption")?.map(|v| super::markdown(v.0)).transpose()?;
        let mut href = item.attr("xlink:href")?;
        if href.is_none() {
            for media in super::blocks::descendants(item.0, "media")? {
                if let Some(value) = media.attr("xlink:href")? { href = Some(value);break; }
            }
        }
        if let Some(href) = href { out.push(Supplement { href, label, caption }); }
    }
    Ok(out)
}
