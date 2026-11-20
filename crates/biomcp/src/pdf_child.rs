//! PDF reads in a capped child process of this binary (1075).
//!
//! The parent writes the PDF to an unnamed temporary file that becomes the child's standard
//! input, and gives it another as standard output, so neither side can block on a pipe the
//! other stopped reading. The child limits CPU time, output and core dumps on Unix. On Linux
//! it also caps address space and asks the kernel to kill it when its parent dies. The parent
//! polls the child until it exits or the call's deadline passes or is
//! cancelled, then kills and reaps it; a guard does the same if the parent unwinds, so no child
//! outlives its call. Any exit but a clean one, or an answer over its bound, is one refusal.
//!
//! The child is entered only by the exact arguments `__pdf-read BYTES CPU_SECONDS PARENT_PID`. The command grammar,
//! help and MCP tools do not know it, and it reads only standard input, so it cannot be pointed
//! at a file or URL.

use std::{
    ffi::OsString,
    io::{Read, Seek, Write},
    num::{NonZeroU16, NonZeroU64},
    path::PathBuf,
    process::{Command, ExitCode, Stdio},
    time::Duration,
};

/// The hidden first argument that makes this binary a PDF-reading child.
const CHILD: &str = "__pdf-read";
/// The default address-space cap: real FDA labels peak near 160 MB resident (1068).
const DEFAULT_MEMORY_MIB: u64 = 1024;
/// Highest address-space cap an operator may configure.
pub(crate) const MAX_MEMORY_MIB: u64 = 4096;
/// One reader fits alongside other requests in a 1 GiB hosted task.
pub(crate) const DEFAULT_HOSTED_READS: NonZeroU16 = NonZeroU16::MIN;
/// Longest pause between checks on a running child.
const LONGEST_PAUSE: Duration = Duration::from_millis(20);

/// Limits carried to the child along with the identity of its spawning process.
#[derive(Debug)]
pub(crate) struct ChildLimits {
    memory: u64,
    cpu_seconds: u64,
    parent: u32,
}

/// Enter the PDF worker only with its exact internal arguments.
pub(crate) fn requested(arguments: &[OsString]) -> Option<ChildLimits> {
    match arguments {
        [first, memory, cpu_seconds, parent] if first.as_os_str() == CHILD => Some(ChildLimits {
            memory: memory.to_str()?.parse().ok()?,
            cpu_seconds: cpu_seconds.to_str()?.parse().ok()?,
            parent: parent.to_str()?.parse().ok()?,
        }),
        _ => None,
    }
}

/// The child: lower the limits, read the PDF from standard input, answer on standard output.
pub(crate) fn serve(limits: ChildLimits) -> ExitCode {
    if require_parent(limits.parent)
        .and_then(|()| limit(limits.memory, limits.cpu_seconds))
        .is_err()
    {
        return ExitCode::FAILURE;
    }
    // The PDF follows one short line giving its page bound.
    let most = biomcp_query::pdf_child::MAX_BYTES.saturating_add(9);
    let mut bytes = Vec::new();
    let read = std::io::stdin().lock().take(most).read_to_end(&mut bytes);
    if read.is_err() || bytes.len() as u64 >= most {
        return ExitCode::FAILURE;
    }
    #[cfg(debug_assertions)]
    if let Ok((_, pdf)) = biomcp_query::pdf_child::pages(&bytes) {
        exercise(pdf);
    }
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
    #[cfg(target_os = "linux")]
    if bytes.starts_with(b"%PDF-1.4\n%hold\n") {
        use rustix::process::{Signal, getpid, kill_process};
        let _ = kill_process(getpid(), Signal::STOP);
    }
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

#[cfg(target_os = "linux")]
fn require_parent(expected: u32) -> std::io::Result<()> {
    use rustix::process::{Signal, getppid, set_parent_process_death_signal};
    set_parent_process_death_signal(Some(Signal::KILL))?;
    // The parent may have died between spawning us and installing the signal.
    if getppid().and_then(|pid| u32::try_from(pid.as_raw_pid()).ok()) != Some(expected) {
        return Err(std::io::Error::other("PDF reader parent exited"));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn require_parent(_expected: u32) -> std::io::Result<()> {
    Ok(())
}

/// Lower this process's limits; the address-space cap only where the system enforces it.
#[cfg(unix)]
fn limit(cap: u64, cpu_seconds: u64) -> std::io::Result<()> {
    use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};
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
    let inherited_cpu = getrlimit(Resource::Cpu).maximum.unwrap_or(cpu_seconds);
    set(Resource::Cpu, cpu_seconds.min(inherited_cpu))?;
    let output = u64::try_from(biomcp_query::pdf_child::MAX_OUTPUT).unwrap_or(u64::MAX);
    set(Resource::Fsize, output)?;
    set(Resource::Core, 0)?;
    Ok(())
}

#[cfg(not(unix))]
fn limit(_cap: u64, _cpu_seconds: u64) -> std::io::Result<()> {
    Ok(())
}

/// The configured PDF reader: children of `executable` capped at `mebibytes`, 1 GiB by default.
pub(crate) fn isolate(
    executable: Option<PathBuf>,
    mebibytes: Option<NonZeroU64>,
    max_reads: Option<NonZeroU16>,
) -> std::sync::Arc<biomcp_fetch::Isolator> {
    let mebibytes = mebibytes.map_or(DEFAULT_MEMORY_MIB, NonZeroU64::get);
    let worker = Child {
        executable,
        cap: mebibytes * (1 << 20),
    };
    std::sync::Arc::new(biomcp_fetch::Isolator::new(worker, max_reads))
}

/// Runs PDF reads in a child of `executable` capped at `cap` bytes.
#[derive(Debug)]
pub(crate) struct Child {
    executable: Option<PathBuf>,
    cap: u64,
}

impl Child {
    fn answer(
        &self,
        input: &[u8],
        max_output: usize,
        deadline: &biomcp_fetch::LocalDeadline,
    ) -> std::io::Result<Option<Vec<u8>>> {
        let Some(executable) = &self.executable else {
            return Ok(None);
        };
        let mut stdin = tempfile::tempfile()?;
        stdin.write_all(input)?;
        stdin.rewind()?;
        let mut stdout = tempfile::tempfile()?;
        if deadline.expired() {
            return Ok(None);
        }
        let time = deadline.remaining();
        let cpu_seconds = time.as_secs() + u64::from(time.subsec_nanos() != 0) + 1;
        let child = Command::new(executable)
            .arg(CHILD)
            .arg(self.cap.to_string())
            .arg(cpu_seconds.to_string())
            .arg(std::process::id().to_string())
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
            if deadline.expired() {
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
        deadline: &biomcp_fetch::LocalDeadline,
    ) -> Result<Vec<u8>, biomcp_fetch::Contained> {
        match self.answer(input, max_output, deadline) {
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
