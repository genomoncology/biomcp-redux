//! Compare selected harvested facts through both actual executables.
use super::{Case, Step, assertions, launch::Environment, load_floors};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path, process::ExitCode};
#[path = "parity_execution.rs"]
mod execution;
#[path = "parity_recording.rs"]
mod recording;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    baseline_release: String,
    cases: Vec<Selection>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    case: String,
    step: usize,
    baseline_command: Vec<String>,
    spelling_table: String,
    baseline_globals: Vec<String>,
    recordings: Vec<recording::Route>,
    #[serde(default)]
    assertion_pointers: Option<Vec<String>>,
    facts: BTreeMap<String, String>,
}

pub(super) fn main(args: Vec<String>) -> ExitCode {
    let result = match args.as_slice() {
        [plan, baseline, out] => run(Path::new(plan), Path::new(baseline), Path::new(out)),
        _ => Err("usage: --parity PLAN BASELINE NEW-EVIDENCE-DIRECTORY".into()),
    };
    use std::io::Write;
    match result {
        Ok(report) => {
            let _ = writeln!(std::io::stdout(), "{report}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            let _ = writeln!(std::io::stderr(), "parity: {error}");
            ExitCode::FAILURE
        }
    }
}
fn run(plan: &Path, baseline: &Path, out: &Path) -> Result<Value, String> {
    let plan: Plan = serde_json::from_slice(&fs::read(plan).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if plan.baseline_release != "0.9.3" || plan.cases.is_empty() || !baseline.is_absolute() {
        return Err("expected nonempty selection and absolute released 0.9.3 executable".into());
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let version = env
        .command(baseline, &["--version".into()])
        .output()
        .map_err(|e| e.to_string())?;
    if !version.status.success()
        || String::from_utf8_lossy(&version.stdout)
            .split_whitespace()
            .nth(1)
            != Some("0.9.3")
    {
        return Err("baseline must report version 0.9.3".into());
    }
    fs::create_dir(out).map_err(|e| e.to_string())?;
    fs::write(out.join("baseline-version.stdout"), version.stdout).map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for (index, selection) in plan.cases.iter().enumerate() {
        rows.push(compare(selection, baseline, out, index)?);
    }
    let report = json!({"baseline_release":"0.9.3", "mode":"recorded-parity",
        "cases":rows, "scope":"explicit selected facts; no whole-comparison claim"});
    fs::write(
        out.join("report.json"),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(json!({"report":out.join("report.json"), "cases":rows.len()}))
}
fn compare(
    selection: &Selection,
    baseline: &Path,
    out: &Path,
    index: usize,
) -> Result<Value, String> {
    let path = super::launch::fixtures()
        .join("cases")
        .join(&selection.case);
    let mut case: Case =
        serde_saphyr::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    load_floors(&mut case, &path)?;
    let step = case
        .steps
        .get(selection.step)
        .ok_or("selected step missing")?;
    assertions::prepare(step)?;
    let command = step
        .run
        .as_ref()
        .ok_or("selection requires a direct CLI step")?;
    if selection.facts.is_empty() || selection.baseline_command.is_empty() {
        return Err("selection requires explicit facts and baseline command".into());
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let table =
        fs::read_to_string(root.join(&selection.spelling_table)).map_err(|e| e.to_string())?;
    let leaf = selection
        .baseline_command
        .iter()
        .take(2)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if !table.contains("0.9") || !table.contains(&leaf) {
        return Err("missing spelling-table provenance".into());
    }
    let replay = super::launch::fixtures()
        .join("replies")
        .join(case.replay.as_ref().ok_or("recording required")?);
    let current_env = Environment::new().map_err(|e| e.to_string())?;
    if let Some(config) = &case.config {
        current_env.config(config).map_err(|e| e.to_string())?;
    }
    let mut current_args = command.clone();
    current_args.extend(current_env.globals(Some(&replay), step.offline));
    let current = execution::invoke(
        &current_env,
        Path::new(env!("CARGO_BIN_EXE_biomcp")),
        &current_args,
        &out.join(format!("{index}-candidate")),
    )?;
    let old = recording::baseline(selection, baseline, &replay, out, index)?;
    let facts = facts(step, selection, &old, &current)?;
    Ok(
        json!({"case":selection.case,"story":case.story,"step":selection.step,
        "spelling_table":selection.spelling_table,"baseline":old,"candidate":current,"facts":facts,
        "unselected_assertion_pointers":step.expect.keys().filter(|p| !selection.facts.contains_key(*p)).collect::<Vec<_>>()}),
    )
}
fn facts(
    step: &Step,
    selection: &Selection,
    old: &Value,
    current: &Value,
) -> Result<Vec<Value>, String> {
    let mut result = Vec::new();
    for (pointer, expected) in &step.expect {
        if selection
            .assertion_pointers
            .as_ref()
            .is_some_and(|v| !v.contains(pointer))
        {
            continue;
        }
        let Some(old_pointer) = selection.facts.get(pointer) else {
            result.push(json!({"pointer":pointer,"expected":expected,
                "baseline":{"verdict":"assessment_gap","reason":format!("missing baseline projection for {pointer}")},
                "candidate":{"verdict":"assessment_gap","reason":"paired assessment needs a baseline projection"}}));
            continue;
        };
        let expected = step
            .expect
            .get(pointer)
            .ok_or("selected assertion missing from harvested case")?;
        let mut fact = step.clone();
        fact.expect = BTreeMap::from([(pointer.clone(), expected.clone())]);
        let prepared = assertions::prepare(&fact)?;
        for (_, _, assertions) in prepared {
            for assertion in assertions {
                let left = verdict(old, old_pointer, &assertion);
                let right = verdict(current, pointer, &assertion);
                result.push(json!({"pointer":pointer,"baseline_pointer":old_pointer,
                    "assertion":format!("{assertion:?}"),"expected":expected,
                    "baseline":left,"candidate":right}));
            }
        }
    }
    Ok(result)
}
fn verdict(output: &Value, pointer: &str, assertion: &assertions::Assertion<'_>) -> Value {
    let Some(reply) = output.get("parsed") else {
        return json!({"verdict":"assessment_gap","reason":"command did not emit JSON"});
    };
    match assertions::select(reply, pointer) {
        Ok(actual) => match assertion.check(pointer, &actual) {
            Ok(()) => json!({"verdict":"right","actual":actual}),
            Err(error) => json!({"verdict":"wrong","actual":actual,"reason":error}),
        },
        Err(error) => json!({"verdict":"wrong","reason":error}),
    }
}
