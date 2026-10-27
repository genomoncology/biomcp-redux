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
    let mut words = args.iter().take_while(|arg| *arg != "--");
    let mut format = None;
    while let Some(word) = words.next().and_then(|arg| arg.to_str()) {
        if word == "--format" {
            format = words.next().and_then(|arg| arg.to_str());
        } else if let Some(value) = word.strip_prefix("--format=") {
            format = Some(value);
        }
    }
    let format = format
        .map(|value| {
            if value == "md" {
                crate::grammar::Format::Md
            } else {
                crate::grammar::Format::Json
            }
        })
        .or_else(|| {
            crate::config::format(parsed.and_then(|parsed| parsed.globals.config).as_ref())
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
    stdout.write_all(bytes)?;
    stdout.flush()
}

pub(crate) fn diagnostic(message: &'static [u8]) {
    let _ = io::stderr().lock().write_all(message);
}
