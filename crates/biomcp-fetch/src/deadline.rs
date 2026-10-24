#![expect(
    clippy::disallowed_types,
    reason = "Advance the shared replay clock atomically without sleeping"
)]
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

/// An absolute request budget measured by the transport clock.
#[derive(Clone, Copy, Debug)]
pub struct Deadline<'a> {
    end: Instant,
    clock: Option<&'a VirtualClock>,
}

#[derive(Debug)]
pub(crate) struct VirtualClock(Mutex<Instant>);
impl Default for VirtualClock {
    fn default() -> Self {
        Self(Mutex::new(Instant::now()))
    }
}
impl VirtualClock {
    fn now(&self) -> Instant {
        *self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
    fn advance(&self, delay: Duration) -> bool {
        let mut now = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(next) = now.checked_add(delay) {
            *now = next;
            true
        } else {
            false
        }
    }
}
impl<'a> Deadline<'a> {
    pub(crate) fn start(budget: Duration, clock: Option<&'a VirtualClock>) -> Self {
        let now = clock.map_or_else(Instant::now, VirtualClock::now);
        Self {
            end: now.checked_add(budget).unwrap_or(now),
            clock,
        }
    }
    pub(crate) fn rebase(self, clock: Option<&'a VirtualClock>) -> Self {
        if self.clock.map(std::ptr::from_ref) == clock.map(std::ptr::from_ref) {
            self
        } else {
            Self::start(self.remaining(), clock)
        }
    }
    pub(crate) fn now(&self) -> Instant {
        self.clock.map_or_else(Instant::now, VirtualClock::now)
    }
    pub(crate) fn virtual_time(&self) -> bool {
        self.clock.is_some()
    }
    pub(crate) fn advance(&self, delay: Duration) -> bool {
        self.clock
            .is_some_and(|clock| clock.advance(delay.min(self.remaining())))
            && !self.expired()
    }
    /// Return the budget still available.
    pub fn remaining(&self) -> Duration {
        self.end.saturating_duration_since(self.now())
    }
    /// Whether the budget has been exhausted.
    pub fn expired(&self) -> bool {
        self.remaining().is_zero()
    }
    /// Cap this deadline with a budget starting now.
    pub fn child(&self, cap: Duration) -> Self {
        Self {
            end: self.end.min(Self::start(cap, self.clock).end),
            clock: self.clock,
        }
    }
}
