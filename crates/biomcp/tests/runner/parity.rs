//! Compare selected harvested facts through both actual executables.
use super::{Case, Step, assertions, launch::Environment, load_floors};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path, process::ExitCode};
#[path = "parity_execution.rs"]
mod execution;
#[path = "parity_inventory.rs"]
mod inventory;

pub(crate) fn inventory_main(args: Vec<String>) -> ExitCode {
    inventory::main(args)
}
#[path = "parity_recording.rs"]
mod recording;
#[path = "parity_registry.rs"]
mod registry;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    baseline_release: String,
    cases: Vec<Selection>,
    #[serde(default)]
    inventory: inventory::Exceptions,
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
    #[serde(default)]
    unprojectable_facts: BTreeMap<String, UnsupportedFact>,
}

#[derive(Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct UnsupportedFact {
    reason: String,
    release_source: String,
}
impl UnsupportedFact {
    fn validate(&self) -> Result<(), String> {
        if self.reason.trim().is_empty() || !self.release_source.starts_with("src/") {
            return Err(
                "unsupported fact requires a reason grounded in released serialization".into(),
            );
        }
        Ok(())
    }
}
pub(crate) fn main(args: Vec<String>) -> ExitCode {
    let result = match args.as_slice() {
        [plan, baseline, out] => run(Path::new(plan), Path::new(baseline), Path::new(out)),
        _ => Err("usage: --parity PLAN BASELINE NEW-EVIDENCE-DIRECTORY".into()),
    };
    use std::io::Write;
    match result {
        Ok(report) => {
            let _ = writeln!(std::io::stdout(), "{report}");
            if report.get("assessment_gaps").and_then(Value::as_u64) == Some(0)
                && report.get("regressions").and_then(Value::as_u64) == Some(0)
            {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
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
    if plan.baseline_release != "0.9.3"
        || plan.cases.is_empty()
        || plan.cases.len() > 1024
        || !baseline.is_absolute()
    {
        return Err("expected nonempty selection and absolute released 0.9.3 executable".into());
    }
    let cases = plan
        .cases
        .iter()
        .map(load_selection)
        .collect::<Result<Vec<_>, _>>()?;
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
    for (index, (selection, case)) in plan.cases.iter().zip(&cases).enumerate() {
        rows.push(compare(selection, case, baseline, out, index)?);
    }
    let facts = rows
        .iter()
        .filter_map(|r| r.get("facts").and_then(Value::as_array))
        .flatten()
        .collect::<Vec<_>>();
    let gaps = facts
        .iter()
        .filter(|f| {
            f.pointer("/baseline/verdict") == Some(&json!("assessment_gap"))
                || f.pointer("/candidate/verdict") == Some(&json!("assessment_gap"))
        })
        .count();
    let regressions = facts
        .iter()
        .filter(|f| {
            f.pointer("/baseline/verdict") == Some(&json!("right"))
                && f.pointer("/candidate/verdict") == Some(&json!("wrong"))
        })
        .count();
    let report = json!({"baseline_release":"0.9.3", "mode":"recorded-parity",
        "baseline_binary":execution::fingerprint(baseline)?,
        "candidate_binary":execution::fingerprint(Path::new(env!("CARGO_BIN_EXE_biomcp")))?,
        "assessment_gaps":gaps,"regressions":regressions,"cases":rows, "scope":"explicit selected facts; no whole-comparison claim"});
    fs::write(
        out.join("report.json"),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(
        json!({"report":out.join("report.json"), "cases":rows.len(),"assessment_gaps":gaps,"regressions":regressions}),
    )
}
fn load_selection(selection: &Selection) -> Result<Case, String> {
    load_selection_from(selection, &super::launch::fixtures().join("cases"))
}
fn load_selection_from(selection: &Selection, cases: &Path) -> Result<Case, String> {
    let path = cases.join(&selection.case);
    let mut case: Case =
        serde_saphyr::from_str(&fs::read_to_string(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    load_floors(&mut case, &path)?;
    let step = case
        .steps
        .get(selection.step)
        .ok_or("selected step missing")?;
    assertions::prepare(step)?;
    step.run
        .as_ref()
        .ok_or("selection requires a direct CLI step")?;
    if step.expect.is_empty() {
        return Err("selection has no fact assertions".into());
    }
    for (pointer, baseline_pointer) in &selection.facts {
        let expected = step
            .expect
            .get(pointer)
            .ok_or("projection names an unknown harvested assertion")?;
        let mut projection = step.clone();
        projection.expect = BTreeMap::from([(baseline_pointer.clone(), expected.clone())]);
        assertions::prepare(&projection)?;
    }
    for (pointer, reason) in &selection.unprojectable_facts {
        if !step.expect.contains_key(pointer) || selection.facts.contains_key(pointer) {
            return Err("unsupported fact is unknown or also projected".into());
        }
        reason.validate()?;
    }
    if selection.baseline_command.is_empty() {
        return Err("selection requires a baseline command".into());
    }
    if let Some(pointers) = &selection.assertion_pointers
        && (pointers.is_empty() || pointers.iter().any(|p| !step.expect.contains_key(p)))
    {
        return Err("selected assertion missing from harvested case".into());
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
    registry::validate(selection)?;
    Ok(case)
}
fn compare(
    selection: &Selection,
    case: &Case,
    baseline: &Path,
    out: &Path,
    index: usize,
) -> Result<Value, String> {
    let step = case
        .steps
        .get(selection.step)
        .ok_or("selected step missing")?;
    let command = step
        .run
        .as_ref()
        .ok_or("selection requires a direct CLI step")?;
    let replay = super::launch::fixtures()
        .join("replies")
        .join(case.replay.as_ref().ok_or("recording required")?);
    let current = execution::candidate(case, step, command, &replay, out, index)?;
    let mut old = recording::baseline(selection, baseline, &replay, out, index)?;
    let mut current = current;
    execution::assess_exit(&mut old, 0)?;
    execution::assess_exit(&mut current, step.exit)?;
    let facts = facts(step, selection, &old, &current)?;
    let unselected = step
        .expect
        .keys()
        .filter(|pointer| {
            selection
                .assertion_pointers
                .as_ref()
                .is_some_and(|selected| !selected.contains(*pointer))
        })
        .collect::<Vec<_>>();
    Ok(
        json!({"case":selection.case,"story":case.story,"step":selection.step,
        "spelling_table":selection.spelling_table,"baseline":old,"candidate":current,"facts":facts,
        "selection":selection.assertion_pointers,
        "unselected_assertion_pointers":unselected}),
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
        let mut fact = step.clone();
        fact.expect = BTreeMap::from([(pointer.clone(), expected.clone())]);
        let prepared = assertions::prepare(&fact)?;
        for (_, _, assertions) in prepared {
            for assertion in assertions {
                if let Some(reason) = selection.unprojectable_facts.get(pointer) {
                    result.push(json!({"pointer":pointer,"expected":expected,"assertion":format!("{assertion:?}"),
                        "baseline":{"verdict":"unprojectable","reason":reason.reason,"release_source":reason.release_source},
                        "candidate":verdict(current, pointer, &assertion)}));
                    continue;
                }
                let Some(old_pointer) = selection.facts.get(pointer) else {
                    result.push(json!({"pointer":pointer,"expected":expected,"assertion":format!("{assertion:?}"),
                        "baseline":{"verdict":"assessment_gap","reason":format!("missing baseline projection for {pointer}")},
                        "candidate":{"verdict":"assessment_gap","reason":"paired assessment needs a baseline projection"}}));
                    continue;
                };
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
    if let Some(error) = output.get("assessment_error") {
        return json!({"verdict":"assessment_gap","reason":error});
    }
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

// Harness contract: the public mode must reject an actual binary of the wrong release.
pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    vec![libtest_mimic::Trial::test(
        "parity/reject-wrong-baseline-release",
        || {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let plan = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/parity/sample.json");
            let args = vec![
                "--parity".into(),
                plan.display().to_string(),
                env!("CARGO_BIN_EXE_biomcp").into(),
                env.root().join("evidence").display().to_string(),
            ];
            #[expect(
                clippy::disallowed_methods,
                reason = "The harness launches its own actual public parity entry point"
            )]
            let runner = std::env::current_exe().map_err(|e| e.to_string())?;
            let result = env
                .command(runner, &args)
                .output()
                .map_err(|e| e.to_string())?;
            if result.status.success()
                || !String::from_utf8_lossy(&result.stderr)
                    .contains("baseline must report version 0.9.3")
            {
                return Err("wrong baseline release was not refused".into());
            }
            Ok(())
        },
    )]
}
