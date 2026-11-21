//! Feature fda-document-text: PDF reads run in a capped child process (1075).
//!
//! The binary's debug-only test switch makes the child hang or, on Linux, allocate without
//! end, so these trials show the time limit and the memory cap stopping a read.
use super::{launch::Environment, response_cases::copy_capture};

#[cfg(all(debug_assertions, target_os = "linux"))]
#[path = "pdf_hosted.rs"]
mod hosted;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::Path,
    process::Stdio,
    time::{Duration, Instant},
};
type Check = Result<(), Box<dyn std::error::Error>>;

const REPLAY: &str = "diagnostic/fda-letter-damaged";
const SIDECAR: &str =
    "drugsfda/b6fedc7c36cb8d646d8526e90aaf852ed1fa58480b304bff41048b5e3fda1106-0.bin";
const REFUSED: &str = concat!(
    "This PDF could not be read safely: reading it went over BioMCP's memory or time limit ",
    "and was stopped, so none of its text was read. Fetch the document another way."
);
/// The call's time limit in these trials, and the slack allowed past it.
const SECONDS: u64 = 4;
const SLACK: Duration = Duration::from_secs(3);

pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    #[cfg(all(debug_assertions, target_os = "linux"))]
    trials.extend(hosted::trials());
    if cfg!(debug_assertions) {
        trials.push(super::transcript_trial(
            "fda-document-text/pdf-read-stopped-at-time-limit",
            || hang().map_err(|e| e.to_string().into()),
        ));
    }
    if cfg!(all(debug_assertions, target_os = "linux")) {
        trials.push(super::transcript_trial(
            "fda-document-text/pdf-read-stopped-at-memory-cap",
            || allocate().map_err(|e| e.to_string().into()),
        ));
    }
    trials.push(super::transcript_trial(
        "fda-document-text/recorded-pdfs-read-the-same-in-child",
        || same().map_err(|e| e.to_string().into()),
    ));
    trials.push(super::transcript_trial(
        "fda-document-text/pdf-memory-config-has-an-upper-bound",
        || memory_config().map_err(|e| e.to_string().into()),
    ));
    trials
}

fn memory_config() -> Check {
    let env = Environment::new()?;
    let args = super::configuration::cached(
        &["get", "gene", "BRAF"],
        &env.root().join("case"),
        Some(&super::configuration::root()),
    );
    env.config(&json!({"pdf_memory_limit_mib": 4096, "pdf_max_reads": 2}))?;
    super::configuration::invoke(&env, args.clone(), 0)?;
    env.config(&json!({"pdf_memory_limit_mib": 4097}))?;
    let rejected = super::configuration::invoke(&env, args, 5)?;
    if !rejected.to_string().contains("pdf_memory_limit_mib") {
        return Err(format!("memory cap error did not name its field: {rejected}").into());
    }
    Ok(())
}

fn args() -> Vec<String> {
    [
        "get",
        "drug",
        "NDA208065",
        "approvals",
        "--source",
        "drugsfda",
        "--document-id",
        "65680",
    ]
    .map(str::to_owned)
    .to_vec()
}

/// One read of the letter replaced by `pdf`: the reply, the time taken and the parent's peak
/// resident memory in KiB where the system reports it.
fn read(pdf: &[u8]) -> Result<(Value, Duration, u64), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    env.config(&json!({ "command_timeout_seconds": SECONDS }))?;
    let replay = env.root().join("recordings");
    copy_capture(REPLAY, &replay, |_, _| {})?;
    fs::write(replay.join(SIDECAR), pdf)?;
    let mut all = args();
    all.extend(env.globals(Some(&replay), false));
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &all);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let started = Instant::now();
    let mut child = command.spawn()?;
    let status = Path::new("/proc")
        .join(child.id().to_string())
        .join("status");
    let mut peak = 0;
    while child.try_wait()?.is_none() {
        peak = peak.max(high_water(&status));
        std::thread::sleep(Duration::from_millis(5));
    }
    let took = started.elapsed();
    let output = command.wait_with_output(child)?;
    if output.status.code() != Some(0) || !output.stderr.is_empty() {
        return Err(format!(
            "exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let value: Value = serde_json::from_slice(&output.stdout)?;
    super::schema::Registry::embedded()?.validate(&value, &args(), true)?;
    Ok((value, took, peak))
}

/// The process's peak resident memory in KiB, or zero once it has exited.
fn high_water(status: &Path) -> u64 {
    fs::read_to_string(status).map_or(0, |text| {
        text.lines()
            .find_map(|line| line.strip_prefix("VmHWM:"))
            .and_then(|rest| rest.trim().trim_end_matches("kB").trim().parse().ok())
            .unwrap_or(0)
    })
}

fn refused(value: &Value, took: Duration) -> Check {
    let state = value.pointer("/result/approvals/data/records/0/application_docs/0/text/state");
    if state != Some(&json!("refused")) {
        return Err(format!("expected a refused read, got {state:?}").into());
    }
    let notices = value
        .pointer("/sources/0/notices")
        .and_then(Value::as_array)
        .ok_or("notices")?;
    if !notices
        .iter()
        .any(|notice| notice.get("message") == Some(&json!(REFUSED)))
    {
        return Err(format!("refusal notice missing: {notices:?}").into());
    }
    let at = "/result/approvals/data/records/0/application_docs/0/text";
    let listed = value
        .get("incomplete")
        .and_then(Value::as_array)
        .ok_or("incomplete list")?;
    if !listed.iter().any(|item| item.get("at") == Some(&json!(at))) {
        return Err(format!("refused read missing from incomplete: {listed:?}").into());
    }
    if took > Duration::from_secs(SECONDS) + SLACK {
        return Err(format!("read took {took:?}, over the {SECONDS} s limit").into());
    }
    Ok(())
}

/// A child that never answers is killed at the call's time limit and the read is refused.
fn hang() -> Check {
    let (value, took, _) = read(b"%PDF-1.7\n%BIOMCP-TEST hang\n")?;
    refused(&value, took)
}

/// A child that allocates without end is stopped by the cap, and the parent's memory stays
/// near what an ordinary read of the same reply takes.
fn allocate() -> Check {
    let damaged = fs::read(
        super::launch::fixtures()
            .join("replies")
            .join(REPLAY)
            .join(SIDECAR),
    )?;
    let (_, _, ordinary) = read(&damaged)?;
    let (value, took, peak) = read(b"%PDF-1.7\n%BIOMCP-TEST allocate\n")?;
    refused(&value, took)?;
    if ordinary == 0 || peak == 0 || peak > ordinary + 32 * 1024 {
        return Err(
            format!("parent peak {peak} KiB against {ordinary} KiB for an ordinary read").into(),
        );
    }
    Ok(())
}

/// Every recorded PDF gives the same answer through the child as read in this process.
fn same() -> Check {
    let mut pdfs = Vec::new();
    sidecars(&super::launch::fixtures().join("replies"), &mut pdfs)?;
    if pdfs.len() < 5 {
        return Err(format!("only {} recorded PDFs found", pdfs.len()).into());
    }
    let cap = (1024_u64 << 20).to_string();
    let env = Environment::new()?;
    for path in pdfs {
        let mut bytes = b"100\n".to_vec();
        bytes.extend(fs::read(&path)?);
        let input = env.root().join("input");
        fs::write(&input, &bytes)?;
        let mut command = env.command(
            env!("CARGO_BIN_EXE_biomcp"),
            &[
                "__pdf-read".into(),
                cap.clone(),
                "21".into(),
                std::process::id().to_string(),
            ],
        );
        command
            .stdin(fs::File::open(&input)?)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let child = command.spawn()?;
        let output = command.wait_with_output(child)?;
        if !output.status.success() || output.stdout != biomcp_query::pdf_child::serve(&bytes) {
            return Err(format!("{} reads differently through the child", path.display()).into());
        }
    }
    Ok(())
}

fn sidecars(directory: &Path, found: &mut Vec<std::path::PathBuf>) -> Check {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            sidecars(&path, found)?;
        } else if path.extension().is_some_and(|extension| extension == "bin")
            && fs::read(&path)?.starts_with(b"%PDF-")
        {
            found.push(path);
        }
    }
    Ok(())
}
