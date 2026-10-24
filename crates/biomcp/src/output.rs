//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn command(
    outcome: &crate::command::Outcome,
    args: &[std::ffi::OsString],
) -> io::Result<()> {
    let format = args.windows(2).find_map(|pair| {
        (pair[0] == "--format").then(|| pair[1].to_str()).flatten()
    }).or_else(|| args.iter().filter_map(|arg| arg.to_str()).find_map(|arg| arg.strip_prefix("--format=")));
    let format = format.map(|value| if value == "md" { crate::grammar::Format::Md } else { crate::grammar::Format::Json }).or_else(crate::config::format);
    let bytes = match format {
        Some(crate::grammar::Format::Md) => crate::view::markdown(&outcome.value)?,
        _ => outcome.cli_bytes.clone(),
    };
    let mut stdout = io::stdout().lock();
    stdout.write_all(&bytes)?;
    stdout.flush()
}

pub(crate) fn diagnostic(message: &'static [u8]) {
    let _ = io::stderr().lock().write_all(message);
}
