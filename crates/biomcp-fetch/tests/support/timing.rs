//! Timing observations compiled only by quota and retry contracts.
use std::time::Instant;

pub(super) struct Timing {
    pub(super) started: Instant,
}
impl Default for Timing {
    fn default() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

pub(super) type Server = super::support::Server<Timing>;
