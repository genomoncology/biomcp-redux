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
    let number = |part: &str, widths: &[usize]| {
        (widths.contains(&part.len()) && part.bytes().all(|b| b.is_ascii_digit()))
            .then(|| part.parse::<u16>().ok())
            .flatten()
    };
    let year = number(year, &[4]).filter(|y| *y >= FIRST_YEAR)?;
    let month = u8::try_from(number(month, &[1, 2])?).ok()?;
    let day = u8::try_from(number(day, &[1, 2])?).ok()?;
    PartialDate::new(year, Some(month), Some(day)).ok()
}

#[cfg(test)]
mod tests {
    use super::row_date;

    fn shown(cell: &str) -> Option<String> {
        row_date(cell).map(|d| d.to_string())
    }

    #[test]
    fn newest_valid_date_or_none() {
        assert_eq!(shown("DEN260027\n(10/20/2026)").as_deref(), Some("2026-10-20"));
        assert_eq!(shown("P250036\n(7/30/2026)").as_deref(), Some("2026-07-30"));
        assert_eq!(
            shown("P150013\n(10/02/2015);\u{a0}\nupdated P150013/S012\n(04/16/2019)\n\u{a0}")
                .as_deref(),
            Some("2019-04-16")
        );
        assert_eq!(shown("P120019/S019\n(08/22/0218)"), None);
        assert_eq!(shown("P190032/S014 \u{a0}\n(06/28/ 2024)"), None);
        assert_eq!(shown("P190032 (08/26/2020) P190032/S008 (02/30/2022)"), None);
        assert_eq!(shown("P190032"), None);
        assert_eq!(shown("P190032 (08/26/2020"), None);
    }
}
