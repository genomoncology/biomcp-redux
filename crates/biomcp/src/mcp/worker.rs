//! Tool work always returns a completion, including an unexpected panic.
use std::io;
pub(crate) fn execute<T>(work: impl FnOnce() -> io::Result<T>) -> io::Result<T> {
    work()
}
