//! Wake cancellable stream work without detaching its network tasks.
use crate::LocalDeadline;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Default)]
pub(super) struct Stop {
    flag: AtomicBool,
    wake: tokio::sync::Notify,
}
impl Stop {
    pub(super) fn cancel(&self) {
        self.flag.store(true, Ordering::Release);
        self.wake.notify_waiters();
    }
    pub(super) fn stopped(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }
    pub(super) async fn wait(&self) {
        loop {
            let wake = self.wake.notified();
            if self.stopped() {
                return;
            }
            wake.await;
        }
    }
}
pub(super) async fn deadline(deadline: &LocalDeadline) {
    while !deadline.expired() {
        tokio::time::sleep(
            deadline
                .end
                .saturating_duration_since(std::time::Instant::now())
                .min(std::time::Duration::from_millis(20)),
        )
        .await;
    }
}
pub(super) struct CallGuard {
    pub(super) stop: Arc<Stop>,
    pub(super) done: std::sync::mpsc::Receiver<()>,
}
impl Drop for CallGuard {
    fn drop(&mut self) {
        self.stop.cancel();
        // The worker acknowledges only after its stream future and connection driver drop.
        // A running system resolver is the sole documented exception.
        if self.done.recv().is_err() {
            self.stop.cancel();
        }
    }
}
