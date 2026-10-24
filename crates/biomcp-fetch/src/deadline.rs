#![expect(
    clippy::disallowed_types,
    reason = "Advance each replay leg clock atomically across its requests"
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
    pub(crate) fn join(&self, other: &Self) {
        let completed = other.now();
        let mut now = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *now = (*now).max(completed);
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
    pub(crate) fn matches(&self, clock: Option<&VirtualClock>) -> bool {
        self.clock.map(std::ptr::from_ref) == clock.map(std::ptr::from_ref)
    }
    pub(crate) fn bind<'b>(&self, clock: Option<&'b VirtualClock>) -> Deadline<'b> {
        Deadline {
            end: self.end,
            clock,
        }
    }
    pub(crate) fn fork(&self) -> Option<VirtualClock> {
        self.clock.map(|_| VirtualClock(Mutex::new(self.now())))
    }
    pub(crate) fn now(&self) -> Instant {
        self.clock.map_or_else(Instant::now, VirtualClock::now)
    }
    /// Report whether this budget advances in replay time.
    pub fn virtual_time(&self) -> bool {
        self.clock.is_some()
    }
    pub(crate) fn advance(&self, delay: Duration) -> bool {
        self.clock
            .is_some_and(|clock| clock.advance(delay.min(self.remaining())))
            && !self.expired()
    }
    /// Return the budget still available on this clock.
    /// Replay progress belongs to the source leg; inspect it with `Leg::remaining`.
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
