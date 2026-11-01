//! Bound framing readers separately from active requests so health stays reachable.
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
pub(super) struct Slots(Arc<AtomicUsize>);
pub(super) struct Permit(Arc<AtomicUsize>);
impl Slots {
    pub(super) fn take(&self) -> Option<Permit> {
        self.0
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                (count < super::MAX_CONNECTIONS).then_some(count + 1)
            })
            .ok()
            .map(|_| Permit(self.0.clone()))
    }
}
impl Drop for Permit {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Release);
    }
}
