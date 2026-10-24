//! The PDF child has no credentials, a bounded protocol, and the leg's parent deadline.
#![expect(
    clippy::disallowed_types,
    reason = "This reviewed boundary launches only the current executable as its isolated PDF child"
)]
use crate::{Leg, RequestError};
use biodata::Reason;
use std::{
    io::{Read, Write},
    process::{Child, Command, Stdio},
    sync::mpsc,
};

/// Shared limits for the one-shot PDF child and its typed response.
#[derive(Debug)]
pub struct PdfWorker;
impl PdfWorker {
    /// Hidden entry point in the current executable.
    pub const ARGUMENT: &str = "--pdf-worker";
    /// Maximum publisher PDF bytes.
    pub const MAX_BYTES: u64 = 25 * 1024 * 1024;
    /// Maximum pages selected for extraction.
    pub const MAX_PAGES: usize = 100;
    /// Maximum extracted text before JSON encoding.
    pub const MAX_TEXT: usize = 4 * 1024 * 1024;
    /// Maximum serialized response, allowing worst-case JSON escaping.
    pub const MAX_RESULT: u64 = (Self::MAX_TEXT as u64) * 6 + 16 * 1024;
    /// Maximum worker address space in bytes.
    pub const MEMORY: u64 = 512 * 1024 * 1024;
    /// Maximum worker CPU seconds.
    pub const CPU_SECONDS: u64 = 15;
}
impl Leg<'_> {
    /// Extract a PDF in the current executable's resource-limited child.
    /// The caller decodes the typed response; failures never expose child stderr.
    #[expect(
        clippy::disallowed_methods,
        reason = "The transport owns the reviewed PDF process boundary and preserves only subprocess coverage"
    )]
    pub fn pdf_worker(&self, bytes: &[u8]) -> Result<Vec<u8>, RequestError> {
        self.check_source()?;
        let bad = || self.failure(Reason::BadReply, None);
        if bytes.len() as u64 > PdfWorker::MAX_BYTES {
            return Err(bad().into());
        }
        let executable = std::env::current_exe().map_err(|_| bad())?;
        let mut command = Command::new(executable);
        command
            .arg(PdfWorker::ARGUMENT)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {
            command.env("LLVM_PROFILE_FILE", profile);
        }
        let mut child = ChildGuard(command.spawn().map_err(|_| bad())?);
        let mut input = child.0.stdin.take().ok_or_else(bad)?;
        let mut output = child.0.stdout.take().ok_or_else(bad)?;
        std::thread::scope(|scope| {
            scope.spawn(move || {
                let _ = input.write_all(bytes);
            });
            let (sent, receive) = mpsc::sync_channel(1);
            scope.spawn(move || {
                let mut bytes = Vec::new();
                let result = output
                    .by_ref()
                    .take(PdfWorker::MAX_RESULT + 1)
                    .read_to_end(&mut bytes)
                    .and_then(|_| {
                        if bytes.len() as u64 > PdfWorker::MAX_RESULT {
                            Err(std::io::Error::other("PDF result limit"))
                        } else {
                            Ok(bytes)
                        }
                    });
                let _ = sent.send(result);
            });
            let response = receive.recv_timeout(self.remaining());
            // Closing the child also unblocks either pipe before joining its reader and writer.
            let _ = child.0.kill();
            let status = child.0.wait();
            match response {
                Ok(Ok(bytes)) if status.is_ok_and(|status| status.success()) => Ok(bytes),
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    Err(self.failure(Reason::DeadlineExceeded, None).into())
                }
                _ => Err(bad().into()),
            }
        })
    }
}
struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
