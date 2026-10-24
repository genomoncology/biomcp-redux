//! Consumed command outcomes and their process exit codes.
use std::{io, process::ExitCode};

pub(crate) enum Status {
    Success,
    Usage,
    Absent,
    Failed,
    Local,
}

pub(crate) fn finish(result: io::Result<()>, status: Status) -> ExitCode {
    match result {
        Ok(()) => match status {
            Status::Success => ExitCode::SUCCESS,
            Status::Usage => ExitCode::from(2),
            Status::Absent => ExitCode::from(1),
            Status::Failed => ExitCode::from(4),
            Status::Local => ExitCode::from(5),
        },
        Err(_) => {
            crate::output::diagnostic(b"biomcp: output failed\n");
            ExitCode::from(5)
        }
    }
}

impl From<biodata::Status> for Status {
    fn from(status: biodata::Status) -> Self {
        match status {
            biodata::Status::Complete | biodata::Status::Partial => Self::Success,
            biodata::Status::Absent => Self::Absent,
            biodata::Status::Failed => Self::Failed,
        }
    }
}
