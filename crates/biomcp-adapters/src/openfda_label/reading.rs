//! How many whole-label pages one process reads at once.
use std::{
    sync::{Condvar, Mutex, PoisonError},
    time::{Duration, Instant},
};

/// A 50-label page is up to 32 MiB, and reading, decoding and caching one took about five times
/// its size in a hosted server. Three pages at once keep that under half of a 1 GiB
/// task even at the page bound, and still overlap a name lookup's pages.
pub(super) const PAGES_AT_ONCE: usize = 3;

pub(super) static PAGES: Gate = Gate::new(PAGES_AT_ONCE);

/// A count of large pages in memory, waited on in arrival order of wakeups.
#[expect(
    clippy::disallowed_types,
    reason = "One process-wide count of large label pages in memory, shared by every call"
)]
pub(super) struct Gate {
    limit: usize,
    reading: Mutex<usize>,
    freed: Condvar,
}
/// A turn to read one large page; dropping it lets the next reader in.
pub(super) struct Turn<'a>(&'a Gate);

impl Gate {
    pub(super) const fn new(limit: usize) -> Self {
        Self {
            limit,
            reading: Mutex::new(0),
            freed: Condvar::new(),
        }
    }
    /// Wait for a turn no longer than `remaining`, the caller's own time left.
    pub(super) fn enter(&self, remaining: Duration) -> Option<Turn<'_>> {
        let end = Instant::now().checked_add(remaining)?;
        let mut reading = self.reading.lock().unwrap_or_else(PoisonError::into_inner);
        while *reading >= self.limit {
            let left = end.checked_duration_since(Instant::now())?;
            reading = self
                .freed
                .wait_timeout(reading, left)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        *reading += 1;
        Some(Turn(self))
    }
}
impl Drop for Turn<'_> {
    fn drop(&mut self) {
        let mut reading = self.0.reading.lock().unwrap_or_else(PoisonError::into_inner);
        *reading = reading.saturating_sub(1);
        self.0.freed.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::Gate;
    use std::time::Duration;

    #[test]
    fn a_full_gate_waits_until_a_turn_ends_or_the_caller_runs_out_of_time() {
        let gate = Gate::new(2);
        let first = gate.enter(Duration::ZERO).expect("first turn");
        let _second = gate.enter(Duration::ZERO).expect("second turn");
        assert!(gate.enter(Duration::from_millis(20)).is_none());
        std::thread::scope(|scope| {
            let waiter = scope.spawn(|| gate.enter(Duration::from_secs(10)).is_some());
            std::thread::sleep(Duration::from_millis(20));
            drop(first);
            assert!(waiter.join().expect("waiter"));
        });
    }
}
