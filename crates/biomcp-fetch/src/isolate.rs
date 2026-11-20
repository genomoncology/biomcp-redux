//! Admit bounded reads and let the binary run them outside the calling process.

use crate::{LocalDeadline, RequestError};
use biomcp_types::{Reason, SourceNotice};
use std::num::NonZeroU16;
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
}

impl Isolator {
    /// Create a worker pool with an optional concurrency cap.
    pub fn new(worker: impl Isolate + 'static, capacity: Option<NonZeroU16>) -> Self {
        Self {
            worker: Box::new(worker),
            slots: capacity.map(|count| Semaphore::new(usize::from(count.get()))),
        }
    }

    fn admit(&self) -> Option<IsolatedRead<'_>> {
        let slot = match &self.slots {
            Some(slots) => Some(slots.try_acquire().ok()?),
            None => None,
        };
        Some(IsolatedRead {
            worker: self.worker.as_ref(),
            _slot: slot,
        })
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
        if let Some(read) = isolator.admit() {
            return Ok(Some(read));
        }
        self.check_source()?;
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
