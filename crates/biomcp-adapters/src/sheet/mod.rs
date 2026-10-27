//! Bounded local spreadsheet readers; parsing never evaluates formulas or links.
mod preflight;
mod xlsx;

use biomcp_fetch::LocalDeadline;
use biomcp_types::{FileKind, Reason};
/// An original sparse sheet row.
#[derive(Debug)]
pub struct Row {
    /// Original one-based row number.
    pub number: u32,
    /// Values in original column positions.
    pub cells: Vec<String>,
}
/// One sheet, retaining original row positions.
#[derive(Debug)]
pub struct Sheet {
    /// Original name.
    pub name: String,
    /// Original populated rows.
    pub rows: Vec<Row>,
}
struct Limits {
    entries: usize,
    directory: usize,
    xml: usize,
    expanded: usize,
    strings: usize,
    text: usize,
    cells: usize,
}
const LIMITS: Limits = Limits {
    entries: 4096,
    directory: 4 * 1024 * 1024,
    xml: 16 * 1024 * 1024,
    expanded: 64 * 1024 * 1024,
    strings: 65536,
    text: 1024 * 1024,
    cells: 1000000,
};
/// Parse checked bytes under the command's remaining local deadline.
pub fn read(bytes: &[u8], kind: FileKind, end: LocalDeadline) -> Result<Vec<Sheet>, Reason> {
    check(&end)?;
    match kind {
        FileKind::Xlsx => xlsx::read(bytes, &end),
        FileKind::Csv | FileKind::Tsv => delimited(bytes, kind, end),
        FileKind::Txt => {
            let text = std::str::from_utf8(bytes).map_err(|_| Reason::BadReply)?;
            let mut rows = Vec::new();
            for (index, line) in text.lines().enumerate() {
                check(&end)?;
                rows.push(Row {
                    number: u32::try_from(index + 1).map_err(|_| Reason::TooLarge)?,
                    cells: vec![line.to_owned()],
                });
            }
            Ok(vec![Sheet {
                name: "text".into(),
                rows,
            }])
        }
        _ => Err(Reason::Unsupported),
    }
}
fn check(end: &LocalDeadline) -> Result<(), Reason> {
    if end.expired() {
        Err(Reason::DeadlineExceeded)
    } else {
        Ok(())
    }
}
fn delimited(bytes: &[u8], kind: FileKind, end: LocalDeadline) -> Result<Vec<Sheet>, Reason> {
    // The transport's checked byte ceiling bounds CSV allocation; buffer capacity is no cap.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(if kind == FileKind::Tsv { b'\t' } else { b',' })
        .from_reader(bytes);
    let mut rows = Vec::new();
    for (index, result) in reader.records().enumerate() {
        check(&end)?;
        let record = result.map_err(|_| Reason::BadReply)?;
        rows.push(Row {
            number: u32::try_from(index + 1).map_err(|_| Reason::TooLarge)?,
            cells: record.iter().map(str::to_owned).collect(),
        });
    }
    Ok(vec![Sheet {
        name: "text".into(),
        rows,
    }])
}
