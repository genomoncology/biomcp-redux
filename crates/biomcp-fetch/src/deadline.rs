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
    frozen: Option<Instant>,
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
        self.reach(other.now());
    }
    fn reach(&self, completed: Instant) {
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
            frozen: None,
            clock,
        }
    }
    pub(crate) fn matches(&self, clock: Option<&VirtualClock>) -> bool {
        self.clock.map(std::ptr::from_ref) == clock.map(std::ptr::from_ref)
    }
    pub(crate) fn bind<'b>(&self, clock: Option<&'b VirtualClock>) -> Deadline<'b> {
        Deadline {
            end: self.end,
            frozen: None,
            clock,
        }
    }
    pub(crate) fn fork(&self) -> Option<VirtualClock> {
        self.clock.map(|_| VirtualClock(Mutex::new(self.now())))
    }
    pub(crate) fn now(&self) -> Instant {
        self.frozen
            .unwrap_or_else(|| self.clock.map_or_else(Instant::now, VirtualClock::now))
    }
    /// Bind this remaining budget to wall time for cooperative local decoding.
    pub fn local(self) -> LocalDeadline {
        let now = Instant::now();
        LocalDeadline(now.checked_add(self.remaining()).unwrap_or(now))
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
            end: self
                .end
                .min(self.now().checked_add(cap).unwrap_or(self.now())),
            frozen: self.frozen,
            clock: self.clock,
        }
    }
    /// Move a command's replay origin past one completed stage, retaining its absolute end.
    /// Real time already advances itself; independent leg deadlines stay unchanged.
    pub fn after_stage(mut self, spent: Duration) -> Self {
        self.frozen = self
            .frozen
            .map(|origin| origin.checked_add(spent).unwrap_or(self.end).min(self.end));
        self
    }
}

/// A sequential command's fixed replay origin, completed once into shared time.
#[derive(Debug)]
pub struct CommandDeadline<'a>(Deadline<'a>);
impl<'a> CommandDeadline<'a> {
    pub(crate) fn start(budget: Duration, clock: Option<&'a VirtualClock>) -> Self {
        let mut deadline = Deadline::start(budget, clock);
        deadline.frozen = clock.map(VirtualClock::now);
        Self(deadline)
    }
    /// Start the command's stages from the same absolute origin.
    pub fn deadline(&self) -> Deadline<'a> {
        self.0
    }
    /// Join the completed absolute endpoint, including an early failure.
    /// Concurrent completions take the maximum endpoint; they never add durations.
    pub fn finish(self, remaining: Duration) {
        if let (Some(clock), Some(origin)) = (self.0.clock, self.0.frozen) {
            let spent = self
                .0
                .end
                .saturating_duration_since(origin)
                .saturating_sub(remaining);
            clock.reach(origin.checked_add(spent).unwrap_or(self.0.end));
        }
    }
}

/// Wall-clock bound for cooperative local parsing, including replay commands.
#[derive(Clone, Copy, Debug)]
pub struct LocalDeadline(Instant);
impl LocalDeadline {
    /// Whether the parsing budget has been exhausted.
    pub fn expired(self) -> bool {
        Instant::now() >= self.0
    }
}
