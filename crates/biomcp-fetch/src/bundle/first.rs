//! Coordinate missing-bundle reads across commands sharing the store.
use super::{Bundle, Row};
use crate::{
    CacheError, Deadline, Leg, RequestError,
    cache::{Cache, db_error, now, write_transaction},
};
use biomcp_types::{Reason, SourceNotice};
use rusqlite::{OptionalExtension, params};
use std::time::Duration;

/// A response can disclose its length before reading its body fails.
#[derive(Default)]
#[expect(
    clippy::disallowed_types,
    reason = "The synchronous response reader reports length to its own first fetch"
)]
pub(super) struct Size(std::cell::Cell<Option<u64>>);

impl Size {
    pub(super) fn set(&self, bytes: Option<u64>) {
        self.0.set(bytes);
    }
}

enum Admission {
    Published,
    Fetch(i64),
    Waiting,
    Failed { message: String, until: i64 },
}

impl Cache {
    fn admit_first(&self, name: &str, budget: Deadline<'_>) -> Result<Admission, CacheError> {
        self.write_sql(budget, |connection| {
            let tx = write_transaction(connection)?;
            if tx.query_row("SELECT EXISTS(SELECT 1 FROM bundle WHERE name=?1)", [name], |r| r.get(0)).map_err(db_error)? {
                return Ok(Admission::Published);
            }
            let current = now();
            let previous = tx.query_row(
                "SELECT until,failure FROM bundle_first_read WHERE name=?1 AND until>?2",
                (name, current),
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, Option<String>>(1)?)),
            ).optional().map_err(db_error)?;
            if let Some((until, message)) = previous {
                return Ok(match message {
                    Some(message) => Admission::Failed { message, until },
                    None => Admission::Waiting,
                });
            }
            // The claim outlives the read budget and its bounded cleanup. A dead process releases it by expiry.
            let until = current.saturating_add(i64::try_from(budget.remaining().as_secs()).unwrap_or(i64::MAX)).saturating_add(3);
            tx.execute("INSERT INTO bundle_first_read VALUES(?1,?2,NULL) ON CONFLICT(name) DO UPDATE SET until=?2,failure=NULL",
                (name, until)).map_err(db_error)?;
            tx.commit().map_err(db_error)?;
            Ok(Admission::Fetch(until))
        })
    }
}

impl Leg<'_> {
    pub(super) fn first_fetch<R: Row>(
        &self,
        bundle: &Bundle<R>,
        fetch: impl FnOnce(&Size) -> Result<(), RequestError>,
    ) -> Result<(), RequestError> {
        let cache = self.bundle_cache()?;
        let mut waited = false;
        let claim = loop {
            match cache.admit_first(bundle.name, self.budget())? {
                Admission::Published => return Ok(()),
                Admission::Fetch(claim) => break claim,
                Admission::Failed { message, until } => {
                    self.first_notice(message, until);
                    return Err(self.failure(Reason::Syncing, None).into());
                }
                Admission::Waiting => {
                    self.wait_for_bundle(bundle, waited)?;
                    waited = true;
                }
            }
        };
        let size = Size::default();
        // Publishing removes the claim in the same transaction as the new bundle.
        let Err(error) = fetch(&size) else {
            return Ok(());
        };
        let reason = match error {
            RequestError::Source(source) if source.reason == Reason::TooLarge => {
                "exceeded the size limit".to_owned()
            }
            RequestError::Source(source) if source.reason == Reason::DeadlineExceeded => {
                "ran out of time".to_owned()
            }
            _ if self.budget().expired() => "ran out of time".to_owned(),
            _ => format!("failed ({error})"),
        };
        let bytes = size
            .0
            .get()
            .map_or_else(|| "unknown".to_owned(), |n| format!("{n} bytes"));
        let mut message = format!("The first-read fetch {reason}. Source size: {bytes}.");
        // Local quota errors also carry a delay, but have no publisher response status.
        if let RequestError::Source(source) = error
            && source.status.is_some()
            && let Some(delay) = source.retry_after
        {
            let seconds = delay
                .as_secs()
                .saturating_add(u64::from(delay.subsec_nanos() != 0));
            message.push_str(&format!(" Publisher Retry-After was {seconds} seconds."));
        }
        let until = now().saturating_add(self.fetch.config.auto_sync_retry_seconds.into());
        self.first_notice(message.clone(), until);
        // Persist the failure even when the download used all of its time.
        cache.write_sql(Deadline::start(Duration::from_secs(1), None), |c| {
            c.execute("UPDATE bundle_first_read SET until=?3,failure=?4 WHERE name=?1 AND until=?2 AND failure IS NULL",
                params![bundle.name, claim, until, message]).map(drop).map_err(db_error)
        })?;
        match error {
            RequestError::Source(_) => Err(self.failure(Reason::Syncing, None).into()),
            local => Err(local),
        }
    }

    fn first_notice(&self, message: String, until: i64) {
        let notice = SourceNotice::RequestGuidance {
            message: message.into(),
            retry_after_seconds: Some(until.saturating_sub(now()).max(0) as u64),
            data_date: None,
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
