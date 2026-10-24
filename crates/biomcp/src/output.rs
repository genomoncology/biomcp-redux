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

pub(crate) fn usage() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    serde_json::to_writer(&mut stdout, &biodata::ErrorEnvelope::schema_usage())?;
    stdout.write_all(b"\n")?;
    stdout.flush()
}
