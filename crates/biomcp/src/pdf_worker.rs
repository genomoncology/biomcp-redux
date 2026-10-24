//! The hidden child consumes a single PDF after installing its resource limits.
use biomcp_adapters::pdf::{self, Extraction};
use std::{
    io::{Read, Write},
    process::ExitCode,
};

pub(crate) const ARGUMENT: &str = biomcp_fetch::PdfWorker::ARGUMENT;
#[global_allocator]
static HEAP: cap::Cap<std::alloc::System> = cap::Cap::new(std::alloc::System, usize::MAX);
const MEMORY: u64 = biomcp_fetch::PdfWorker::MEMORY;
#[cfg(unix)]
const CPU_SECONDS: u64 = biomcp_fetch::PdfWorker::CPU_SECONDS;

pub(crate) fn run() -> ExitCode {
    let result = match HEAP
        .set_limit(MEMORY as usize)
        .map_err(|()| std::io::Error::other("PDF heap limit"))
        .and_then(|()| limits())
        .and_then(|()| input())
    {
        Ok(bytes) => pdf::extract(&bytes),
        Err(_) => Extraction::BadReply,
    };
    let written = serde_json::to_writer(std::io::stdout().lock(), &result)
        .map_err(std::io::Error::other)
        .and_then(|()| std::io::stdout().flush());
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
    // Some platforms reject address-space limits; the heap cap remains mandatory.
    let _ = rlimit::Resource::AS.set(MEMORY, MEMORY);
    Ok(())
}
#[cfg(not(unix))]
fn limits() -> std::io::Result<()> {
    Ok(())
}
