//! Normalize shared FDA receipt bounds before comparison or request serialization.
use biodata::PartialDate;
pub(super) fn day(date: Option<PartialDate>, upper: bool) -> Result<Option<PartialDate>, ()> {
    date.map(|date| {
        let month = date.month().ok_or(())?;
        let day = date.day().unwrap_or_else(|| {
            if upper {
                (28..=31).rev().find(|day| {
                    PartialDate::new(date.year(), Some(month), Some(*day)).is_ok()
                }).unwrap_or(28)
            } else {
                1
            }
        });
        PartialDate::new(date.year(), Some(month), Some(day)).map_err(|_| ())
    }).transpose()
}
