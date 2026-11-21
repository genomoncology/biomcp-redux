//! Summarize FDA date cells while preserving every distinct published value.
use super::decode::MONTHS;
use biodata::PartialDate;
use biomcp_fetch::BadReply;

pub(super) struct Dates {
    pub(super) values: Box<[Box<str>]>,
    pub(super) earliest: Option<Box<str>>,
    pub(super) unreported: Option<u32>,
}
impl Dates {
    pub(super) fn read<'a>(cells: impl Iterator<Item = Option<&'a str>>) -> Self {
        let mut missing = 0;
        let mut dates: Vec<_> = cells
            .filter_map(|cell| match cell {
                Some(cell) => Some((DateOrder::of(cell), cell)),
                None => {
                    missing += 1;
                    None
                }
            })
            .collect();
        // Dates with no recognized calendar spelling stay visible, after ordered dates.
        dates.sort_unstable();
        dates.dedup();
        let earliest = dates
            .first()
            .and_then(|(key, cell)| matches!(key, DateOrder::Known(..)).then(|| Box::from(*cell)));
        let unreported = (missing > 0 && !dates.is_empty()).then_some(missing);
        let values = dates.into_iter().map(|(_, cell)| cell.into()).collect();
        Self {
            values,
            earliest,
            unreported,
        }
    }

    pub(super) fn differing(self) -> Box<[Box<str>]> {
        if self.values.len() > 1 || self.unreported.is_some() || self.earliest.is_none() {
            self.values
        } else {
            Box::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum DateOrder {
    Known(u16, u8, u8),
    Unordered,
}

impl DateOrder {
    fn of(cell: &str) -> Self {
        let parsed = cell.parse::<PartialDate>().or_else(|_| calendar_date(cell));
        match parsed {
            Ok(date) => match (date.month(), date.day()) {
                (Some(month), Some(day)) => Self::Known(date.year(), month, day),
                _ => Self::Unordered,
            },
            Err(_) => Self::Unordered,
        }
    }
}

fn calendar_date(cell: &str) -> Result<PartialDate, BadReply> {
    let (month, day, year) = if let Some((month, rest)) = cell.split_once(' ') {
        let (day, year) = rest.split_once(',').ok_or(BadReply)?;
        (month, day, year.trim())
    } else {
        let (day, rest) = cell.split_once('-').ok_or(BadReply)?;
        let (month, year) = rest.split_once('-').ok_or(BadReply)?;
        (month, day, year)
    };
    let month = MONTHS
        .iter()
        .position(|name| {
            name.eq_ignore_ascii_case(month)
                || name
                    .get(..3)
                    .is_some_and(|short| short.eq_ignore_ascii_case(month))
        })
        .ok_or(BadReply)?
        + 1;
    let day = day.parse().map_err(|_| BadReply)?;
    let year_number = year.parse::<u16>().map_err(|_| BadReply)?;
    // Two-digit years in the historical FDA export refer to 21st-century biosimilar dates.
    let year = if year.len() == 2 {
        2000 + year_number
    } else {
        year_number
    };
    PartialDate::new(year, Some(month as u8), Some(day)).map_err(|_| BadReply)
}
