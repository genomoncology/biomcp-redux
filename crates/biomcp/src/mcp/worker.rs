//! Tool work always returns a completion, including an unexpected panic.
use std::io;
pub(crate) fn execute<T>(work: impl FnOnce() -> io::Result<T>) -> io::Result<T> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(work))
        .unwrap_or_else(|_| Err(io::Error::other("MCP worker panicked")))
}

#[cfg(test)]
#[path = "worker_tests.rs"]
mod tests;
