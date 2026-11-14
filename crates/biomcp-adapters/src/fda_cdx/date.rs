//! The row date: the newest date FDA prints in a row's own device cell.
//!
//! FDA's live page (2026-11-14) prints every date in parentheses in the device cell, one per
//! listed device number, as `MM/DD/YYYY` or once `M/DD/YYYY`. Two cells are malformed there
//! (`08/22/0218`, `06/28/ 2024`); they are not repaired, so those rows have no date.
use biodata::PartialDate;

/// FDA's premarket approval pathway began in 1976, so an earlier year is a typo.
const FIRST_YEAR: u16 = 1976;

/// The newest date when every parenthesized group in the cell is one valid date.
pub(super) fn row_date(cell: &str) -> Option<PartialDate> {
    let mut newest: Option<PartialDate> = None;
    let mut rest = cell;
    while let Some(open) = rest.find('(') {
        let after = rest.get(open + 1..)?;
        let close = after.find(')')?;
        let date = date(after.get(..close)?)?;
        if newest.is_none_or(|v| key(date) > key(v)) {
            newest = Some(date);
        }
        rest = after.get(close + 1..)?;
    }
    if rest.contains(')') {
        return None;
    }
    newest
}

/// Year, month and day for ordering.
pub(super) fn key(date: PartialDate) -> (u16, u8, u8) {
    (
        date.year(),
        date.month().unwrap_or_default(),
        date.day().unwrap_or_default(),
    )
}

fn date(text: &str) -> Option<PartialDate> {
    let text = text.trim();
    let mut parts = text.split('/');
    let (month, day, year) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let year = digits(year, 4).filter(|y| *y >= FIRST_YEAR)?;
    let month = u8::try_from(digits(month, 2)?).unwrap_or(u8::MAX);
    let day = u8::try_from(digits(day, 2)?).unwrap_or(u8::MAX);
    PartialDate::new(year, Some(month), Some(day)).ok()
}

/// A number written in one to `widest` ASCII digits.
fn digits(part: &str, widest: usize) -> Option<u16> {
    if part.is_empty() || part.len() > widest {
        return None;
    }
    part.bytes().try_fold(0u16, |number, byte| {
        byte.is_ascii_digit()
            .then(|| number * 10 + u16::from(byte - b'0'))
    })
}
