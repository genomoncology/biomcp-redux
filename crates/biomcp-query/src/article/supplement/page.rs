use crate::{Action, Outcome};
use biodata::{Text};
use biomcp_types::{ArticleSupplement, PageInfo, Reason, Request, SheetInfo, SheetRow, StepKind, SupplementContent};
use biomcp_adapters::sheet;
use biomcp_fetch::LocalDeadline;
const PAGE_BYTES: usize = 4000;
const FRAGMENT_BYTES: usize = 1000;
struct Page {
    offset: u32,
    total: u32,
    bytes: usize,
    rows: Vec<SheetRow>,
    full: bool,
    end: LocalDeadline,
}
impl Page {
    fn push(&mut self, row: SheetRow) -> Result<(), Reason> {
        if self.end.expired() {
            return Err(Reason::DeadlineExceeded);
        }
        let position = self.total;
        self.total = self.total.checked_add(1).ok_or(Reason::TooLarge)?;
        if position < self.offset || self.full {
            return Ok(());
        }
        let encoded = serde_json::to_vec(&row).map_err(|_| Reason::BadReply)?;
        if self.rows.len() == 25 || !self.rows.is_empty() && self.bytes + encoded.len() > PAGE_BYTES
        {
            self.full = true;
            return Ok(());
        }
        self.bytes += encoded.len();
        self.rows.push(row);
        Ok(())
    }
}
pub(super) fn select(
    query: &ArticleSupplement,
    sheets: &[sheet::Sheet],
    content: &mut SupplementContent,
    out: &mut Outcome,
    selected: &[String],
    end: LocalDeadline,
) -> Result<bool, Reason> {
    if query.rows().is_none() && selected.is_empty() {
        overview(query, sheets, content, out);
        return Ok(true);
    }
    if query.rows().is_some()
        && (sheets.len() > 1 && query.sheet().is_none()
            || query
                .sheet()
                .is_some_and(|name| !sheets.iter().any(|s| s.name == name.as_str())))
    {
        return Ok(false);
    }
    let needles: Vec<_> = selected.iter().map(|v| v.to_lowercase()).collect();
    let mut page = Page {
        offset: query.offset(),
        total: 0,
        bytes: 0,
        rows: Vec::new(),
        full: false,
        end,
    };
    for sheet in sheets {
        if query.sheet().is_some_and(|v| v.as_str() != sheet.name) {
            continue;
        }
        let header = sheet
            .rows
            .first()
            .filter(|r| r.number == 1)
            .map(|r| r.cells.as_slice())
            .unwrap_or(&[]);
        let name = Text::new(&sheet.name).map_err(|_| Reason::BadReply)?;
        for row in &sheet.rows {
            if end.expired() {
                return Err(Reason::DeadlineExceeded);
            }
            let included = if let Some(range) = query.rows() {
                row.number >= range.start() && row.number <= range.end()
            } else {
                row.cells
                    .iter()
                    .any(|v| needles.iter().any(|n| v.to_lowercase().contains(n)))
            };
            if included {
                fragments(&mut page, name.clone(), row, header)?;
            }
        }
    }
    if query.offset() > page.total {
        return Ok(false);
    }
    let returned = u32::try_from(page.rows.len()).map_err(|_| Reason::TooLarge)?;
    let next = query
        .offset()
        .checked_add(returned)
        .ok_or(Reason::TooLarge)?;
    if next < page.total && returned > 0 {
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
        total: u64::from(page.total),
        pool: None,
        truncated: None,
    });
    content.rows = Some(page.rows.into());
    Ok(true)
}
fn fragments(
    out: &mut Page,
    sheet: Text,
    row: &sheet::Row,
    header: &[String],
) -> Result<(), Reason> {
    let simple = SheetRow {
        sheet: sheet.clone(),
        row: row.number,
        header: header.into(),
        cells: row.cells.clone().into(),
        fragment: None,
    };
    if serde_json::to_vec(&simple).is_ok_and(|v| v.len() <= FRAGMENT_BYTES) {
        return out.push(simple);
    }
    // Preserve every original column and character through explicit sparse fragments.
    for (is_header, cells) in [(true, header), (false, row.cells.as_slice())] {
        for (column, value) in cells.iter().enumerate() {
            let mut offset = 0;
            for chunk in chunks(value) {
                let length = chunk.chars().count() as u32;
                out.push(SheetRow {
                    sheet: sheet.clone(),
                    row: row.number,
                    header: Box::default(),
                    cells: Box::default(),
                    fragment: Some(biomcp_types::CellFragment {
                        header: is_header,
                        column: column as u32 + 1,
                        offset,
                        text: chunk,
                    }),
                })?;
                offset += length;
            }
        }
    }
    Ok(())
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

fn overview(
    query: &ArticleSupplement,
    sheets: &[sheet::Sheet],
    content: &mut SupplementContent,
    out: &mut Outcome,
) {
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
}
