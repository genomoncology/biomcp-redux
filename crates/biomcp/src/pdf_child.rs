//! PDF reads in a capped child process of this binary (1075).
//!
//! The parent writes the PDF to an unnamed temporary file that becomes the child's standard
//! input, and gives it another as standard output, so neither side can block on a pipe the
//! other stopped reading. The child lowers its own limits before it reads a byte: on Linux its
//! address space to the configured cap, everywhere the size of what it may write and core dumps
//! to nothing. macOS does not enforce an address-space limit, so there the time limit alone
//! bounds a read. The parent polls the child until it exits or the call's deadline passes or is
//! cancelled, then kills and reaps it; a guard does the same if the parent unwinds, so no child
//! outlives its call. Any exit but a clean one, or an answer over its bound, is one refusal.
//!
//! The child is entered only by the exact arguments `__pdf-read BYTES`. The command grammar,
//! help and MCP tools do not know it, and it reads only standard input, so it cannot be pointed
//! at a file or URL.

use std::{
    ffi::OsString,
    io::{Read, Seek, Write},
    path::PathBuf,
    process::{Command, ExitCode, Stdio},
    time::Duration,
};

/// The hidden first argument that makes this binary a PDF-reading child.
const CHILD: &str = "__pdf-read";
/// The default address-space cap: real FDA labels peak near 160 MB resident (1068).
const DEFAULT_MEMORY_MIB: u64 = 1024;
/// Longest pause between checks on a running child.
const LONGEST_PAUSE: Duration = Duration::from_millis(20);

/// The cap in bytes when this process was started as a PDF-reading child.
pub(crate) fn requested(arguments: &[OsString]) -> Option<u64> {
    match arguments {
        [first, cap] if first.as_os_str() == CHILD => cap.to_str()?.parse().ok(),
        _ => None,
    }
}

/// The child: lower the limits, read the PDF from standard input, answer on standard output.
pub(crate) fn serve(cap: u64) -> ExitCode {
    if limit(cap).is_err() {
        return ExitCode::FAILURE;
    }
    let most = biomcp_query::pdf_child::MAX_BYTES.saturating_add(1);
    let mut bytes = Vec::new();
    let read = std::io::stdin().lock().take(most).read_to_end(&mut bytes);
    if read.is_err() || bytes.len() as u64 >= most {
        return ExitCode::FAILURE;
    }
    #[cfg(debug_assertions)]
    exercise(&bytes);
    let answer = biomcp_query::pdf_child::serve(&bytes);
    let mut out = std::io::stdout().lock();
    match out.write_all(&answer).and_then(|()| out.flush()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

/// Test-only switch, compiled into debug builds only: a PDF whose second line is
/// `%BIOMCP-TEST hang` never answers, and on Linux one whose second line is
/// `%BIOMCP-TEST allocate` allocates until the cap stops it, so the trials can show the time
/// limit and the cap at work without a hostile file.
#[cfg(debug_assertions)]
fn exercise(bytes: &[u8]) {
    if bytes.starts_with(b"%PDF-1.7\n%BIOMCP-TEST hang\n") {
        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    #[cfg(target_os = "linux")]
    if bytes.starts_with(b"%PDF-1.7\n%BIOMCP-TEST allocate\n") {
        let mut held: Vec<Vec<u8>> = Vec::new();
        loop {
            held.push(vec![1; 1 << 20]);
        }
    }
}

/// Lower this process's limits; the address-space cap only where the system enforces it.
#[cfg(unix)]
fn limit(cap: u64) -> std::io::Result<()> {
    use rustix::process::{Resource, Rlimit, setrlimit};
    let set = |resource, value: u64| {
        setrlimit(
            resource,
            Rlimit {
                current: Some(value),
                maximum: Some(value),
            },
        )
    };
    #[cfg(target_os = "linux")]
    set(Resource::As, cap)?;
    #[cfg(not(target_os = "linux"))]
    let _ = cap;
    let output = u64::try_from(biomcp_query::pdf_child::MAX_OUTPUT).unwrap_or(u64::MAX);
    set(Resource::Fsize, output)?;
    set(Resource::Core, 0)?;
    Ok(())
}

#[cfg(not(unix))]
fn limit(_cap: u64) -> std::io::Result<()> {
    Ok(())
}

/// The configured PDF reader: children of `executable` capped at `mebibytes`, 1 GiB by default.
pub(crate) fn isolate(
    executable: Option<PathBuf>,
    mebibytes: Option<std::num::NonZeroU64>,
) -> std::sync::Arc<dyn biomcp_fetch::Isolate> {
    let mebibytes = mebibytes.map_or(DEFAULT_MEMORY_MIB, std::num::NonZeroU64::get);
    std::sync::Arc::new(Child::new(executable, mebibytes))
}

/// Runs PDF reads in a child of `executable` capped at `cap` bytes.
#[derive(Debug)]
pub(crate) struct Child {
    executable: Option<PathBuf>,
    cap: u64,
}

impl Child {
    /// A runner for this binary with a cap of `mebibytes`; with no executable every read is
    /// refused rather than run unbounded.
    pub(crate) fn new(executable: Option<PathBuf>, mebibytes: u64) -> Self {
        Self {
            executable,
            cap: mebibytes.saturating_mul(1 << 20),
        }
    }

    fn answer(
        &self,
        input: &[u8],
        max_output: usize,
        expired: &dyn Fn() -> bool,
    ) -> std::io::Result<Option<Vec<u8>>> {
        let Some(executable) = &self.executable else {
            return Ok(None);
        };
        let mut stdin = tempfile::tempfile()?;
        stdin.write_all(input)?;
        stdin.rewind()?;
        let mut stdout = tempfile::tempfile()?;
        let child = Command::new(executable)
            .arg(CHILD)
            .arg(self.cap.to_string())
            .env_clear()
            .stdin(stdin)
            .stdout(stdout.try_clone()?)
            .stderr(Stdio::null())
            .spawn()?;
        let mut child = Reaped(child);
        let mut pause = Duration::from_millis(1);
        let status = loop {
            if let Some(status) = child.0.try_wait()? {
                break status;
            }
            if expired() {
                return Ok(None);
            }
            std::thread::sleep(pause);
            pause = (pause * 2).min(LONGEST_PAUSE);
        };
        let limit = u64::try_from(max_output).unwrap_or(u64::MAX);
        if !status.success() || stdout.metadata()?.len() > limit {
            return Ok(None);
        }
        stdout.rewind()?;
        let mut answer = Vec::new();
        stdout.take(limit).read_to_end(&mut answer)?;
        Ok(Some(answer))
    }
}

impl biomcp_fetch::Isolate for Child {
    fn run(
        &self,
        input: &[u8],
        max_output: usize,
        expired: &dyn Fn() -> bool,
    ) -> Result<Vec<u8>, biomcp_fetch::Contained> {
        match self.answer(input, max_output, expired) {
            Ok(Some(answer)) => Ok(answer),
            Ok(None) | Err(_) => Err(biomcp_fetch::Contained),
        }
    }
}

/// A child that is killed and reaped when its call ends, however it ends.
struct Reaped(std::process::Child);

impl Drop for Reaped {
    fn drop(&mut self) {
        if matches!(self.0.try_wait(), Ok(None)) {
            let _ = self.0.kill();
        }
        let _ = self.0.wait();
    }
}
