//! The hidden child consumes a single PDF after installing operating-system limits.
use biomcp_adapters::pdf::{self, Extraction};
use std::{
    io::{Read, Write},
    process::ExitCode,
};

pub(crate) const ARGUMENT: &str = "--pdf-worker";
const MEMORY: u64 = 512 * 1024 * 1024;
const CPU_SECONDS: u64 = 15;

pub(crate) fn run() -> ExitCode {
    let result = match limits().and_then(|()| input()) {
        Ok(bytes) => pdf::extract(&bytes),
        Err(_) => Extraction::BadReply,
    };
    let written = serde_json::to_writer(std::io::stdout().lock(), &result)
        .and_then(|()| std::io::stdout().flush().map_err(Into::into));
    if written.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
fn input() -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(pdf::MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > pdf::MAX_BYTES || !bytes.starts_with(b"%PDF-") {
        return Err(std::io::Error::other("invalid PDF input"));
    }
    Ok(bytes)
}
#[cfg(unix)]
fn limits() -> std::io::Result<()> {
    rlimit::Resource::CORE.set(0, 0)?;
    rlimit::Resource::CPU.set(CPU_SECONDS, CPU_SECONDS)?;
    rlimit::Resource::AS.set(MEMORY, MEMORY)?;
    Ok(())
}
#[cfg(not(unix))]
fn limits() -> std::io::Result<()> {
    // Fail closed until this platform supplies an allocation and CPU boundary.
    Err(std::io::Error::other("PDF process limits unavailable"))
}
