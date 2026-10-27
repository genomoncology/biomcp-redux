//! Independent command transports retain shared data while allowing socket shutdown.
use crate::{Fetch, FetchError, Mode, network::Agents};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// A command's cancellation handle cannot shut down another command's connections.
pub struct Cancellation {
    agents: Arc<Agents>,
    cancelled: Arc<AtomicBool>,
}
impl std::fmt::Debug for Cancellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cancellation").finish_non_exhaustive()
    }
}
impl Cancellation {
    /// Interrupt active socket work and prevent further requests in this command.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        self.agents.shutdown();
    }
}
impl Fetch {
    /// Isolate cancellable connections while retaining quotas, cache, recordings and observations.
    pub fn cancellable(&self) -> Result<(Self, Cancellation), FetchError> {
        let agents = Arc::new(Agents::new(
            &self.config.network,
            self.config.proxy.as_ref(),
        )?);
        let cancelled = Arc::new(AtomicBool::new(false));
        let mut fetch = self.clone();
        fetch.agents = agents.clone();
        fetch.cancelled = Some(cancelled.clone());
        Ok((fetch, Cancellation { agents, cancelled }))
    }
    /// Restrict this command to stored replies while retaining its shared observations sink.
    pub fn offline(&self) -> Self {
        let mut fetch = self.clone();
        fetch.mode = Mode::Offline;
        fetch
    }
}
