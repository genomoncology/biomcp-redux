//! Live diagnostics are separate from replay assertions and never join the gate.
use super::{Case, Step, action, collect, launch::Environment, schema};
use biomcp_fetch::{Observation, Shape};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
const MAX_CASES: usize = 1024;
const RUN_BUDGET: Duration = Duration::from_secs(3600);
const PACE: Duration = Duration::from_secs(2);

pub(super) fn drift(before: &Shape, after: &Shape) -> Vec<String> {
    let mut changes = Vec::new();
    for key in before
        .fields
        .keys()
        .filter(|key| !after.fields.contains_key(*key))
    {
        changes.push(format!("field removed: {key}"));
    }
    for (key, positive) in &before.positive {
        if *positive && after.positive.get(key) == Some(&false) {
            changes.push(format!("count dropped to zero: {key}"));
        }
    }
    if before.header != after.header {
        changes.push("bundle header changed".into());
    }
    changes
}

pub(crate) fn run() -> Result<Value, String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases");
    let mut paths = Vec::new();
    collect(&root, &mut paths).map_err(|e| e.to_string())?;
    paths.sort();
    if paths.len() > MAX_CASES {
        return Err("live case discovery limit exceeded".into());
    }
    let registry = schema::Registry::embedded()?;
    let started = Instant::now();
    let mut results = Vec::new();
    let mut sources = BTreeMap::<String, Value>::new();
    for path in paths {
        let name = path
            .strip_prefix(&root)
            .map_err(|e| e.to_string())?
            .display()
            .to_string();
        let case: Case =
            serde_saphyr::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let skip = eligibility(&case);
        if let Some(reason) = skip {
            results.push(json!({"case": name, "skipped": reason}));
            continue;
        }
        if started.elapsed() >= RUN_BUDGET {
            results.push(json!({"case": name, "skipped": "run budget exhausted"}));
            continue;
        }
        let result = check(&case, &registry, &mut sources);
        results.push(match result {
            Ok(changes) => json!({"case": name, "drift": changes}),
            Err(error) => json!({"case": name, "error": error}),
        });
        // The existing limiter handles requests within a command. No new child
        // starts until a full keyless interval follows the preceding child's exit.
        std::thread::sleep(PACE);
    }
    Ok(
        json!({"mode": "live-check", "elapsed_ms": started.elapsed().as_millis(), "sources": sources, "cases": results}),
    )
}
fn eligibility(case: &Case) -> Option<&str> {
    match case.live_check.as_deref() {
        Some("public")
            if case.replay.is_some()
                && case.config.is_none()
                && case
                    .steps
                    .iter()
                    .all(|s| !s.offline && s.exit == 0 && !s.is_error) =>
        {
            None
        }
        Some("public") => Some("local, configured or failure case"),
        Some(reason) => Some(reason),
        None => Some("provenance not reviewed for live use"),
    }
}
fn check(
    case: &Case,
    registry: &schema::Registry,
    sources: &mut BTreeMap<String, Value>,
) -> Result<Vec<String>, String> {
    let replay = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies")
        .join(case.replay.as_ref().ok_or("no recording")?);
    let baseline = Environment::new().map_err(|e| e.to_string())?;
    let live = Environment::new().map_err(|e| e.to_string())?;
    let mut previous = (None, None);
    let mut changes = Vec::new();
    let mut ordinal = 0;
    for step in &case.steps {
        let mut args = (
            arguments(step, previous.0.as_ref())?,
            arguments(step, previous.1.as_ref())?,
        );
        let pages = step
            .walk
            .as_ref()
            .map_or(1, |walk| walk.max_pages.min(1000));
        for page in 0..pages {
            ordinal += 1;
            if ordinal > 128 {
                return Err("case command budget exhausted".into());
            }
            let (expected, before) = invoke(&baseline, &args.0, Some(&replay), ordinal)?;
            let (actual, after) = invoke(&live, &args.1, None, ordinal)?;
            std::thread::sleep(PACE);
            if expected.get("status") != actual.get("status") {
                changes.push("status changed from recorded result".into());
            }
            if registry.validate(&actual, &args.1, true).is_err() {
                changes.push("output schema failure".into());
            }
            compare(&before, &after, &mut changes, sources);
            previous = (Some(expected), Some(actual));
            if step.walk.is_none() {
                break;
            }
            let next = (
                action(previous.0.as_ref().ok_or("missing baseline")?, "page"),
                action(previous.1.as_ref().ok_or("missing live result")?, "page"),
            );
            match next {
                (Ok(before), Ok(after)) => args = (before, after),
                (Err(_), Err(_)) => break,
                _ => {
                    changes.push("page continuation changed".into());
                    break;
                }
            }
            if page + 1 == pages {
                changes.push("walk reached its bounded page count".into());
            }
        }
    }
    Ok(changes)
}
fn arguments(step: &Step, previous: Option<&Value>) -> Result<Vec<String>, String> {
    match &step.follow {
        Some(kind) => action(previous.ok_or("follow lacks predecessor")?, kind),
        None => step
            .run
            .as_ref()
            .or(step.mcp.as_ref())
            .cloned()
            .ok_or_else(|| "missing command".into()),
    }
}
fn invoke(
    environment: &Environment,
    args: &[String],
    replay: Option<&Path>,
    ordinal: usize,
) -> Result<(Value, Vec<Observation>), String> {
    let path = environment
        .root()
        .join(format!("observations-{ordinal}.jsonl"));
    let mut args = args.to_vec();
    let mut globals = environment.globals(replay, false);
    globals.extend(["--observe".into(), path.display().to_string()]);
    let position = args
        .iter()
        .position(|arg| arg == "--")
        .unwrap_or(args.len());
    args.splice(position..position, globals);
    let mut command = environment.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let (_, stdout, _) =
        super::process::Process::new(command.spawn().map_err(|e| e.to_string())?)?.finish()?;
    let value = serde_json::from_slice(&stdout).map_err(|_| "command did not return JSON")?;
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    if text.len() > 8 * 1024 * 1024 {
        return Err("observation file exceeds bound".into());
    }
    let observations = text
        .lines()
        .map(|line| serde_json::from_str(line).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    Ok((value, observations))
}
pub(super) fn compare(
    before: &[Observation],
    after: &[Observation],
    changes: &mut Vec<String>,
    sources: &mut BTreeMap<String, Value>,
) {
    for item in after {
        let name = serde_json::to_value(item.source)
            .unwrap_or(Value::Null)
            .as_str()
            .unwrap_or("unknown")
            .to_owned();
        let totals = sources
            .entry(name)
            .or_insert(json!({"attempts": 0, "bytes": 0, "elapsed_ms": 0}));
        for (key, value) in [
            ("attempts", 1),
            ("bytes", item.bytes),
            ("elapsed_ms", item.elapsed_ms),
        ] {
            let previous = totals.get(key).and_then(Value::as_u64).unwrap_or(0);
            if let Some(slot) = totals.get_mut(key) {
                *slot = json!(previous.saturating_add(value));
            }
        }
        if let Some(old) = before.iter().find(|old| old.identity == item.identity) {
            if !before
                .iter()
                .any(|old| old.identity == item.identity && old.status == item.status)
            {
                changes.push(format!(
                    "{}: new HTTP status {:?}",
                    item.identity, item.status
                ));
            }
            changes.extend(
                drift(&old.shape, &item.shape)
                    .into_iter()
                    .map(|s| format!("{}: {s}", item.identity)),
            );
        } else {
            changes.push(format!("{}: new request", item.identity));
        }
    }
    for old in before {
        if !after.iter().any(|item| item.identity == old.identity) {
            changes.push(format!("{}: request absent", old.identity));
        }
    }
}
