use std::time::{Duration, Instant};

/// An absolute request budget measured by the monotonic clock.
#[derive(Clone, Copy, Debug)]
pub struct Deadline(Instant);

impl Deadline {
    pub(crate) fn after(budget: Duration) -> Self {
        let now = Instant::now();
        Self(now.checked_add(budget).unwrap_or(now))
    }

    /// Return the budget still available.
    pub fn remaining(&self) -> Duration {
        self.0.saturating_duration_since(Instant::now())
    }

    /// Whether the budget has been exhausted.
    pub fn expired(&self) -> bool {
        self.remaining().is_zero()
    }

    /// Cap this deadline with a budget starting now.
    pub fn child(&self, cap: Duration) -> Self {
        Self(self.0.min(Self::after(cap).0))
    }
}
