//! Resolve each source request's effective pacing identity once at admission.
use super::limiter::{Caller, Permit};
use crate::{KeyUse, Leg};
use std::time::Duration;

impl Leg<'_> {
    pub(crate) fn acquire_quota(&self) -> Result<Option<Permit<'_>>, Duration> {
        self.acquire_quota_by(self.budget())
    }
    /// Wait for a slot no longer than `deadline`, which may be shorter than the leg's budget.
    pub(crate) fn acquire_quota_by<'a>(
        &'a self,
        deadline: crate::Deadline<'a>,
    ) -> Result<Option<Permit<'a>>, Duration> {
        let caller = if self.fetch.hosted {
            Caller::Hosted(match self.spec.key {
                KeyUse::Optional(key) | KeyUse::Required(key) => {
                    self.fetch.secrets.fingerprint(key.secret)
                }
                KeyUse::None => None,
            })
        } else {
            Caller::Process
        };
        self.fetch.limiters.acquire(
            self.fetch.config.source_pacing.resolve(self.spec.quota),
            self.keyed(),
            caller,
            self.fetch.config.rate_share,
            deadline,
        )
    }
}

impl crate::Fetch {
    /// Time BioMCP's own pacing adds to `requests` sequential keyless requests to `spec`: one
    /// interval after each but the first, so a batch BioMCP accepts is not cut short by its own
    /// spacing (1003).
    pub fn paced_time(&self, spec: &crate::SourceSpec, requests: usize) -> Duration {
        let rate = self
            .config
            .source_pacing
            .resolve(spec.quota)
            .credential_rate(false);
        let tokens = u128::try_from(requests.saturating_sub(1))
            .unwrap_or(u128::MAX)
            .saturating_mul(1_000_000_000);
        self.config.rate_share.delay(tokens, rate)
    }
    /// The caller's budget plus [`Self::paced_time`]; a hosted call's end lengthens to match, as
    /// for a declared source budget. Pass the same count to [`Self::paced_leg`].
    pub fn paced_budget(
        &self,
        spec: &crate::SourceSpec,
        caller: Duration,
        requests: usize,
    ) -> Duration {
        let paced = self.paced_time(spec, requests);
        let budget = caller.saturating_add(paced);
        if !paced.is_zero()
            && let Some(end) = &self.call_end
        {
            end.lengthen(budget);
        }
        budget
    }
    /// A leg for `requests` sequential requests: the source's deadline plus [`Self::paced_time`].
    pub fn paced_leg<'a>(
        &'a self,
        spec: &'static crate::SourceSpec,
        deadline: crate::Deadline<'a>,
        requests: usize,
    ) -> Leg<'a> {
        self.leg_within(spec, deadline, self.paced_time(spec, requests))
    }
}
