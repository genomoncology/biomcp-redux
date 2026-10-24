//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn command(outcome: &crate::command::Outcome) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    match outcome.schema_text {
        Some(text) => stdout.write_all(text.as_bytes())?,
        None => {
            serde_json::to_writer(&mut stdout, &outcome.value)?;
            stdout.write_all(b"\n")?;
        }
    }
    stdout.flush()
}

pub(crate) fn diagnostic(message: &'static [u8]) {
    let _ = io::stderr().lock().write_all(message);
}
