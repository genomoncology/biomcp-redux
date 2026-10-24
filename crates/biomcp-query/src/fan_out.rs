//! Scoped source work with declaration-order results and no detached workers.
#[expect(
    clippy::disallowed_methods,
    reason = "Join every source worker before returning the command result"
)]
pub(crate) fn run<'a, T: Send + 'a>(jobs: Vec<Box<dyn FnOnce() -> T + Send + 'a>>) -> Vec<T> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = jobs.into_iter().map(|job| scope.spawn(job)).collect();
        handles
            .into_iter()
            .map(|handle| match handle.join() {
                Ok(value) => value,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .collect()
    })
}
