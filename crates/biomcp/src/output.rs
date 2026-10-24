//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn schema(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(text.as_bytes())?;
    stdout.flush()
}

pub(crate) fn diagnostic(message: &'static [u8]) {
    let _ = io::stderr().lock().write_all(message);
}
