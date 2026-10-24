//! Consumed command outcomes and their process exit codes.
use std::{io, process::ExitCode};

pub(crate) enum Status {
    Success,
    Usage,
    Expired,
    Absent,
    Failed,
    Local,
}

pub(crate) fn finish(result: io::Result<()>, status: Status) -> ExitCode {
    match result {
        Ok(()) => match status {
            Status::Success => ExitCode::SUCCESS,
            Status::Usage => ExitCode::from(2),
            Status::Expired => ExitCode::from(3),
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

impl From<biomcp_types::Status> for Status {
    fn from(status: biomcp_types::Status) -> Self {
        match status {
            biomcp_types::Status::Complete | biomcp_types::Status::Partial => Self::Success,
            biomcp_types::Status::Absent => Self::Absent,
            biomcp_types::Status::Failed => Self::Failed,
        }
    }
}
