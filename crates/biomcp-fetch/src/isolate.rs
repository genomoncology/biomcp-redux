//! A hook through which the binary reads hostile input, such as a publisher's PDF, in a child
//! process whose memory the operating system caps and which is killed at the time limit (1075).
//! The library itself starts no process; without a hook, readers run in this process.

/// Runs one bounded task on bytes outside this process.
pub trait Isolate: std::fmt::Debug + Send + Sync {
    /// The task's output for `input`, at most `max_output` bytes, or why there is none.
    /// `expired` says when to stop waiting; the child is then killed and reaped.
    fn run(
        &self,
        input: &[u8],
        max_output: usize,
        expired: &dyn Fn() -> bool,
    ) -> Result<Vec<u8>, Contained>;
}

/// Why an isolated task gave no output: it was killed at the time limit or by its memory cap,
/// it crashed, its output was too long, or it could not be started. One refusal covers all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Contained;
