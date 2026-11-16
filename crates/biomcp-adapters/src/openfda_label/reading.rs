//! How many whole-label pages one process decodes at once.
use std::{
    num::NonZeroUsize,
    sync::{Condvar, Mutex, PoisonError},
    time::{Duration, Instant},
};

/// Default for [`set_pages_at_once`]. Decoding, checking and caching a large label page took
/// about five times its size in a hosted server: a page near the 32 MiB page bound cost about
/// 140 MiB. Three such pages at once stay under half of a 1 GiB task.
pub const PAGES_AT_ONCE: usize = 3;
/// A body at least this large waits for a turn; smaller pages and single labels never wait.
pub(super) const LARGE_PAGE: usize = 1024 * 1024;

pub(super) static PAGES: Gate = Gate::new(PAGES_AT_ONCE);

/// Set how many large label pages this process decodes at once.
pub fn set_pages_at_once(limit: NonZeroUsize) {
    PAGES.set_limit(limit.get());
}

/// A count of large pages in memory and the most allowed at once.
#[expect(
    clippy::disallowed_types,
    reason = "One process-wide count of large label pages in memory, shared by every call"
)]
pub(super) struct Gate {
    state: Mutex<(usize, usize)>,
    freed: Condvar,
}
/// A turn to decode one large page; dropping it lets the next reader in.
pub(super) struct Turn<'a>(&'a Gate);

impl Gate {
    pub(super) const fn new(limit: usize) -> Self {
        Self {
            state: Mutex::new((0, limit)),
            freed: Condvar::new(),
        }
    }
    fn set_limit(&self, limit: usize) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.1 != limit {
            state.1 = limit;
            self.freed.notify_all();
        }
    }
    /// Wait for a turn no longer than `remaining`, the caller's own time left.
    pub(super) fn enter(&self, remaining: Duration) -> Option<Turn<'_>> {
        let end = Instant::now().checked_add(remaining)?;
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        while state.0 >= state.1 {
            let left = end.checked_duration_since(Instant::now())?;
            state = self
                .freed
                .wait_timeout(state, left)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        state.0 += 1;
        Some(Turn(self))
    }
}
impl Drop for Turn<'_> {
    fn drop(&mut self) {
        let mut state = self.0.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.0 = state.0.saturating_sub(1);
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

    #[test]
    fn a_raised_limit_lets_a_waiting_reader_in() {
        let gate = Gate::new(1);
        let _first = gate.enter(Duration::ZERO).expect("first turn");
        std::thread::scope(|scope| {
            let waiter = scope.spawn(|| gate.enter(Duration::from_secs(10)).is_some());
            std::thread::sleep(Duration::from_millis(20));
            gate.set_limit(2);
            assert!(waiter.join().expect("waiter"));
        });
    }
}
