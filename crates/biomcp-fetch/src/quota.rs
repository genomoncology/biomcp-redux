//! Provider declarations and shared FIFO admission.
#![expect(
    clippy::disallowed_types,
    reason = "Transport quota state and FIFO waits are shared only within one Fetch"
)]

use crate::Deadline;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

/// A provider's declared limit identity, independent of URLs and source names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceKey(&'static str);

impl SourceKey {
    /// Declare a nonempty key containing lowercase ASCII letters, digits and hyphens.
    ///
    /// # Panics
    /// Invalid declarations fail here, including during constant evaluation.
    pub const fn declare(name: &'static str) -> Self {
        assert!(!name.is_empty(), "empty source key");
        let bytes = name.as_bytes();
        let mut i = 0;
        while let Some(byte) = bytes.get(i) {
            assert!(
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-',
                "invalid source key"
            );
            i += 1;
        }
        Self(name)
    }
}

/// A validated token rate with an explicit burst allowance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rate {
    per_second: u16,
    burst: u16,
}

impl Rate {
    /// Declare a rate with no burst beyond the next single request.
    ///
    /// # Panics
    /// A zero rate fails during construction, including constant evaluation.
    pub const fn per_second(per_second: u16) -> Self {
        Self::new(per_second, 1)
    }

    /// Declare a rate and burst allowance.
    ///
    /// # Panics
    /// Zero rates or bursts fail during construction, including constant evaluation.
    pub const fn new(per_second: u16, burst: u16) -> Self {
        assert!(per_second > 0 && burst > 0, "zero rate or burst");
        Self { per_second, burst }
    }

    fn interval(self) -> Duration {
        Duration::from_nanos(1_000_000_000u64.div_ceil(u64::from(self.per_second)))
    }
}

/// One provider limit shared by every source declaring its key.
#[derive(Debug)]
pub struct Quota {
    key: SourceKey,
    rate: Rate,
    keyed_rate: Option<Rate>,
    in_flight: u8,
}

impl Quota {
    /// Declare validated provider limits.
    ///
    /// # Panics
    /// A zero in-flight limit fails during construction, including constant evaluation.
    pub const fn new(key: SourceKey, rate: Rate, keyed_rate: Option<Rate>, in_flight: u8) -> Self {
        assert!(in_flight > 0, "zero in-flight limit");
        Self {
            key,
            rate,
            keyed_rate,
            in_flight,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Limiters(Mutex<BTreeMap<SourceKey, Arc<Limiter>>>);

impl Limiters {
    pub(crate) fn acquire(
        &self,
        quota: &'static Quota,
        keyed: bool,
        deadline: Deadline,
    ) -> Option<Permit> {
        let limiter = self
            .0
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entry(quota.key)
            .or_insert_with(|| Arc::new(Limiter::new(quota)))
            .clone();
        limiter.acquire(keyed, deadline)
    }
}

#[derive(Debug)]
struct Limiter {
    quota: &'static Quota,
    state: Mutex<State>,
    wake: Condvar,
}

#[derive(Debug, Default)]
struct State {
    queue: VecDeque<Arc<()>>,
    in_flight: u8,
    next: Option<Instant>,
}

impl Limiter {
    fn new(quota: &'static Quota) -> Self {
        Self {
            quota,
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
        }
    }

    fn acquire(self: Arc<Self>, keyed: bool, deadline: Deadline) -> Option<Permit> {
        let rate = if keyed {
            self.quota.keyed_rate.unwrap_or(self.quota.rate)
        } else {
            self.quota.rate
        };
        let interval = rate.interval();
        let burst = interval * u32::from(rate.burst - 1);
        let ticket = Arc::new(());
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.queue.push_back(ticket.clone());
        loop {
            if deadline.expired() {
                state.queue.retain(|v| !Arc::ptr_eq(v, &ticket));
                self.wake.notify_all();
                return None;
            }
            let now = Instant::now();
            let delay = state
                .next
                .map(|next| next.saturating_duration_since(now).saturating_sub(burst))
                .unwrap_or_default();
            let head = state.queue.front().is_some_and(|v| Arc::ptr_eq(v, &ticket));
            if head && state.in_flight < self.quota.in_flight && delay.is_zero() {
                state.queue.pop_front();
                state.in_flight += 1;
                state.next = Some(state.next.unwrap_or(now).max(now) + interval);
                self.wake.notify_all();
                drop(state);
                return Some(Permit(self));
            }
            let wait = if head && state.in_flight < self.quota.in_flight {
                deadline.remaining().min(delay)
            } else {
                deadline.remaining()
            };
            // Condvar releases the mutex while this caller waits for its rate slot or predecessor.
            state = self
                .wake
                .wait_timeout(state, wait)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
    }
}

#[derive(Debug)]
pub(crate) struct Permit(Arc<Limiter>);

impl Drop for Permit {
    fn drop(&mut self) {
        let mut state = self.0.state.lock().unwrap_or_else(|e| e.into_inner());
        state.in_flight -= 1;
        self.0.wake.notify_all();
    }
}
