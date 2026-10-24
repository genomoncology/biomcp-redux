//! Consumed command outcomes and their process exit codes.
use std::{io, process::ExitCode};

pub(crate) enum Status {
    Success,
    Usage,
}

pub(crate) fn finish(result: io::Result<()>, status: Status) -> ExitCode {
    match result {
        Ok(()) => match status {
            Status::Success => ExitCode::SUCCESS,
            Status::Usage => ExitCode::from(2),
        },
        Err(_) => {
            crate::output::diagnostic(b"biomcp: output failed\n");
            ExitCode::from(5)
        }
    }
}
