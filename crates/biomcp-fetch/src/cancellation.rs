//! Independent command transports retain shared data while allowing socket shutdown.
use crate::{Fetch, FetchError, Mode, network::Agents};
#[expect(
    clippy::disallowed_types,
    reason = "Share bounded transport state across the request owners named by this module"
)]
use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
#[expect(
    clippy::disallowed_types,
    reason = "Cancel all request workers through one command-owned atomic flag"
)]
/// A command's cancellation handle cannot shut down another command's connections.
pub struct Cancellation {
    agents: Arc<Mutex<Vec<Weak<Agents>>>>,
    cancelled: Arc<AtomicBool>,
    end: Option<CallEnd>,
}
#[expect(
    clippy::disallowed_types,
    reason = "A hosted call's worker lengthens the end its waiter reads"
)]
/// When a call must end: its start plus its budget. Only a hosted call's end may lengthen, to its
/// start plus a declared source budget (sdlc/tickets/0912-hosted-calls-honour-slow-source-budgets.md).
#[derive(Clone)]
pub(crate) struct CallEnd {
    start: std::time::Instant,
    budget: Arc<AtomicU64>,
    lengthens: bool,
}
impl CallEnd {
    #[expect(
        clippy::disallowed_types,
        reason = "A hosted call's worker lengthens the end its waiter reads"
    )]
    fn new(budget: std::time::Duration, lengthens: bool) -> Self {
        Self {
            start: std::time::Instant::now(),
            budget: Arc::new(AtomicU64::new(nanos(budget))),
            lengthens,
        }
    }
    pub(crate) fn remaining(&self) -> std::time::Duration {
        let budget = std::time::Duration::from_nanos(self.budget.load(Ordering::Acquire));
        self.start
            .checked_add(budget)
            .unwrap_or(self.start)
            .saturating_duration_since(std::time::Instant::now())
    }
    /// Give a running hosted call `budget` from its start; a fixed or already ended call keeps its end.
    pub(crate) fn lengthen(&self, budget: std::time::Duration) {
        if self.lengthens && !self.remaining().is_zero() {
            self.budget.fetch_max(nanos(budget), Ordering::AcqRel);
        }
    }
}
fn nanos(budget: std::time::Duration) -> u64 {
    u64::try_from(budget.as_nanos()).unwrap_or(u64::MAX)
}
impl std::fmt::Debug for Cancellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cancellation").finish_non_exhaustive()
    }
}
impl Cancellation {
    #[expect(
        clippy::disallowed_types,
        reason = "One command registers only its live isolated connection owners"
    )]
    pub(crate) fn new(end: Option<CallEnd>) -> Self {
        Self {
            agents: Arc::new(Mutex::new(Vec::new())),
            cancelled: Arc::new(AtomicBool::new(false)),
            end,
        }
    }
    /// Start one cancellable wall-clock scope before transport construction.
    pub fn with_budget(budget: std::time::Duration) -> Self {
        Self::new(Some(CallEnd::new(budget, false)))
    }
    /// Start a hosted call's scope with the operator's budget. A command that calls a source with a
    /// declared command budget lengthens it to that budget from the call's start; cancel still ends it.
    pub fn hosted(budget: std::time::Duration) -> Self {
        Self::new(Some(CallEnd::new(budget, true)))
    }
    /// Return the wall-clock budget left, independent of any replay clock.
    pub fn remaining(&self) -> std::time::Duration {
        if self.is_cancelled() {
            return std::time::Duration::ZERO;
        }
        self.end
            .as_ref()
            .map_or(std::time::Duration::MAX, CallEnd::remaining)
    }
    pub(crate) fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
    /// Interrupt active socket work and prevent further requests in this command.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        for agents in self.agents.lock().unwrap_or_else(|e| e.into_inner()).iter() {
            if let Some(agents) = agents.upgrade() {
                agents.shutdown();
            }
        }
    }
}
impl Clone for Cancellation {
    fn clone(&self) -> Self {
        Self {
            agents: self.agents.clone(),
            cancelled: self.cancelled.clone(),
            end: self.end.clone(),
        }
    }
}
impl Fetch {
    /// Isolate cancellable connections while retaining quotas, cache, recordings and observations.
    pub fn cancellable(&self) -> Result<(Self, Cancellation), FetchError> {
        let cancellation = Cancellation::new(None);
        let fetch = self.with_cancellation(&cancellation)?;
        Ok((fetch, cancellation))
    }
    /// Bind this transport's isolated connections to an existing command scope.
    pub fn with_cancellation(&self, cancellation: &Cancellation) -> Result<Self, FetchError> {
        let agents = Arc::new(Agents::new(
            &self.config.network,
            self.config.proxy.as_ref(),
            self.tls.clone(),
        )?);
        let mut owners = cancellation
            .agents
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        owners.retain(|owner| owner.strong_count() > 0);
        owners.push(Arc::downgrade(&agents));
        if cancellation.cancelled.load(Ordering::Acquire) {
            agents.shutdown();
        }
        let mut fetch = self.clone();
        fetch.agents = agents;
        fetch.cancelled = Some(cancellation.cancelled.clone());
        fetch.call_end = cancellation.end.clone();
        Ok(fetch)
    }
    /// Restrict this command to stored replies while retaining its shared observations sink.
    pub fn offline(&self) -> Self {
        let mut fetch = self.clone();
        fetch.mode = Mode::Offline;
        fetch
    }
}
