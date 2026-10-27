//! Provider declarations and shared FIFO admission.
#![expect(
    clippy::disallowed_types,
    reason = "Transport quota state and FIFO waits are shared only within one Fetch"
)]

use crate::Deadline;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, VecDeque},
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
        let mut bytes = name.as_bytes();
        while let [byte, rest @ ..] = bytes {
            assert!(
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-',
                "invalid source key"
            );
            bytes = rest;
        }
        Self(name)
    }
}

/// A validated token rate with an explicit burst allowance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rate {
    per_second: u16,
    seconds: u16,
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

    /// Declare one request per whole-second interval, without a burst.
    ///
    /// # Panics
    /// A zero interval fails during construction, including constant evaluation.
    pub const fn every_seconds(seconds: u16) -> Self {
        assert!(seconds > 0, "zero rate interval");
        Self {
            per_second: 1,
            seconds,
            burst: 1,
        }
    }

    /// Declare a rate and burst allowance.
    ///
    /// # Panics
    /// Zero rates or bursts fail during construction, including constant evaluation.
    pub const fn new(per_second: u16, burst: u16) -> Self {
        assert!(per_second > 0 && burst > 0, "zero rate or burst");
        Self {
            per_second,
            seconds: 1,
            burst,
        }
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
    /// Nominal request interval for this credential policy, excluding bursts.
    /// A diagnostic caller may use it to pace separate command processes.
    pub fn interval(&self, keyed: bool) -> Duration {
        let rate = if keyed {
            self.keyed_rate.unwrap_or(self.rate)
        } else {
            self.rate
        };
        Duration::from_nanos(
            (1_000_000_000 * u64::from(rate.seconds)).div_ceil(u64::from(rate.per_second)),
        )
    }
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
    pub(crate) fn acquire<'a>(
        &self,
        quota: &'static Quota,
        keyed: bool,
        deadline: Deadline<'a>,
    ) -> Option<Permit<'a>> {
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
    updated: Option<Instant>,
    credit: u128,
    completed: Option<BinaryHeap<Reverse<Instant>>>,
}

// Nanosecond token units retain exact rates across credential changes.
const TOKEN: u128 = 1_000_000_000;

impl State {
    fn admission_time(&self, deadline: Deadline<'_>) -> Instant {
        let now = deadline.now();
        let slot = self
            .completed
            .as_ref()
            .and_then(|times| times.peek())
            .map(|time| time.0);
        if deadline.virtual_time()
            && let Some(last) = self
                .updated
                .into_iter()
                .chain(slot)
                .max()
                .filter(|last| *last > now)
        {
            deadline.advance(last.duration_since(now));
        }
        deadline.now()
    }

    fn available(&self, rate: Rate, now: Instant) -> u128 {
        let capacity = u128::from(rate.burst) * TOKEN;
        self.updated
            .map(|last| {
                self.credit
                    .saturating_add(
                        now.duration_since(last)
                            .as_nanos()
                            .saturating_mul(u128::from(rate.per_second))
                            / u128::from(rate.seconds),
                    )
                    .min(capacity)
            })
            .unwrap_or(capacity)
    }
}

impl Limiter {
    fn new(quota: &'static Quota) -> Self {
        Self {
            quota,
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
        }
    }

    fn acquire<'a>(self: Arc<Self>, keyed: bool, deadline: Deadline<'a>) -> Option<Permit<'a>> {
        let rate = if keyed {
            self.quota.keyed_rate.unwrap_or(self.quota.rate)
        } else {
            self.quota.rate
        };
        let ticket = Arc::new(());
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if deadline.virtual_time() && state.completed.is_none() {
            state.completed = Some(
                (0..self.quota.in_flight)
                    .map(|_| Reverse(deadline.now()))
                    .collect(),
            );
        }
        state.queue.push_back(ticket.clone());
        loop {
            if deadline.expired() {
                state.queue.retain(|v| !Arc::ptr_eq(v, &ticket));
                self.wake.notify_all();
                return None;
            }
            let now = state.admission_time(deadline);
            if deadline.expired() {
                continue;
            }
            let available = state.available(rate, now);
            // Missing credit is at most one token, so this fits in a u64.
            let nanos = TOKEN
                .saturating_sub(available)
                .saturating_mul(u128::from(rate.seconds))
                .div_ceil(u128::from(rate.per_second));
            let delay = Duration::from_nanos(nanos as u64);
            let head = state.queue.front().is_some_and(|v| Arc::ptr_eq(v, &ticket));
            if head && state.in_flight < self.quota.in_flight && delay.is_zero() {
                state.queue.pop_front();
                state.in_flight += 1;
                if let Some(completed) = &mut state.completed {
                    completed.pop();
                }
                state.credit = available - TOKEN;
                state.updated = Some(now);
                self.wake.notify_all();
                drop(state);
                return Some(Permit {
                    limiter: self,
                    deadline,
                });
            }
            let wait = if head && state.in_flight < self.quota.in_flight {
                deadline.remaining().min(delay)
            } else {
                deadline.remaining()
            };
            if deadline.virtual_time() && head && state.in_flight < self.quota.in_flight {
                deadline.advance(wait);
                continue;
            }
            if deadline.virtual_time() {
                state = self.wake.wait(state).unwrap_or_else(|e| e.into_inner());
                continue;
            }
            // Condvar releases the mutex while this caller waits for its rate slot or predecessor.
            state = self
                .wake
                .wait_timeout(state, deadline.wait_slice(wait))
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
    }
}

#[derive(Debug)]
pub(crate) struct Permit<'a> {
    limiter: Arc<Limiter>,
    deadline: Deadline<'a>,
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let mut state = self.limiter.state.lock().unwrap_or_else(|e| e.into_inner());
        state.in_flight -= 1;
        if let Some(completed) = &mut state.completed {
            completed.push(Reverse(self.deadline.now()));
        }
        self.limiter.wake.notify_all();
    }
}
