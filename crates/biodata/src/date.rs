use crate::{Error, text_value};
use std::{fmt, str::FromStr};

/// A Gregorian date preserving year, month or day precision. The parser additionally validates the calendar day, including leap years.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PartialDate {
    year: u16,
    month: Option<u8>,
    day: Option<u8>,
}

impl PartialDate {
    /// Check components without filling missing precision.
    pub fn new(year: u16, month: Option<u8>, day: Option<u8>) -> Result<Self, Error> {
        if !(1..=9999).contains(&year) || (day.is_some() && month.is_none()) {
            return Err(Error::DateInvalid);
        }
        if let Some(month) = month {
            let days = match month {
                4 | 6 | 9 | 11 => 30,
                2 if year.is_multiple_of(400)
                    || (year.is_multiple_of(4) && !year.is_multiple_of(100)) =>
                {
                    29
                }
                2 => 28,
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                _ => return Err(Error::DateInvalid),
            };
            if day.is_some_and(|day| !(1..=days).contains(&day)) {
                return Err(Error::DateInvalid);
            }
        }
        Ok(Self { year, month, day })
    }

    /// Return the stated year.
    pub fn year(self) -> u16 {
        self.year
    }
    /// Return the stated month, if present.
    pub fn month(self) -> Option<u8> {
        self.month
    }
    /// Return the stated day, if present.
    pub fn day(self) -> Option<u8> {
        self.day
    }
}

impl FromStr for PartialDate {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Error> {
        let mut parts = value.split('-');
        let year = parts.next().ok_or(Error::DateInvalid)?;
        let month = parts.next();
        let day = parts.next();
        if parts.next().is_some() {
            return Err(Error::DateInvalid);
        }
        let parse = |part: &str, width| {
            if part.len() != width || !part.bytes().all(|ch| ch.is_ascii_digit()) {
                return Err(Error::DateInvalid);
            }
            part.parse::<u16>().map_err(|_| Error::DateInvalid)
        };
        let component = |part: &str| u8::try_from(parse(part, 2)?).map_err(|_| Error::DateInvalid);
        Self::new(
            parse(year, 4)?,
            month.map(component).transpose()?,
            day.map(component).transpose()?,
        )
    }
}

impl fmt::Display for PartialDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:04}", self.year)?;
        if let Some(month) = self.month {
            write!(formatter, "-{month:02}")?;
        }
        if let Some(day) = self.day {
            write!(formatter, "-{day:02}")?;
        }
        Ok(())
    }
}

// Calendar validity remains in new(); this pattern describes canonical precision.
text_value!(
    PartialDate,
    "PartialDate",
    r"^(?!0000)[0-9]{4}(-(0[1-9]|1[0-2])(-(0[1-9]|[12][0-9]|3[01]))?)?$",
    "A Gregorian date preserving year, month or day precision. The parser additionally validates the calendar day, including leap years."
);
