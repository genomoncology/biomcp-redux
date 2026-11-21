//! Disclose publisher dataset dates without treating acquisition as publication.
use crate::{Fetch, Leg, RecordError, record::Recordings};
use biomcp_types::{SourceNotice, SourceOutcome};
use std::{fs, io::Read, path::Path};

impl Leg<'_> {
    /// Return the current UTC year and month, using the recorded date if replay pins it.
    pub fn utc_month(&self) -> (i64, i64) {
        let today = match self.fetch.recordings.as_ref() {
            Recordings::Replay {
                today: Some(today), ..
            } => *today,
            _ => crate::cache::now().div_euclid(86_400),
        };
        let (year, month, _) = civil_date(today);
        (year, month)
    }

    /// Retain a native dataset date or timestamp after the adapter identifies its role.
    /// Invalid dates and dates without day precision are disclosed without an age claim.
    pub fn data_date(&self, date: &str) {
        let notice = self.fetch.data_date_notice(date);
        let mut report = self
            .report
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !report.data_notices.contains(&notice) {
            report.data_notices.push(notice);
        }
    }
    /// Report a validated local copy past its refresh date.
    /// A hosted server serves a pinned snapshot its callers cannot refresh, so the
    /// reply stays complete and states the copy's date. Elsewhere it stays partial.
    pub(crate) fn past_refresh(&self, retrieved_at: Option<i64>) {
        let mut report = self
            .report
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !self.fetch.hosted {
            report.stale = true;
            return;
        }
        let retrieved = retrieved_at
            .and_then(utc_date)
            .unwrap_or_else(|| "on an unknown date".into());
        let notice = SourceNotice::RequestGuidance {
            data_date: None,
            message: format!(
                "Local copy retrieved {retrieved}, past its refresh date. This hosted server answers from that copy."
            )
            .into(),
            retry_after_seconds: None,
        };
        if !report.data_notices.contains(&notice) {
            report.data_notices.push(notice);
        }
    }
}

impl Fetch {
    /// Attach a retained publisher dataset date without repeating the same fact.
    pub fn retain_data_date(&self, source: &mut SourceOutcome, date: &str) {
        if !source.notices.iter().any(|notice| matches!(notice, SourceNotice::RequestGuidance { data_date: Some(known), .. } if known.as_ref() == date)) {
            source.notices.push(self.data_date_notice(date));
        }
    }
    /// Reassess retained publisher dates when returning stored continuations.
    pub fn refresh_data_dates(&self, sources: &mut [SourceOutcome]) {
        for source in sources {
            for notice in &mut source.notices {
                if let SourceNotice::RequestGuidance {
                    data_date: Some(date),
                    ..
                } = notice
                {
                    *notice = self.data_date_notice(date);
                }
            }
        }
    }

    /// Describe one native publisher dataset date using the current configured age limit.
    pub fn data_date_notice(&self, date: &str) -> SourceNotice {
        let max_days = self.config.source_data_max_age_days;
        let old = day(date).is_some_and(|date| {
            let today = crate::cache::now().div_euclid(86_400);
            today.saturating_sub(date) > i64::from(max_days)
        });
        let message = if old {
            format!(
                "Publisher dataset date: {date}. Old data: exceeds the configured {max_days}-day age limit. Retrieval does not establish a newer data date."
            )
        } else {
            format!(
                "Publisher dataset date: {date}. This is distinct from retrieval and record event dates."
            )
        };
        SourceNotice::RequestGuidance {
            data_date: Some(date.into()),
            message: message.into(),
            retry_after_seconds: None,
        }
    }
}

/// Read the optional YYYY-MM-DD calendar date pinned by a replay fixture.
pub(crate) fn replay_day(dir: &Path) -> Result<Option<i64>, RecordError> {
    const MAX_BYTES: u64 = 12;
    let path = dir.join("today.txt");
    let meta = match path.symlink_metadata() {
        Ok(meta) => meta,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if !meta.is_file() {
        return Err(RecordError::Malformed);
    }
    let mut text = String::new();
    fs::File::open(path)?
        .take(MAX_BYTES + 1)
        .read_to_string(&mut text)?;
    let date = text.trim_end_matches(['\r', '\n']);
    if text.len() as u64 > MAX_BYTES || date.len() != 10 {
        return Err(RecordError::Malformed);
    }
    day(date).map(Some).ok_or(RecordError::Malformed)
}

fn day(value: &str) -> Option<i64> {
    let date = value.get(..10)?;
    if value.len() != 10 {
        value.parse::<biomcp_types::SourceTimestampValue>().ok()?;
    }
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
    Some(
        era * 146_097 + year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year
            - 719_468,
    )
}

fn utc_date(seconds: i64) -> Option<String> {
    let (year, month, day) = civil_date(seconds.div_euclid(86_400));
    (0..=9999)
        .contains(&year)
        .then(|| format!("{year:04}-{month:02}-{day:02}"))
}

// Unix days to a Gregorian civil date, the inverse of `day`.
fn civil_date(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month + 2) / 5 + 1;
    let month = if month < 10 { month + 3 } else { month - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
#[path = "../tests/record/data_age.rs"]
mod tests;
