//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn command(
    outcome: &crate::command::Outcome,
    args: &[std::ffi::OsString],
) -> io::Result<()> {
    let parsed = crate::grammar::parse(args).ok();
    if crate::grammar::is_housekeeping(args)
        && parsed.as_ref().is_none_or(|parsed| parsed.help.is_none())
    {
        return bytes(&outcome.cli_bytes);
    }
    let format = parsed
        .as_ref()
        .and_then(|parsed| parsed.globals.format)
        .or_else(|| {
            crate::config::format(
                parsed
                    .as_ref()
                    .and_then(|parsed| parsed.globals.config.as_ref()),
            )
        });
    let bytes = match format {
        Some(crate::grammar::Format::Md) if outcome.value.get("schema").is_some() => {
            crate::view::markdown(&outcome.value)?
        }
        _ => outcome.cli_bytes.clone(),
    };
    self::bytes(&bytes)
}

pub(crate) fn bytes(bytes: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    match stdout.write_all(bytes).and_then(|()| stdout.flush()) {
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result,
    }
}

pub(crate) fn diagnostic(message: &[u8]) {
    let _ = io::stderr().lock().write_all(message);
}
