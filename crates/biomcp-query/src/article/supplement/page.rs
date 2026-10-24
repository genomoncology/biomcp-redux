use crate::{Action, Outcome};
use biodata::{
    ArticleSupplement, PageInfo, Request, SheetInfo, SheetRow, StepKind, SupplementContent, Text,
};
use biomcp_adapters::sheet;
const PAGE_BYTES: usize = 4000;
const FRAGMENT_BYTES: usize = 1000;
pub(super) fn select(
    query: &ArticleSupplement,
    sheets: &[sheet::Sheet],
    content: &mut SupplementContent,
    out: &mut Outcome,
) -> bool {
    if query.rows().is_none() && query.needles().is_empty() {
        content.sheets = Some(
            sheets
                .iter()
                .filter_map(|s| {
                    Some(SheetInfo {
                        name: Text::new(&s.name).ok()?,
                        rows: s.rows.last().map_or(0, |r| r.number),
                    })
                })
                .collect(),
        );
        if let Some(first) = sheets.first()
            && let Ok(name) = Text::new(&first.name)
        {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::ArticleSupplement(
                        query.first_rows(name, first.rows.last().map_or(1, |r| r.number)),
                    ),
                    fresh: false,
                },
            ));
        }
        return true;
    }
    if query.rows().is_some()
        && (sheets.len() > 1 && query.sheet().is_none()
            || query
                .sheet()
                .is_some_and(|name| !sheets.iter().any(|s| s.name == name.as_str())))
    {
        return false;
    }
    let needles: Vec<_> = query.needles().iter().map(|v| v.to_lowercase()).collect();
    let mut matching = Vec::new();
    for sheet in sheets {
        if query.sheet().is_some_and(|v| v.as_str() != sheet.name) {
            continue;
        }
        let header = sheet
            .rows
            .first()
            .map(|r| r.cells.as_slice())
            .unwrap_or(&[]);
        for row in &sheet.rows {
            let included = if let Some(range) = query.rows() {
                row.number >= range.0 && row.number <= range.1
            } else {
                row.cells
                    .iter()
                    .any(|v| needles.iter().any(|n| v.to_lowercase().contains(n)))
            };
            if !included {
                continue;
            }
            let Ok(name) = Text::new(&sheet.name) else {
                return false;
            };
            fragments(&mut matching, name, row, header);
        }
    }
    let Ok(offset) = usize::try_from(query.offset()) else {
        return false;
    };
    if offset > matching.len() {
        return false;
    }
    let mut page = Vec::new();
    let mut bytes = 0;
    for row in matching.iter().skip(offset).take(25) {
        let Ok(encoded) = serde_json::to_vec(row) else {
            return false;
        };
        if !page.is_empty() && bytes + encoded.len() > PAGE_BYTES {
            break;
        }
        bytes += encoded.len();
        page.push(row.clone());
    }
    let Ok(returned) = u32::try_from(page.len()) else {
        return false;
    };
    let Some(next) = query.offset().checked_add(returned) else {
        return false;
    };
    if (next as usize) < matching.len() && returned > 0 {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleSupplement(query.page(next)),
                fresh: false,
            },
        ));
    }
    out.page = Some(PageInfo {
        offset: query.offset(),
        returned,
        total: matching.len() as u64,
        pool: None,
        truncated: None,
    });
    content.rows = Some(page.into());
    true
}
fn fragments(out: &mut Vec<SheetRow>, sheet: Text, row: &sheet::Row, header: &[String]) {
    let simple = SheetRow {
        sheet: sheet.clone(),
        row: row.number,
        header: header.into(),
        cells: row.cells.clone().into(),
        fragment: None,
    };
    if serde_json::to_vec(&simple).is_ok_and(|v| v.len() <= FRAGMENT_BYTES) {
        out.push(simple);
        return;
    }
    // Preserve every original column and character through explicit sparse fragments.
    for (is_header, cells) in [(true, header), (false, row.cells.as_slice())] {
        for (column, value) in cells.iter().enumerate() {
            let mut offset = 0;
            for chunk in chunks(value) {
                out.push(SheetRow {
                    sheet: sheet.clone(),
                    row: row.number,
                    header: Box::default(),
                    cells: Box::default(),
                    fragment: Some(biodata::CellFragment {
                        header: is_header,
                        column: column as u32 + 1,
                        offset,
                        text: chunk.clone(),
                    }),
                });
                offset += chunk.chars().count() as u32;
            }
        }
    }
}
fn chunks(value: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut chunk = String::new();
    let mut bytes = 0;
    for c in value.chars() {
        let cost = if c == '"' || c == '\\' || c.is_control() {
            6
        } else {
            c.len_utf8()
        };
        if bytes + cost > FRAGMENT_BYTES && !chunk.is_empty() {
            chunks.push(std::mem::take(&mut chunk));
            bytes = 0;
        }
        chunk.push(c);
        bytes += cost;
    }
    if !chunk.is_empty() || chunks.is_empty() {
        chunks.push(chunk);
    }
    chunks
}
