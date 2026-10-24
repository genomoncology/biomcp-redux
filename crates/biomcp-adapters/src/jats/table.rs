use super::{
    blocks,
    body::{BodyRow, Group},
    markdown,
};
use crate::xml_element::Element;
use biodata::{Text, TextPartKind};
use biomcp_fetch::BadReply;
const MAX_COLUMNS: usize = 256;
pub(super) fn group(table: Element<'_>, path: &[Text]) -> Result<Group, BadReply> {
    let label = table.value(&["label"])?.unwrap_or_else(|| "Table".into());
    let caption = table.child("caption")?;
    let text = caption
        .map(|v| markdown::inline(v.0))
        .transpose()?
        .unwrap_or_default();
    let caption = format!("**{}.** {}", label.trim_end_matches('.'), text.trim());
    let mut source = Vec::new();
    for row in blocks::descendants(table.0, "tr")? {
        source.push(cells(row)?);
    }
    let width = source.iter().map(Vec::len).max().unwrap_or(0);
    let mut rendered = Vec::new();
    for cells in source {
        let plain = cells
            .iter()
            .map(|(_, p)| p.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let mut md: Vec<_> = cells
            .into_iter()
            .map(|(m, _)| m.replace('|', "\\|").replace('\n', "<br>"))
            .collect();
        md.resize(width, String::new());
        rendered.push(BodyRow {
            plain,
            markdown: format!("| {} |", md.join(" | ")),
            context: false,
            references: Box::default(),
        });
    }
    let mut rows = vec![BodyRow {
        plain: table.value(&["caption"])?.unwrap_or_default(),
        markdown: caption.clone(),
        context: true,
        references: Box::default(),
    }];
    let mut context = caption;
    if !rendered.is_empty() {
        let mut header = rendered.remove(0);
        header.context = true;
        let separator = format!("| {} |", vec!["---"; width].join(" | "));
        context.push_str(&format!("\n\n{}\n{separator}", header.markdown));
        rows.push(header);
        rows.push(BodyRow {
            plain: String::new(),
            markdown: separator,
            context: true,
            references: Box::default(),
        });
    }
    rows.extend(rendered);
    Ok(Group {
        title: label,
        path: path.to_vec(),
        kind: TextPartKind::Table,
        context: Some(context),
        rows,
    })
}
fn cells(row: Element<'_>) -> Result<Vec<(String, String)>, BadReply> {
    let mut out = Vec::new();
    for (name, cell) in blocks::body_nodes(row.0)? {
        if !matches!(name.as_str(), "td" | "th") {
            continue;
        }
        let span = cell
            .attr("colspan")?
            .map(|v| v.parse::<usize>().map_err(|_| BadReply))
            .transpose()?
            .unwrap_or(1);
        if span == 0 || span > MAX_COLUMNS || out.len() + span > MAX_COLUMNS {
            return Err(BadReply);
        }
        let value = (markdown::inline(cell.0)?.trim().to_owned(), cell.text()?);
        out.extend(std::iter::repeat_n(value, span));
    }
    Ok(out)
}
