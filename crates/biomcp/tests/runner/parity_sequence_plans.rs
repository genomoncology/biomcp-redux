//! Reuse reviewed direct declarations only for the same recorded question.
use super::{Case, Plan, Selection, sequence::Sequence};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

pub(super) fn resolve(plan: &Plan) -> Result<Vec<Sequence>, String> {
    if plan
        .sequences
        .iter()
        .all(|sequence| !sequence.steps.is_empty())
    {
        return Ok(plan.sequences.clone());
    }
    let root = super::super::launch::fixtures().join("cases");
    let cases = super::inventory::cases(&root)?;
    plan.sequences.iter().map(|sequence| {
        if !sequence.steps.is_empty() { return Ok(sequence.clone()); }
        let case = cases.get(&sequence.case).ok_or("unknown sequence case")?;
        let steps = case.steps.iter().enumerate().map(|(index, step)| {
            declaration(&sequence.case, index, step, case, plan, &cases, &root)?
                .ok_or_else(|| format!("{} step {index}: no reviewed direct declaration preserves this question", sequence.case))
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Sequence { case: sequence.case.clone(), steps })
    }).collect()
}

pub(super) fn inventory(
    name: &str,
    case: &Case,
    plan: &Plan,
    root: &Path,
    cases: &BTreeMap<String, Case>,
) -> Value {
    let mut steps = Vec::new();
    let mut gaps = Vec::new();
    for (index, step) in case.steps.iter().enumerate() {
        match declaration(name, index, step, case, plan, cases, root) {
            Ok(Some(selection)) => steps.push(selection),
            other => {
                let reason = other.err().unwrap_or_else(|| {
                    "no reviewed direct declaration preserves this question and configuration"
                        .into()
                });
                gaps.push(
                    json!({"step":index,"command":step.run.as_ref().or(step.mcp.as_ref()),
                    "follow":step.follow,"reason":reason}),
                );
            }
        }
    }
    if gaps.is_empty() {
        let sequence = Sequence {
            case: name.into(),
            steps,
        };
        if let Err(reason) = super::sequence::validate_from(&sequence, root) {
            gaps.push(json!({"case":name,"reason":reason}));
        }
    }
    json!({"missing_steps":gaps,"gaps":gaps.len(),"reused_steps":steps_len(case, &gaps)})
}
fn steps_len(case: &Case, gaps: &[Value]) -> usize {
    case.steps
        .len()
        .saturating_sub(gaps.iter().filter(|gap| gap.get("step").is_some()).count())
}
fn declaration(
    name: &str,
    index: usize,
    step: &super::Step,
    case: &Case,
    plan: &Plan,
    cases: &BTreeMap<String, Case>,
    root: &Path,
) -> Result<Option<Selection>, String> {
    if step.follow.is_some() || step.walk.is_some() {
        return Ok(None);
    }
    let command = step
        .run
        .as_ref()
        .or(step.mcp.as_ref())
        .ok_or("command missing")?;
    let template = plan
        .cases
        .iter()
        .filter(|selection| {
            let Some(original) = cases.get(&selection.case) else {
                return false;
            };
            original.config == case.config
                && original.steps.get(selection.step).is_some_and(|original| {
                    original.exit == step.exit
                        && original.run.as_ref().or(original.mcp.as_ref()) == Some(command)
                })
                && selection.recordings.iter().all(|route| {
                    case.replay.as_ref().is_some_and(|replay| {
                        root.parent().is_some_and(|fixtures| {
                            fixtures
                                .join("replies")
                                .join(replay)
                                .join(&route.capture)
                                .is_file()
                        })
                    })
                })
                && step.expect.keys().all(|pointer| {
                    selection.facts.contains_key(pointer)
                        || selection.unprojectable_facts.contains_key(pointer)
                })
        })
        .min_by_key(|selection| (selection.case != name, selection.step != index));
    let Some(template) = template else {
        return super::shape::resolve(&plan.shapes, &plan.cases, name, index, step, case, root);
    };
    let mut selected = template.clone();
    selected.case = name.into();
    selected.step = index;
    selected.assertion_pointers = None;
    selected
        .facts
        .retain(|pointer, _| step.expect.contains_key(pointer));
    selected
        .unprojectable_facts
        .retain(|pointer, _| step.expect.contains_key(pointer));
    Ok(Some(selected))
}
