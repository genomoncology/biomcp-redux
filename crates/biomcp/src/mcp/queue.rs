//! Stdio calls over the concurrency limit wait their turn in arrival order.
use serde_json::Value;
use std::{
    collections::VecDeque,
    io,
    time::{Duration, Instant},
};

pub(super) type Work = Box<dyn FnOnce() -> io::Result<super::research::Completed> + Send>;

pub(super) struct Waiting {
    pub(super) key: String,
    pub(super) id: Value,
    pub(super) modern: bool,
    pub(super) cancellation: Option<biomcp_fetch::Cancellation>,
    pub(super) work: Work,
    arrived: Instant,
}

/// Every call waits the same time, so arrival order is also expiry order.
pub(super) struct Queue {
    waiting: VecDeque<Waiting>,
    wait: Duration,
    /// A running call ends within the command budget, which bounds the retry advice.
    pub(super) budget: Duration,
}

impl Queue {
    pub(super) fn new(settings: &crate::config::Settings) -> Self {
        let budget = crate::config::budget(settings);
        Self {
            waiting: VecDeque::new(),
            wait: crate::config::queue_wait(settings).min(budget),
            budget,
        }
    }
    pub(super) fn wait(&self) -> Duration {
        self.wait
    }
    pub(super) fn push(
        &mut self,
        key: String,
        id: Value,
        modern: bool,
        cancellation: Option<biomcp_fetch::Cancellation>,
        work: Work,
    ) {
        self.waiting.push_back(Waiting {
            key,
            id,
            modern,
            cancellation,
            work,
            arrived: Instant::now(),
        });
    }
    pub(super) fn contains(&self, key: &str) -> bool {
        self.waiting.iter().any(|call| call.key == key)
    }
    /// A cancelled call leaves the queue and, like a cancelled running call, gets no reply.
    pub(super) fn remove(&mut self, key: &str) {
        self.waiting.retain(|call| call.key != key);
    }
    pub(super) fn pop(&mut self) -> Option<Waiting> {
        self.waiting.pop_front()
    }
    pub(super) fn next_expiry(&self) -> Option<Instant> {
        self.waiting
            .front()
            .map(|call| call.arrived.checked_add(self.wait).unwrap_or(call.arrived))
    }
    pub(super) fn pop_expired(&mut self) -> Option<Waiting> {
        self.next_expiry()
            .filter(|expiry| *expiry <= Instant::now())
            .and_then(|_| self.waiting.pop_front())
    }
}
