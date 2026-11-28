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
    Failed(FirstFailure),
}

struct FirstFailure {
    message: String,
    until: i64,
    /// The local size or time limit that stopped the fetch. Nothing is syncing then (1247).
    limit: Option<Limit>,
    publisher_status: Option<u16>,
    publisher_until: Option<i64>,
}

/// A first-read limit, stored as its code in `bundle_first_read.first_limit`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Limit {
    Size = 1,
    Time = 2,
}

impl Limit {
    fn from_code(code: Option<i64>) -> Option<Self> {
        match code {
            Some(1) => Some(Self::Size),
            Some(2) => Some(Self::Time),
            _ => None,
        }
    }
    fn reason(self) -> Reason {
        match self {
            Self::Size => Reason::TooLarge,
            Self::Time => Reason::DeadlineExceeded,
        }
    }
}

impl FirstFailure {
    fn report(self, leg: &Leg<'_>, fallback: Reason) -> crate::SourceError {
        let until = self
            .publisher_until
            .map_or(self.until, |publisher| publisher.max(self.until));
        let remaining = until.saturating_sub(now()).max(0) as u64;
        let reason = match (self.publisher_status, self.limit) {
            (Some(status), _) => crate::retry::status_reason(status),
            (None, Some(limit)) => limit.reason(),
            (None, None) => fallback,
        };
        let mut error = leg.failure(reason, self.publisher_status);
        // The rate-limit guidance already carries the timing. Keep the first-fetch context alongside it.
        if self.publisher_status.is_some() {
            error.retry_after = Some(Duration::from_secs(remaining));
        }
        // A retry of an oversized first read fails the same way, so it names no delay.
        leg.first_notice(
            self.message,
            (!matches!(reason, Reason::RateLimited | Reason::TooLarge)).then_some(remaining),
        );
        error
    }
}

fn publisher_refusal(error: RequestError) -> Option<(u16, Option<Duration>)> {
    let RequestError::Source(error) = error else {
        return None;
    };
    error
        .status
        .filter(|status| (400..=599).contains(status))
        .map(|status| (status, error.retry_after))
}

impl Cache {
    fn admit_first(&self, name: &str, budget: Deadline<'_>) -> Result<Admission, CacheError> {
        self.write_sql(budget, |connection| {
            let tx = write_transaction(connection)?;
            if tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM bundle WHERE name=?1)",
                    [name],
                    |r| r.get(0),
                )
                .map_err(db_error)?
            {
                return Ok(Admission::Published);
            }
            let current = now();
            let previous = tx
                .query_row(
                    "SELECT until,failure,publisher_status,publisher_until,first_limit FROM bundle_first_read
                 WHERE name=?1 AND (until>?2 OR publisher_until>?2)",
                    (name, current),
                    |r| {
                        let message = r.get::<_, Option<String>>(1)?;
                        Ok(match message {
                            Some(message) => Admission::Failed(FirstFailure {
                                message,
                                until: r.get(0)?,
                                publisher_status: r.get(2)?,
                                publisher_until: r.get(3)?,
                                limit: Limit::from_code(r.get(4)?),
                            }),
                            None => Admission::Waiting,
                        })
                    },
                )
                .optional()
                .map_err(db_error)?;
            if let Some(admission) = previous {
                return Ok(admission);
            }
            // The claim outlives the read budget and its bounded cleanup. A dead process releases it by expiry.
            let until = current
                .saturating_add(i64::try_from(budget.remaining().as_secs()).unwrap_or(i64::MAX))
                .saturating_add(3);
            tx.execute(
                "INSERT INTO bundle_first_read(name,until) VALUES(?1,?2) ON CONFLICT(name)
                 DO UPDATE SET until=?2,failure=NULL,publisher_status=NULL,publisher_until=NULL,first_limit=NULL",
                (name, until),
            )
            .map_err(db_error)?;
            tx.commit().map_err(db_error)?;
            Ok(Admission::Fetch(until))
        })
    }
}

impl Leg<'_> {
    pub(super) fn first_fetch<R: Row>(
        &self,
        bundle: &Bundle<R>,
        failure_reason: Reason,
        fetch: impl FnOnce(&Size) -> Result<(), RequestError>,
    ) -> Result<(), RequestError> {
        let cache = self.bundle_cache()?;
        let mut waited = false;
        let claim = loop {
            match cache.admit_first(bundle.name, self.budget())? {
                Admission::Published => return Ok(()),
                Admission::Fetch(claim) => break claim,
                Admission::Failed(failure) => {
                    return Err(failure.report(self, failure_reason).into());
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
        let failure = self.first_failure(error, size);
        // Persist the failure even when the download used all of its time.
        cache.write_sql(Deadline::start(Duration::from_secs(1), None), |c| {
            c.execute(
                "UPDATE bundle_first_read SET until=?3,failure=?4,publisher_status=?5,publisher_until=?6,first_limit=?7
                 WHERE name=?1 AND until=?2 AND failure IS NULL",
                params![
                    bundle.name,
                    claim,
                    failure.until,
                    failure.message,
                    failure.publisher_status,
                    failure.publisher_until,
                    failure.limit.map(|limit| limit as i64),
                ],
            ).map(drop).map_err(db_error)
        })?;
        let reported = failure.report(self, failure_reason);
        match error {
            RequestError::Source(_) => Err(reported.into()),
            local => Err(local),
        }
    }

    fn first_failure(&self, error: RequestError, size: Size) -> FirstFailure {
        let current = now();
        let publisher = publisher_refusal(error);
        let bytes = size
            .0
            .get()
            .map_or_else(|| "unknown".to_owned(), |n| format!("{n} bytes"));
        let mut limit = None;
        let message = if let Some((status, _)) = publisher {
            let mut message =
                format!("Publisher refused first fetch (HTTP {status}). Size: {bytes}.");
            if crate::retry::status_reason(status) != Reason::RateLimited {
                message.push_str(" Retry the read after the reported delay.");
            }
            message
        } else {
            limit = match error {
                RequestError::Source(source) if source.reason == Reason::TooLarge => {
                    Some(Limit::Size)
                }
                RequestError::Source(source) if source.reason == Reason::DeadlineExceeded => {
                    Some(Limit::Time)
                }
                _ if self.budget().expired() => Some(Limit::Time),
                _ => None,
            };
            let reason = match limit {
                Some(Limit::Size) => "exceeded the size limit".to_owned(),
                Some(Limit::Time) => "ran out of time".to_owned(),
                None => format!("failed ({error})"),
            };
            format!("The first-read fetch {reason}. Source size: {bytes}.")
        };
        FirstFailure {
            until: current.saturating_add(self.fetch.config.auto_sync_retry_seconds.into()),
            message,
            limit,
            publisher_status: publisher.map(|(status, _)| status),
            publisher_until: publisher.and_then(|(_, delay)| delay).map(|delay| {
                let seconds = delay
                    .as_secs()
                    .saturating_add(u64::from(delay.subsec_nanos() != 0));
                current.saturating_add(i64::try_from(seconds).unwrap_or(i64::MAX))
            }),
        }
    }

    fn first_notice(&self, message: String, retry_after_seconds: Option<u64>) {
        let notice = SourceNotice::RequestGuidance {
            message: message.into(),
            retry_after_seconds,
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
