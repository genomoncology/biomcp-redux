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
#[path = "parity_projection.rs"]
mod projection;
#[path = "parity_recording.rs"]
mod recording;
#[path = "parity_registry.rs"]
mod registry;
#[path = "parity_request.rs"]
mod request;
#[path = "parity_sequence.rs"]
mod sequence;
#[path = "parity_sequence_plans.rs"]
mod sequence_plans;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    baseline_release: String,
    cases: Vec<Selection>,
    #[serde(default)]
    sequences: Vec<sequence::Sequence>,
    #[serde(default)]
    batches: Vec<String>,
    #[serde(default)]
    inventory: inventory::Exceptions,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    case: String,
    step: usize,
    baseline_command: Vec<String>,
    #[serde(default)]
    candidate_command: Option<Vec<String>>,
    #[serde(default)]
    pages: Vec<sequence::Page>,
    #[serde(default)]
    baseline_items: Option<String>,
    #[serde(default)]
    baseline_walk_once: Option<UnsupportedFact>,
    #[serde(default)]
    unsupported_command: Option<UnsupportedFact>,
    spelling_table: String,
    release_source: Vec<String>,
    baseline_globals: Vec<String>,
    #[serde(default)]
    command_selectors: BTreeMap<String, String>,
    recordings: Vec<recording::Route>,
    #[serde(default)]
    source_free: bool,
    #[serde(default)]
    stored_only: bool,
    #[serde(default)]
    baseline_exit: i32,
    #[serde(default)]
    baseline_stream: execution::JsonStream,
    #[serde(default)]
    assertion_pointers: Option<Vec<String>>,
    facts: BTreeMap<String, projection::Projection>,
    #[serde(default)]
    unprojectable_facts: BTreeMap<String, UnsupportedFact>,
}

#[derive(Clone, Deserialize, serde::Serialize)]
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
            if writeln!(std::io::stdout(), "{report}").is_err() {
                return ExitCode::FAILURE;
            }
            if report.get("assessment_gaps").and_then(Value::as_u64) == Some(0)
                && report.get("regressions").and_then(Value::as_u64) == Some(0)
            {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            if writeln!(std::io::stderr(), "parity: {error}").is_err() {
                return ExitCode::FAILURE;
            }
            ExitCode::FAILURE
        }
    }
}
fn load_plan(path: &Path) -> Result<Plan, String> {
    let read = |path: &Path| -> Result<Plan, String> {
        serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    };
    let mut plan = read(path)?;
    for batch in std::mem::take(&mut plan.batches) {
        let name = Path::new(&batch);
        if name
            .components()
            .any(|part| !matches!(part, std::path::Component::Normal(_)))
        {
            return Err("batch must be a relative path without traversal".into());
        }
        let child = read(&path.parent().ok_or("plan parent missing")?.join(name))?;
        if child.baseline_release != plan.baseline_release || !child.batches.is_empty() {
            return Err("batch must target the same release without nested batches".into());
        }
        plan.cases.extend(child.cases);
        plan.sequences.extend(child.sequences);
        plan.inventory.extend(child.inventory);
    }
    Ok(plan)
}
fn run(plan: &Path, baseline: &Path, out: &Path) -> Result<Value, String> {
    let plan = load_plan(plan)?;
    if plan.baseline_release != "0.9.3"
        || plan.cases.is_empty() && plan.sequences.is_empty()
        || plan.cases.len() + plan.sequences.len() > 1024
        || !baseline.is_absolute()
    {
        return Err("expected nonempty selection and absolute released 0.9.3 executable".into());
    }
    let cases = plan
        .cases
        .iter()
        .map(load_selection)
        .collect::<Result<Vec<_>, _>>()?;
    let sequences = sequence_plans::resolve(&plan)?;
    sequence::validate(&sequences)?;
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
    for (index, sequence) in sequences.iter().enumerate() {
        rows.extend(sequence::compare(sequence, baseline, out, index)?);
    }
    let (gaps, regressions) = totals(&rows);
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
fn totals(rows: &[Value]) -> (usize, usize) {
    let facts = rows
        .iter()
        .flat_map(|r| {
            ["facts", "mcp_facts"]
                .into_iter()
                .filter_map(|name| r.get(name).and_then(Value::as_array))
        })
        .flatten()
        .collect::<Vec<_>>();
    let gaps = facts
        .iter()
        .filter(|f| {
            f.pointer("/baseline/verdict") == Some(&json!("assessment_gap"))
                || f.pointer("/candidate/verdict") == Some(&json!("assessment_gap"))
        })
        .count()
        + rows
            .iter()
            .filter(|row| {
                row.get("facts")
                    .and_then(Value::as_array)
                    .is_some_and(Vec::is_empty)
                    && ["baseline", "candidate"].iter().any(|side| {
                        row.get(side)
                            .is_some_and(|output| output.get("assessment_error").is_some())
                    })
            })
            .count();
    let regressions = facts
        .iter()
        .filter(|f| {
            f.pointer("/baseline/verdict") == Some(&json!("right"))
                && f.pointer("/candidate/verdict") == Some(&json!("wrong"))
        })
        .count();
    (gaps, regressions)
}
fn load_selection(selection: &Selection) -> Result<Case, String> {
    load_selection_from(selection, &super::launch::fixtures().join("cases"))
}
fn load_selection_from(selection: &Selection, cases: &Path) -> Result<Case, String> {
    load_selection_mode(selection, cases, false)
}
fn load_selection_mode(selection: &Selection, cases: &Path, setup: bool) -> Result<Case, String> {
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
    let mut resolved = step.clone();
    if let Some(command) = &selection.candidate_command {
        if step
            .run
            .as_ref()
            .is_some_and(|original| original != command)
        {
            return Err("candidate command differs from harvested command".into());
        }
        resolved.run = Some(command.clone());
    } else if resolved.run.is_none() {
        resolved.run = resolved.mcp.clone();
    }
    let step = &resolved;
    step.run
        .as_ref()
        .ok_or("selection requires a reviewed candidate command")?;
    if step.expect.is_empty() && !setup {
        return Err("selection has no fact assertions".into());
    }
    for (pointer, baseline_pointer) in &selection.facts {
        let expected = step
            .expect
            .get(pointer)
            .ok_or("projection names an unknown harvested assertion")?;
        for pointer in baseline_pointer.pointers() {
            let mut projected = step.clone();
            projected.expect = BTreeMap::from([(pointer.to_owned(), expected.clone())]);
            assertions::prepare(&projected)?;
        }
    }
    for (pointer, reason) in &selection.unprojectable_facts {
        if !step.expect.contains_key(pointer) || selection.facts.contains_key(pointer) {
            return Err("unsupported fact is unknown or also projected".into());
        }
        reason.validate()?;
    }
    if selection.release_source.is_empty()
        || selection
            .release_source
            .iter()
            .any(|p| !p.starts_with("src/"))
    {
        return Err("projection requires released command and serializer provenance".into());
    }
    if let Some(reason) = &selection.unsupported_command {
        reason.validate()?;
        if !setup
            || !selection.baseline_command.is_empty()
            || !selection.recordings.is_empty()
            || !selection.facts.is_empty()
            || !selection.pages.is_empty()
        {
            return Err(
                "unsupported sequence command cannot declare released execution or facts".into(),
            );
        }
        return Ok(case);
    }
    if selection.baseline_command.is_empty() {
        return Err("selection requires a baseline command".into());
    }
    if let Some(pointers) = &selection.assertion_pointers
        && (pointers.is_empty() || pointers.iter().any(|p| !step.expect.contains_key(p)))
    {
        return Err("selected assertion missing from harvested case".into());
    }
    validate_baseline(selection, step)?;
    Ok(case)
}
fn validate_baseline(selection: &Selection, step: &Step) -> Result<(), String> {
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
    request::validate_command(selection, step)?;
    Ok(())
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
        .join(case.replay.as_deref().unwrap_or("parity-empty"));
    let replay = if case.replay.is_some() {
        replay
    } else {
        let empty = out.join(format!("{index}-empty-replay"));
        fs::create_dir(&empty).map_err(|e| e.to_string())?;
        empty
    };
    let current = execution::candidate(case, step, command, &replay, out, index)?;
    let mut old = recording::baseline(selection, baseline, &replay, out, index)
        .unwrap_or_else(|error| json!({"assessment_error":error}));
    let mut current = current;
    if old.get("assessment_error").is_none() {
        execution::assess_exit(&mut old, selection.baseline_exit)?;
    }
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
        "spelling_table":selection.spelling_table,"release_source":selection.release_source,
        "baseline_routes":selection.recordings,"baseline":old,"candidate":current,"facts":facts,
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
                let left = projected_verdict(old, old_pointer, &assertion);
                let right = verdict(current, pointer, &assertion);
                result.push(json!({"pointer":pointer,"baseline_projection":old_pointer,
                    "assertion":format!("{assertion:?}"),"expected":expected,
                    "baseline":left,"candidate":right}));
            }
        }
    }
    Ok(result)
}
fn projected_verdict(
    output: &Value,
    projection: &projection::Projection,
    assertion: &assertions::Assertion<'_>,
) -> Value {
    if let Some(error) = output.get("assessment_error") {
        return json!({"verdict":"assessment_gap","reason":error});
    }
    let Some(reply) = output.get("parsed") else {
        return json!({"verdict":"assessment_gap","reason":"command did not emit JSON"});
    };
    match projection.select(reply) {
        Ok(actual) => match assertion.check("released projection", &actual) {
            Ok(()) => json!({"verdict":"right","actual":actual,"projection":projection}),
            Err(error) => {
                json!({"verdict":"wrong","actual":actual,"reason":error,"projection":projection})
            }
        },
        Err(error) => json!({"verdict":"assessment_gap","reason":error,"projection":projection}),
    }
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
