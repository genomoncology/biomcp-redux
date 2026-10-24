//! Live diagnostics are separate from replay assertions and never join the gate.
use super::{Case, Step, action, collect, launch::Environment, schema};
use biomcp_fetch::{Observation, Shape};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    time::{Duration, Instant},
};
const MAX_CASES: usize = 1024;
const MAX_CHANGES: usize = 16;
const CASE_SAMPLE: usize = 20;
const RUN_BUDGET: Duration = Duration::from_secs(3600);

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

pub(crate) fn run(list: bool, details: bool) -> Result<Value, String> {
    let root = super::launch::fixtures().join("cases");
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
        if list {
            results.push(json!({"case": name, "eligible": true}));
            continue;
        }
        let result = check(&case, &registry, &mut sources, started);
        results.push(match result {
            Ok(report) => json!({"case": name, "result": report}),
            Err(error) => json!({"case": name, "error": error}),
        });
    }
    if list {
        return Ok(json!({"mode": "live-check-plan", "cases": results}));
    }
    let mut skipped = BTreeMap::<String, usize>::new();
    for result in &results {
        if let Some(reason) = result.get("skipped").and_then(Value::as_str) {
            *skipped.entry(reason.into()).or_default() += 1;
        }
    }
    let checked: Vec<_> = results
        .iter()
        .filter(|r| r.get("skipped").is_none())
        .collect();
    Ok(
        json!({"mode": "live-check", "elapsed_ms": started.elapsed().as_millis(), "sources": sources, "discovered": results.len(), "checked": checked.len(), "skipped": skipped, "case_sample": checked.iter().take(if details { MAX_CASES } else { CASE_SAMPLE }).collect::<Vec<_>>(), "omitted": checked.len().saturating_sub(if details { MAX_CASES } else { CASE_SAMPLE })}),
    )
}
fn eligibility(case: &Case) -> Option<&str> {
    match case.live_check.as_deref() {
        Some("public")
            if case.replay.is_some()
                && case.config.is_none()
                && case.steps.iter().any(eligible_step) =>
        {
            None
        }
        Some("public") => Some("configured, local-only or offline-only case"),
        Some(reason) => Some(reason),
        None => Some("provenance not reviewed for live use"),
    }
}
fn eligible_step(step: &Step) -> bool {
    if step.offline || step.exit > 1 {
        return false;
    }
    step.follow.is_some()
        || step
            .run
            .as_ref()
            .or(step.mcp.as_ref())
            .and_then(|args| args.first())
            .is_some_and(|verb| matches!(verb.as_str(), "get" | "search" | "sync" | "article"))
}
fn check(
    case: &Case,
    registry: &schema::Registry,
    sources: &mut BTreeMap<String, Value>,
    started: Instant,
) -> Result<Value, String> {
    let replay = super::launch::fixtures()
        .join("replies")
        .join(case.replay.as_ref().ok_or("no recording")?);
    let baseline = Environment::new().map_err(|e| e.to_string())?;
    let live = Environment::new().map_err(|e| e.to_string())?;
    let mut previous = (None, None);
    let mut changes = Vec::new();
    let mut ordinal = 0;
    let mut skipped = Vec::new();
    for (index, step) in case.steps.iter().enumerate() {
        if !eligible_step(step) {
            skipped.push(json!({"step": index + 1, "reason": "local validation or offline step"}));
            continue;
        }
        let mut args = (
            arguments(step, previous.0.as_ref())?,
            arguments(step, previous.1.as_ref())?,
        );
        let pages = step
            .walk
            .as_ref()
            .map_or(1, |walk| walk.max_pages.min(1000));
        for page in 0..pages {
            if started.elapsed() >= RUN_BUDGET {
                return Err("run budget exhausted".into());
            }
            ordinal += 1;
            if ordinal > 128 {
                return Err("case command budget exhausted".into());
            }
            let (expected, before) = invoke(&baseline, &args.0, Some(&replay), ordinal)?;
            if ordinal == 1 && before.is_empty() {
                return Err("recorded command has no provider observations".into());
            }
            let reply = invoke(&live, &args.1, None, ordinal);
            pace(&before, &reply, started);
            let (actual, after) = reply?;
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
    Ok(json!({"drift": changes, "skipped_steps": skipped}))
}
fn pace(
    before: &[Observation],
    reply: &Result<(Value, Vec<Observation>), String>,
    started: Instant,
) {
    let observed = reply
        .as_ref()
        .ok()
        .map(|(_, records)| records.as_slice())
        .unwrap_or_default();
    let interval = before
        .iter()
        .chain(observed)
        .map(|o| o.interval_ms)
        .max()
        .unwrap_or(0);
    // Even a killed child may have sent a request just before it died.
    std::thread::sleep(
        Duration::from_millis(interval).min(RUN_BUDGET.saturating_sub(started.elapsed())),
    );
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
    let mut omitted = 0;
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
        if changes.len() > MAX_CHANGES {
            omitted += changes.len() - MAX_CHANGES;
            changes.truncate(MAX_CHANGES);
        }
    }
    for old in before {
        if !after.iter().any(|item| item.identity == old.identity) {
            if changes.len() < MAX_CHANGES {
                changes.push(format!("{}: request absent", old.identity));
            } else {
                omitted += 1;
            }
        }
    }
    if omitted > 0 {
        changes.push(format!("{omitted} more drift details omitted"));
    }
}
