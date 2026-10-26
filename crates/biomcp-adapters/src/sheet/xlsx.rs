use super::{LIMITS, Row, Sheet, check};
use biomcp_fetch::LocalDeadline;
use biomcp_types::Reason;
use calamine::Reader;
use std::io::{Cursor, Read, Seek, SeekFrom};
struct Guarded<'a> {
    cursor: Cursor<&'a [u8]>,
    end: &'a LocalDeadline,
}
impl Read for Guarded<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        check(self.end).map_err(|_| std::io::ErrorKind::TimedOut)?;
        self.cursor.read(buf)
    }
}
impl Seek for Guarded<'_> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        check(self.end).map_err(|_| std::io::ErrorKind::TimedOut)?;
        self.cursor.seek(pos)
    }
}
pub(super) fn read(bytes: &[u8], end: &LocalDeadline) -> Result<Vec<Sheet>, Reason> {
    super::preflight::xml(bytes, end)?;
    let mut workbook = calamine::Xlsx::new(Guarded {
        cursor: Cursor::new(bytes),
        end,
    })
    .map_err(|_| Reason::BadReply)?;
    let mut sheets = Vec::new();
    let mut visited = 0usize;
    let mut assembled = 0usize;
    let mut materialized = 0usize;
    for name in workbook.sheet_names() {
        check(end)?;
        let mut rows = Vec::<Row>::new();
        let mut cells = workbook
            .worksheet_cells_reader(&name)
            .map_err(|_| Reason::BadReply)?;
        while let Some(cell) = cells.next_cell().map_err(|_| Reason::BadReply)? {
            check(end)?;
            visited += 1;
            if visited > LIMITS.cells {
                return Err(Reason::TooLarge);
            }
            let (row, column) = cell.get_position();
            let value = text(cell.get_value(), &mut materialized)?;
            if value.len() > LIMITS.text || column >= 16384 {
                return Err(Reason::TooLarge);
            }
            let number = row.checked_add(1).ok_or(Reason::TooLarge)?;
            if rows.last().is_none_or(|r| r.number != number) {
                rows.push(Row {
                    number,
                    cells: Vec::new(),
                });
            }
            let current = rows.last_mut().ok_or(Reason::BadReply)?;
            let column = usize::try_from(column).map_err(|_| Reason::TooLarge)?;
            if column < current.cells.len() {
                return Err(Reason::BadReply);
            }
            assembled = assembled
                .checked_add(column + 1 - current.cells.len())
                .ok_or(Reason::TooLarge)?;
            if assembled > LIMITS.cells {
                return Err(Reason::TooLarge);
            }
            current.cells.resize(column, String::new());
            current.cells.push(value);
        }
        sheets.push(Sheet { name, rows });
    }
    Ok(sheets)
}

// Shared strings can fan out far beyond their compressed or expanded XML size.
fn text(value: &calamine::DataRef<'_>, total: &mut usize) -> Result<String, Reason> {
    use calamine::DataRef;
    let borrowed = match value {
        DataRef::String(v) | DataRef::DateTimeIso(v) | DataRef::DurationIso(v) => Some(v.as_str()),
        DataRef::SharedString(v) => Some(*v),
        _ => None,
    };
    let size = borrowed.map_or(64, str::len);
    *total = total.checked_add(size).ok_or(Reason::TooLarge)?;
    if size > LIMITS.text || *total > LIMITS.expanded {
        return Err(Reason::TooLarge);
    }
    Ok(match borrowed {
        Some(text) => text.to_owned(),
        None => calamine::Data::from(value.clone()).to_string(),
    })
}
