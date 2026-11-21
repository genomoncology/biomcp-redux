//! Admit bounded reads and let the binary run them outside the calling process.

use crate::{LocalDeadline, RequestError};
use biomcp_types::{Reason, SourceNotice};
use std::{
    future::Future,
    num::NonZeroU16,
    pin::pin,
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
};
use tokio::sync::{Semaphore, SemaphorePermit};

/// Runs one bounded task on bytes outside this process.
pub trait Isolate: std::fmt::Debug + Send + Sync {
    /// Return at most `max_output` bytes, killing and reaping the worker if the deadline
    /// expires or is cancelled.
    fn run(
        &self,
        input: &[u8],
        max_output: usize,
        deadline: &LocalDeadline,
    ) -> Result<Vec<u8>, Contained>;
}

/// The isolated worker failed, was stopped by a limit, or gave too much output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Contained;

/// A worker implementation and its shared limit on in-flight reads.
#[derive(Debug)]
pub struct Isolator {
    worker: Box<dyn Isolate>,
    slots: Option<Semaphore>,
    wait: Duration,
}

impl Isolator {
    /// Create a worker pool with an optional concurrency cap.
    pub fn new(
        worker: impl Isolate + 'static,
        capacity: Option<NonZeroU16>,
        wait: Duration,
    ) -> Self {
        Self {
            worker: Box::new(worker),
            slots: capacity.map(|count| Semaphore::new(usize::from(count.get()))),
            wait,
        }
    }

    fn admit(&self, deadline: &LocalDeadline) -> Option<IsolatedRead<'_>> {
        let slot = match &self.slots {
            Some(slots) => Some(acquire(slots, self.wait, deadline)?),
            None => None,
        };
        Some(IsolatedRead {
            worker: self.worker.as_ref(),
            _slot: slot,
        })
    }
}

/// Keep one FIFO ticket while waiting. Poll in short slices to notice call cancellation.
fn acquire<'a>(
    slots: &'a Semaphore,
    wait: Duration,
    deadline: &LocalDeadline,
) -> Option<SemaphorePermit<'a>> {
    if let Ok(slot) = slots.try_acquire() {
        return Some(slot);
    }
    // A short call still needs time to download and parse after getting its slot.
    let limit = wait.min(deadline.remaining() / 2);
    let entered = Instant::now();
    let mut queued = pin!(slots.acquire());
    let mut context = Context::from_waker(Waker::noop());
    loop {
        let remaining = limit.saturating_sub(entered.elapsed());
        if remaining.is_zero() || deadline.expired() {
            return None;
        }
        if let Poll::Ready(permit) = queued.as_mut().poll(&mut context) {
            return permit.ok();
        }
        std::thread::park_timeout(remaining.min(Duration::from_millis(20)));
    }
}

/// One admitted read. Keep it alive from before fetching the input until the worker is reaped.
#[derive(Debug)]
pub struct IsolatedRead<'a> {
    worker: &'a dyn Isolate,
    _slot: Option<SemaphorePermit<'a>>,
}

impl IsolatedRead<'_> {
    /// Run the admitted read.
    pub fn run(
        &self,
        input: &[u8],
        max_output: usize,
        deadline: &LocalDeadline,
    ) -> Result<Vec<u8>, Contained> {
        self.worker.run(input, max_output, deadline)
    }
}

impl crate::Leg<'_> {
    /// Reserve space before fetching a document. Without a worker hook, the caller reads in process.
    pub fn admit_isolated_read(&self) -> Result<Option<IsolatedRead<'_>>, RequestError> {
        let Some(isolator) = &self.fetch.config.isolate else {
            return Ok(None);
        };
        self.check_source()?;
        if let Some(read) = isolator.admit(&self.local_deadline()) {
            return Ok(Some(read));
        }
        let notice = SourceNotice::RequestGuidance {
            data_date: None,
            message: "The hosted server is reading as many PDFs as it can at once. Retry shortly."
                .into(),
            retry_after_seconds: None,
        };
        let mut report = self
            .report
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !report.data_notices.contains(&notice) {
            report.data_notices.push(notice);
        }
        Err(self.failure(Reason::LocalQuota, None).into())
    }
}
