//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn command(
    outcome: &crate::command::Outcome,
    args: &[std::ffi::OsString],
) -> io::Result<()> {
    if crate::grammar::is_housekeeping(args)
        && crate::grammar::parse(args)
            .ok()
            .is_none_or(|parsed| parsed.help.is_none())
    {
        return bytes(&outcome.cli_bytes);
    }
    let (format, config) = crate::grammar::presentation(args).unwrap_or_default();
    let format = format.or_else(|| crate::config::format(config.as_ref()));
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
