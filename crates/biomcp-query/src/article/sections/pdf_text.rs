//! PDF part and literal find selection share the stored original page positions.
use super::pdf_parts::Document;
use crate::{Action, Outcome};
use biodata::{
    ArticleText, Markdown, PageInfo, Request, StepKind, TextContent, TextHit, TextOrigin,
};

pub(super) fn select(
    document: &Document,
    query: &ArticleText,
    selection: &crate::article::protein_change::Selection,
    out: &mut Outcome,
) -> Option<TextContent> {
    let mut content = TextContent {
        origin: Some(TextOrigin::Pdf),
        first_page: None,
        last_page: None,
        forms: selection.forms.clone(),
        license: document.outline.license,
        processing_version: None,
        manuscript: None,
        scope: biodata::Text::new("Publisher PDF text; layout, tables and figures may be lost")
            .ok()?,
        part: None,
        markdown: None,
        heading_path: None,
        hits: None,
    };
    if let Some(number) = query.selected_part() {
        let part = document
            .parts
            .iter()
            .find(|part| part.meta.number == number)?;
        content.part = Some(number);
        content.first_page = Some(part.meta.first_page);
        content.last_page = Some(part.meta.last_page);
        content.markdown = Markdown::new(
            &part
                .rows
                .iter()
                .map(|row| row.text.as_str())
                .collect::<Vec<_>>()
                .join("\n\n"),
        )
        .ok();
        if let Some(number) = number
            .checked_add(1)
            .filter(|n| document.parts.iter().any(|part| part.meta.number == *n))
        {
            let query = ArticleText::pdf_part(query.input().clone(), number).ok()?;
            push(out, query);
        }
    } else {
        content.hits = Some(find(document, query, selection, out)?);
    }
    Some(content)
}
fn find(
    document: &Document,
    query: &ArticleText,
    selection: &crate::article::protein_change::Selection,
    out: &mut Outcome,
) -> Option<Box<[TextHit]>> {
    let needles: Vec<_> = selection
        .needles
        .iter()
        .map(|term| term.to_lowercase())
        .collect();
    let all: Vec<_> = document
        .parts
        .iter()
        .flat_map(|part| part.rows.iter().map(move |row| (part, row)))
        .collect();
    let matching: std::collections::BTreeSet<_> = all
        .chunk_by(|(_, a), (_, b)| a.paragraph == b.paragraph)
        .filter_map(|rows| {
            let text: String = rows.iter().map(|(_, row)| row.text.as_str()).collect();
            let text = text.to_lowercase();
            needles
                .iter()
                .any(|needle| text.contains(needle))
                .then(|| rows.first().map(|(_, row)| row.paragraph))
                .flatten()
        })
        .collect();
    let rows: Vec<_> = all
        .into_iter()
        .filter(|(_, row)| matching.contains(&row.paragraph))
        .collect();
    let offset = usize::try_from(query.offset()).ok()?;
    if offset > rows.len() {
        return None;
    }
    let mut hits = Vec::new();
    let mut bytes = 0;
    for (part, row) in rows.iter().skip(offset).take(25) {
        let size = biomcp_adapters::jats::escaped_bytes(&row.text) + 100;
        if !hits.is_empty() && bytes + size > 4_200 {
            break;
        }
        bytes += size;
        hits.push(TextHit {
            page: Some(row.page),
            part: part.meta.number,
            markdown: Markdown::new(&row.text).ok()?,
            heading_path: Box::default(),
        });
    }
    let returned = u32::try_from(hits.len()).ok()?;
    let total = u64::try_from(rows.len()).ok()?;
    let following = query.offset().checked_add(returned)?;
    if returned > 0 && u64::from(following) < total {
        push(out, query.page(following));
    }
    out.page = Some(PageInfo {
        offset: query.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    Some(hits.into())
}
fn push(out: &mut Outcome, query: ArticleText) {
    if out.next.len() < 5 {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleText(query),
                fresh: false,
            },
        ));
    }
}
