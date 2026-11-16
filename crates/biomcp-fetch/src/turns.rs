//! A process-wide count of large replies in memory, with waits bounded by each caller's time.
#![expect(
    clippy::disallowed_types,
    reason = "One count of large replies being decoded, shared by every call in the process"
)]
use std::{
    sync::{Condvar, Mutex, PoisonError},
    time::{Duration, Instant},
};

/// At most `limit` holders at once; the limit can change while the process runs.
#[derive(Debug)]
pub struct Turns {
    state: Mutex<(usize, usize)>,
    freed: Condvar,
}
/// A held turn; dropping it lets the next waiter in.
#[derive(Debug)]
pub struct Turn<'a>(&'a Turns);

impl Turns {
    /// Allow `limit` turns at once.
    pub const fn new(limit: usize) -> Self {
        Self {
            state: Mutex::new((0, limit)),
            freed: Condvar::new(),
        }
    }
    /// Change how many turns may be held at once.
    pub fn set_limit(&self, limit: usize) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.1 != limit {
            state.1 = limit;
            self.freed.notify_all();
        }
    }
    /// Wait for a turn no longer than `remaining`, the caller's own time left.
    pub fn enter(&self, remaining: Duration) -> Option<Turn<'_>> {
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
#[expect(
    clippy::disallowed_methods,
    reason = "A waiter thread proves a turn passes on when another ends"
)]
mod tests {
    use super::Turns;
    use std::time::Duration;

    #[test]
    fn a_full_count_waits_until_a_turn_ends_or_the_caller_runs_out_of_time() {
        let turns = Turns::new(2);
        let first = turns.enter(Duration::ZERO).expect("first turn");
        let _second = turns.enter(Duration::ZERO).expect("second turn");
        assert!(turns.enter(Duration::from_millis(20)).is_none());
        std::thread::scope(|scope| {
            let waiter = scope.spawn(|| turns.enter(Duration::from_secs(10)).is_some());
            std::thread::sleep(Duration::from_millis(20));
            drop(first);
            assert!(waiter.join().expect("waiter"));
        });
    }

    #[test]
    fn a_raised_limit_lets_a_waiter_in() {
        let turns = Turns::new(1);
        let _first = turns.enter(Duration::ZERO).expect("first turn");
        std::thread::scope(|scope| {
            let waiter = scope.spawn(|| turns.enter(Duration::from_secs(10)).is_some());
            std::thread::sleep(Duration::from_millis(20));
            turns.set_limit(2);
            assert!(waiter.join().expect("waiter"));
        });
    }
}
