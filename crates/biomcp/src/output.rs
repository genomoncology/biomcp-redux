//! The command output boundary.
use std::io::{self, Write};

pub(crate) fn command(
    outcome: &crate::command::Outcome,
    args: &[std::ffi::OsString],
) -> io::Result<()> {
    if crate::grammar::is_housekeeping(args) {
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
            crate::config::format(
                crate::grammar::parse(args)
                    .ok()
                    .and_then(|p| p.globals.config)
                    .as_ref(),
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

pub(crate) fn diagnostic(message: &'static [u8]) {
    let _ = io::stderr().lock().write_all(message);
}
