//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn command(outcome: &crate::command::Outcome) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(&outcome.cli_bytes)?;
    stdout.flush()
}

pub(crate) fn diagnostic(message: &'static [u8]) {
    let _ = io::stderr().lock().write_all(message);
}
