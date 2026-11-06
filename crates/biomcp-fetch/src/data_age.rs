//! Disclose publisher dataset dates without treating acquisition as publication.
use crate::Leg;
use biomcp_types::SourceNotice;

impl Leg<'_> {
    /// Retain a native dataset date or timestamp after the adapter identifies its role.
    /// Invalid dates and dates without day precision are disclosed without an age claim.
    pub fn data_date(&self, date: &str) {
        let max_days = self.fetch.config.source_data_max_age_days;
        let old = day(date).is_some_and(|date| {
            let today = crate::cache::now().div_euclid(86_400);
            today.saturating_sub(date) > i64::from(max_days)
        });
        let message = if old {
            format!("Publisher dataset date: {date}. Old data: exceeds the configured {max_days}-day age limit. Retrieval does not establish a newer data date.")
        } else {
            format!("Publisher dataset date: {date}. This is distinct from retrieval and record event dates.")
        };
        let notice = SourceNotice::RequestGuidance {
            message: message.into(),
            retry_after_seconds: None,
        };
        let mut report = self
            .report
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !report.data_notices.contains(&notice) {
            report.data_notices.push(notice);
        }
    }
}

fn day(value: &str) -> Option<i64> {
    let date = value.get(..10)?;
    // Reuse the public calendar validator, including leap-year checks.
    format!("{date}T00:00:00Z")
        .parse::<biomcp_types::SourceTimestampValue>()
        .ok()?;
    let mut parts = date.split('-');
    let mut year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    // Gregorian civil date to Unix days, with March starting each leap-year cycle.
    year -= i64::from(month <= 2);
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month + 2) / 5 + day - 1;
    Some(era * 146_097 + year_of_era * 365 + year_of_era / 4 - year_of_era / 100
        + day_of_year - 719_468)
}

#[cfg(test)]
mod tests {
    #[test]
    fn calendar_days_preserve_leap_years_and_reject_partial_dates() {
        assert_eq!(super::day("1970-01-01"), Some(0));
        assert_eq!(super::day("2000-03-01").unwrap() - super::day("2000-02-28").unwrap(), 2);
        assert_eq!(super::day("1900-03-01").unwrap() - super::day("1900-02-28").unwrap(), 1);
        assert_eq!(super::day("2026-02-29"), None);
        assert_eq!(super::day("2026-01"), None);
    }
}
