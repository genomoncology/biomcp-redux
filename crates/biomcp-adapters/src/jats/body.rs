use super::{blocks, markdown, parts::Part};
use crate::xml_element::Element;
use biodata::{Text, TextPartKind};
use biomcp_fetch::BadReply;
/// One source row and its rendered representation, before bounded fragmentation.
#[derive(Debug)]
pub struct BodyRow {
    /// Source text used for literal matching.
    pub plain: String,
    /// Whole rendered source row.
    pub markdown: String,
    /// This row supplies a table's caption or column headings.
    pub context: bool,
    /// Explicit citation reference identities for this source row.
    pub references: Box<[Text]>,
}
pub(super) struct Group {
    pub(super) title: String,
    pub(super) path: Vec<Text>,
    pub(super) kind: TextPartKind,
    pub(super) context: Option<String>,
    pub(super) rows: Vec<BodyRow>,
}
impl Group {
    pub(super) fn section(path: Vec<Text>) -> Self {
        Self { title: path.last().map_or("Body", Text::as_str).to_owned(), path, kind: TextPartKind::Section, context: None, rows: Vec::new() }
    }
}
pub(super) fn section(node: Element<'_>, ancestors: &[Text]) -> Result<Vec<Group>, BadReply> {
    let mut path = ancestors.to_vec();
    let title = node.value(&["title"])?;
    if let Some(title) = &title { path.push(Text::new(title).map_err(|_| BadReply)?); }
    let mut group = Group::section(path.clone());
    if let Some(title) = node.child("title")? {
        group.rows.push(BodyRow { plain: title.text()?, markdown: format!("{} {}", "#".repeat(path.len()+1), markdown::inline(title.0)?.trim()), context: false, references: super::references::ids(title.0)? });
    }
    let mut out = Vec::new();
    for (name, child) in blocks::body_nodes(node.0)? {
        match name.as_str() {
            "title" | "table-wrap" | "table-wrap-group" => {}
            "sec" => {
                if !group.rows.is_empty() { out.push(group); }
                out.extend(section(child, &path)?);
                group = Group::section(path.clone());
            }
            "fig" => {
                if let Some(row) = figure(child)? { group.rows.push(row); }
            }
            _ => {
                let plain = child.text()?;
                if !plain.trim().is_empty() { group.rows.push(BodyRow { plain, markdown: markdown::inline(child.0)?.trim().to_owned(), context: false, references: super::references::ids(child.0)? }); }
            }
        }
    }
    if !group.rows.is_empty() { out.push(group); }
    Ok(out)
}
pub(super) fn figure(fig: Element<'_>) -> Result<Option<BodyRow>, BadReply> {
    let Some(caption) = fig.child("caption")? else { return Ok(None); };
    let label = fig.value(&["label"])?.unwrap_or_else(|| "Figure".into());
    let plain = caption.text()?;
    if plain.trim().is_empty() { return Ok(None); }
    Ok(Some(BodyRow { plain, markdown: format!("**{}.** {}", label.trim_end_matches('.'), markdown::inline(caption.0)?.trim()), context: false, references: super::references::ids(caption.0)? }))
}
pub(super) fn append(parts: &mut Vec<Part>, group: Group, rows: &mut Vec<BodyRow>) -> Result<(), BadReply> {
    super::parts::append(parts, group, rows)
}
