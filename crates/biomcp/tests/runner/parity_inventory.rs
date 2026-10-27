//! Classify every harvested case without launching either executable.
use super::{Case, Plan, Selection, load_selection_from};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
    process::ExitCode,
};

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Exceptions {
    #[serde(default)]
    unprojectable: Vec<Unsupported>,
    #[serde(default)]
    covered: Vec<Covered>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Unsupported {
    case: String,
    reason: String,
    release_source: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Covered {
    case: String,
    by_case: String,
    facts: BTreeMap<String, String>,
}
pub(super) fn main(args: Vec<String>) -> ExitCode {
    let result = match args.as_slice() {
        [plan, cases, report] => run(Path::new(plan), Path::new(cases), Path::new(report)),
        _ => Err("usage: --parity-inventory PLAN CASE-DIRECTORY NEW-REPORT.json".into()),
    };
    let result = result.and_then(|report| {
        writeln!(std::io::stdout(), "{report}").map_err(|e| e.to_string())?;
        Ok(report.get("assessment_gaps") == Some(&json!(0))
            && report.get("unsafe_plans") == Some(&json!(0)))
    });
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            if writeln!(std::io::stderr(), "parity inventory: {error}").is_err() {
                return ExitCode::FAILURE;
            }
            ExitCode::FAILURE
        }
    }
}
fn run(plan_path: &Path, root: &Path, output: &Path) -> Result<Value, String> {
    let plan: Plan = serde_json::from_slice(&fs::read(plan_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if plan.baseline_release != "0.9.3" {
        return Err("expected released 0.9.3 plan".into());
    }
    super::registry::load()?;
    let cases = cases(root)?;
    validate_references(&plan, &cases)?;
    let mut rows = Vec::new();
    for (name, case) in &cases {
        rows.push(classify(name, case, &plan, root, &cases)?);
    }
    let gaps = rows
        .iter()
        .filter_map(|row| row.get("gaps").and_then(Value::as_u64))
        .sum::<u64>();
    let unsafe_plans = rows
        .iter()
        .filter(|row| {
            row.get("unsafe_reason")
                .is_some_and(|reason| !reason.is_null())
        })
        .count();
    let mut classes = BTreeMap::<String, usize>::new();
    for row in &rows {
        *classes
            .entry(
                row.get("class")
                    .and_then(Value::as_str)
                    .ok_or("missing class")?
                    .into(),
            )
            .or_default() += 1;
    }
    let report = json!({"baseline_release":"0.9.3", "mode":"dry-inventory", "classes":classes,
        "assessment_gaps":gaps, "unsafe_plans":unsafe_plans, "cases":rows,
        "scope":"classification and declared projections; no executable or provider calls"});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(&mut file, &report).map_err(|e| e.to_string())?;
    Ok(
        json!({"report":output, "cases":cases.len(), "classes":classes,
        "assessment_gaps":gaps,"unsafe_plans":unsafe_plans}),
    )
}
fn cases(root: &Path) -> Result<BTreeMap<String, Case>, String> {
    let mut paths = Vec::new();
    super::super::collect(root, &mut paths).map_err(|e| e.to_string())?;
    if paths.is_empty() {
        return Err("case directory is empty".into());
    }
    paths
        .into_iter()
        .map(|path| {
            let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let mut case: Case = serde_saphyr::from_str(&text).map_err(|e| e.to_string())?;
            super::super::load_floors(&mut case, &path)?;
            for step in &case.steps {
                super::super::assertions::prepare(step)?;
            }
            let name = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_str()
                .ok_or("case filename must be UTF-8")?
                .replace('\\', "/");
            Ok((name, case))
        })
        .collect()
}
fn validate_references(plan: &Plan, cases: &BTreeMap<String, Case>) -> Result<(), String> {
    let mut selected = BTreeSet::new();
    for selection in &plan.cases {
        if !cases.contains_key(&selection.case)
            || !selected.insert((&selection.case, selection.step))
        {
            return Err("unknown or duplicate selected case step".into());
        }
    }
    let mut exceptions = BTreeSet::new();
    for case in plan
        .inventory
        .unprojectable
        .iter()
        .map(|e| &e.case)
        .chain(plan.inventory.covered.iter().map(|e| &e.case))
    {
        if !cases.contains_key(case) || !exceptions.insert(case) {
            return Err("unknown or duplicate inventory exception".into());
        }
        if selected.iter().any(|(name, _)| name == &case) {
            return Err("case has both a direct plan and an inventory exception".into());
        }
    }
    Ok(())
}
fn classify(
    name: &str,
    case: &Case,
    plan: &Plan,
    root: &Path,
    cases: &BTreeMap<String, Case>,
) -> Result<Value, String> {
    let surfaces = json!({"cli":case.steps.iter().filter(|s| s.run.is_some()).count(),
        "mcp":case.steps.iter().filter(|s| s.mcp.is_some()).count(),
        "follow":case.steps.iter().filter(|s| s.follow.is_some()).count(),
        "walk":case.steps.iter().filter(|s| s.walk.is_some()).count()});
    if let Some(exception) = plan.inventory.unprojectable.iter().find(|e| e.case == name) {
        if exception.reason.trim().is_empty() || !exception.release_source.starts_with("src/") {
            return Err(format!(
                "{name}: unprojectable requires a reason grounded in release source"
            ));
        }
        return Ok(
            json!({"case":name,"class":"unprojectable", "reason":exception.reason,
            "release_source":exception.release_source,"surfaces":surfaces,"gaps":0}),
        );
    }
    if let Some(link) = plan.inventory.covered.iter().find(|e| e.case == name) {
        covered(link, case, plan, cases)?;
        return Ok(json!({"case":name,"class":"covered","by_case":link.by_case,
            "facts":link.facts,"surfaces":surfaces,"gaps":0}));
    }
    if let Some(reason) = sequence_reason(case) {
        return Ok(json!({"case":name,"class":"sequence","reason":reason,
            "surfaces":surfaces,"gaps":0}));
    }
    let selection = plan.cases.iter().find(|s| s.case == name && s.step == 0);
    direct(name, case, selection, root, surfaces)
}
fn sequence_reason(case: &Case) -> Option<&'static str> {
    if case.steps.len() != 1 {
        return Some("ordered case steps require shared case state");
    }

    if case.config.is_some() {
        return Some("configured case requires scenario setup");
    }
    let step = case.steps.first()?;
    if step.follow.is_some() || step.walk.is_some() {
        return Some("followed action or complete walk requires an ordered command sequence");
    }
    if step.offline
        || step
            .run
            .as_ref()
            .is_some_and(|r| r.first().is_some_and(|v| v == "sync"))
    {
        return Some("bundle or offline case requires retained case state");
    }
    None
}
fn direct(
    name: &str,
    case: &Case,
    selection: Option<&Selection>,
    root: &Path,
    surfaces: Value,
) -> Result<Value, String> {
    let step = case.steps.first().ok_or("case requires steps")?;
    let missing = step
        .expect
        .keys()
        .filter(|pointer| {
            selection.is_none_or(|s| {
                (!s.facts.contains_key(*pointer) && !s.unprojectable_facts.contains_key(*pointer))
                    || s.assertion_pointers
                        .as_ref()
                        .is_some_and(|p| !p.contains(*pointer))
            })
        })
        .collect::<Vec<_>>();
    let unsafe_reason = selection.and_then(|s| load_selection_from(s, root).err());
    let gaps = if selection.is_none_or(|s| s.facts.is_empty()) {
        missing.len().max(1)
    } else {
        missing.len()
    };
    Ok(
        json!({"case":name, "class":if gaps == 0 && unsafe_reason.is_none() {"direct"} else {"unclassified"},
        "surfaces":surfaces,"missing_projections":missing,"gaps":gaps,
        "unsafe_reason":unsafe_reason,"unprojectable_facts":selection.map(|s| &s.unprojectable_facts)}),
    )
}
fn covered(
    link: &Covered,
    case: &Case,
    plan: &Plan,
    cases: &BTreeMap<String, Case>,
) -> Result<(), String> {
    let target = cases
        .get(&link.by_case)
        .ok_or("covered case target missing")?;
    if link.case == link.by_case
        || case.steps.len() != 1
        || target.steps.len() != 1
        || !plan
            .cases
            .iter()
            .any(|s| s.case == link.by_case && s.step == 0)
    {
        return Err("coverage link needs a distinct direct planned case".into());
    }
    let own = case.steps.first().ok_or("covered step missing")?;
    if sequence_reason(target).is_some() || case.config != target.config {
        return Err("coverage target must be a direct case with the same configuration".into());
    }
    let target = target.steps.first().ok_or("target step missing")?;
    let selection = plan
        .cases
        .iter()
        .find(|s| s.case == link.by_case && s.step == 0)
        .ok_or("coverage target requires a direct plan")?;
    if target.expect.keys().any(|p| {
        (!selection.facts.contains_key(p) && !selection.unprojectable_facts.contains_key(p))
            || selection
                .assertion_pointers
                .as_ref()
                .is_some_and(|selected| !selected.contains(p))
    }) {
        return Err("coverage target has unprojected facts".into());
    }
    if target.run.is_none()
        || own.run.as_ref().or(own.mcp.as_ref()) != target.run.as_ref()
        || own.expect.is_empty()
        || link.facts.len() != own.expect.len()
    {
        return Err("coverage link must preserve the command and every asserted fact".into());
    }
    for (pointer, expected) in &own.expect {
        let mapped = link
            .facts
            .get(pointer)
            .ok_or("coverage link missing assertion")?;
        if target.expect.get(mapped) != Some(expected) {
            return Err("covered fact differs from its explicit CLI assertion".into());
        }
    }
    Ok(())
}
