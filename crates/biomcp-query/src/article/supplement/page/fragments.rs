//! Preserve original cell coordinates while returning matching text before context.
use super::Page;
use biodata::Text;
use biomcp_adapters::sheet;
use biomcp_types::{CellFragment, FirstSheetRow, Reason, SheetRow};
const FRAGMENT_BYTES: usize = 1000;
pub(super) fn contains(value: &str, needles: &[String]) -> bool {
    let lower = value.to_lowercase();
    needles.iter().any(|needle| lower.contains(needle))
}
pub(super) fn matches(row: &SheetRow, needles: &[String]) -> bool {
    row.fragment.as_ref().map_or_else(
        || row.cells.iter().any(|cell| contains(cell, needles)),
        |fragment| !fragment.first_row && contains(&fragment.text, needles),
    )
}
pub(super) fn push(
    out: &mut Page,
    sheet: Text,
    row: &sheet::Row,
    first_row: &sheet::Row,
    needles: &[String],
    matching: Option<bool>,
) -> Result<(), Reason> {
    let simple = SheetRow {
        sheet: sheet.clone(),
        row: row.number,
        first_row: FirstSheetRow { row: first_row.number, cells: first_row.cells.clone().into() },
        cells: row.cells.clone().into(),
        fragment: None,
    };
    if serde_json::to_vec(&simple).is_ok_and(|v| v.len() <= FRAGMENT_BYTES) {
        if matching != Some(false) {
            out.push(simple)?;
        }
        return Ok(());
    }
    for (is_first_row, cells) in [(true, first_row.cells.as_slice()), (false, row.cells.as_slice())] {
        for (column, value) in cells.iter().enumerate() {
            let mut offset = 0;
            for chunk in chunks(value, needles) {
                let length = chunk.chars().count() as u32;
                let hit = !is_first_row && contains(&chunk, needles);
                if matching.is_none_or(|v| v == hit) {
                    out.push(SheetRow {
                        sheet: sheet.clone(),
                        row: row.number,
                        first_row: FirstSheetRow { row: first_row.number, cells: Box::default() },
                        cells: Box::default(),
                        fragment: Some(CellFragment {
                            first_row: is_first_row,
                            column: column as u32 + 1,
                            offset,
                            text: chunk,
                        }),
                    })?;
                }
                offset += length;
            }
        }
    }
    Ok(())
}
fn chunks(value: &str, needles: &[String]) -> Vec<String> {
    let ordinary = bounded_chunks(value);
    if ordinary.iter().any(|chunk| contains(chunk, needles)) {
        return ordinary;
    }
    // Avoid hiding a literal match across a fixed-size fragment boundary.
    let lower = value.to_lowercase();
    let Some(start) = needles.iter().filter_map(|needle| lower.find(needle)).min() else {
        return ordinary;
    };
    let mut lower_offset = 0;
    let cut = value
        .char_indices()
        .find_map(|(byte, c)| {
            let width = c.to_lowercase().map(char::len_utf8).sum::<usize>();
            let found = (start < lower_offset + width).then_some(byte);
            lower_offset += width;
            found
        })
        .unwrap_or(0);
    if cut == 0 {
        return ordinary;
    }
    let mut parts = bounded_chunks(&value[..cut]);
    parts.extend(bounded_chunks(&value[cut..]));
    parts
}
fn bounded_chunks(value: &str) -> Vec<String> {
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
