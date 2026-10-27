//! Execute declared direct command sequences with isolated case state.
use super::{Environment, Selection, execution, recording};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{fs, path::Path};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Sequence {
    pub(super) case: String,
    pub(super) steps: Vec<Selection>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Page {
    candidate_command: Vec<String>,
    baseline_command: Vec<String>,
    recordings: Vec<recording::Route>,
}
pub(super) fn validate(sequences: &[Sequence]) -> Result<(), String> {
    for sequence in sequences {
        validate_from(sequence, &super::super::launch::fixtures().join("cases"))?;
    }
    Ok(())
}
pub(super) fn validate_from(sequence: &Sequence, root: &Path) -> Result<(), String> {
    if sequence.steps.is_empty() {
        return Err("sequence requires every harvested step in order".into());
    }
    for (index, selection) in sequence.steps.iter().enumerate() {
        if selection.case != sequence.case || selection.step != index {
            return Err("sequence requires every harvested step in order".into());
        }
        let case = super::load_selection_mode(selection, root, true)?;
        if case.steps.len() != sequence.steps.len() {
            return Err("sequence requires every harvested step in order".into());
        }
        let step = case.steps.get(index).ok_or("sequence step missing")?;
        if step.expect.keys().any(|pointer| {
            !selection.facts.contains_key(pointer)
                && !selection.unprojectable_facts.contains_key(pointer)
        }) || selection.assertion_pointers.is_some()
        {
            return Err("sequence must assess every harvested assertion".into());
        }
        if step.walk.is_some() != selection.baseline_items.is_some() {
            return Err("walk requires the released aggregate array pointer".into());
        }
        if let Some(reason) = &selection.baseline_walk_once {
            reason.validate()?;
            if step.walk.is_none() || !selection.pages.is_empty() {
                return Err(
                    "one released walk response requires a walk without released pages".into(),
                );
            }
        }
        for page in &selection.pages {
            let mapped = page_selection(selection, page);
            let mut current = step.clone();
            current.run = Some(page.candidate_command.clone());
            super::registry::validate(&mapped)?;
            super::request::validate_command(&mapped, &current)?;
        }
    }
    Ok(())
}
fn page_selection(selection: &Selection, page: &Page) -> Selection {
    let mut mapped = selection.clone();
    mapped.baseline_command.clone_from(&page.baseline_command);
    mapped.recordings.clone_from(&page.recordings);
    mapped
}
pub(super) fn compare(
    sequence: &Sequence,
    baseline: &Path,
    out: &Path,
    index: usize,
) -> Result<Vec<Value>, String> {
    let root = out.join(format!("sequence-{index}"));
    fs::create_dir(&root).map_err(|e| e.to_string())?;
    let case = super::load_selection_mode(
        sequence.steps.first().ok_or("sequence empty")?,
        &super::super::launch::fixtures().join("cases"),
        true,
    )?;
    let replay = if let Some(replay) = &case.replay {
        super::super::launch::fixtures()
            .join("replies")
            .join(replay)
    } else {
        let empty = root.join("empty-replay");
        fs::create_dir(&empty).map_err(|e| e.to_string())?;
        empty
    };
    let candidate = Environment::new().map_err(|e| e.to_string())?;
    let old = Environment::new().map_err(|e| e.to_string())?;
    let guard = super::super::network_stub::Stub::for_source(
        "replay-only.invalid",
        "application/json",
        |_| Err("candidate attempted a provider connection".into()),
    )?;
    let mut config = case.config.clone().unwrap_or(json!({}));
    execution::field(
        &mut config,
        "proxy",
        json!(format!("http://{}", guard.address)),
    )?;
    candidate.config(&config).map_err(|e| e.to_string())?;
    let mut previous = None;
    let mut rows = Vec::new();
    for (selection, step) in sequence.steps.iter().zip(&case.steps) {
        let directory = root.join(format!("step-{}", selection.step));
        fs::create_dir(&directory).map_err(|e| e.to_string())?;
        let context = Context {
            candidate: &candidate,
            old: &old,
            baseline,
            replay: &replay,
            candidate_replay: case.replay.as_ref().map(|_| replay.as_path()),
        };
        let (mut current, released) =
            execute(selection, step, previous.as_ref(), &context, &directory)?;
        execution::assess_exit(&mut current, step.exit)?;
        previous = current.get("parsed").cloned();
        rows.push(json!({"case":sequence.case,"step":selection.step,
            "evidence_directory":directory,"candidate":current,"baseline":released,
            "mcp_facts":current.get("mcp").map(|mcp| super::facts(step, selection, &released, mcp)).transpose()?,
            "facts":super::facts(step, selection, &released, &current)?}));
    }
    match guard.finish() {
        Ok(requests) if requests.is_empty() => {}
        other => return Err(format!("offline replay guard: {other:?}")),
    }
    Ok(rows)
}
struct Context<'a> {
    candidate: &'a Environment,
    old: &'a Environment,
    baseline: &'a Path,
    replay: &'a Path,
    candidate_replay: Option<&'a Path>,
}
fn execute(
    selection: &Selection,
    step: &super::Step,
    previous: Option<&Value>,
    context: &Context<'_>,
    out: &Path,
) -> Result<(Value, Value), String> {
    let mut exchanges = Vec::new();
    let mut released = Vec::new();
    let result = super::super::run_sequence_with(step, &Vec::new(), previous, |current, _| {
        let ordinal = exchanges.len();
        let command = current
            .run
            .as_ref()
            .or(current.mcp.as_ref())
            .ok_or("command missing")?;
        let mapped = mapped_command(selection, step, command, ordinal)?;
        let actual = candidate(context, current, out, ordinal)?;
        let value = actual
            .get("parsed")
            .cloned()
            .ok_or("candidate did not emit JSON")?;
        exchanges.push(actual);
        if ordinal == 0 || selection.baseline_walk_once.is_none() {
            released.push(if let Some(reason) = &mapped.unsupported_command {
                json!({"unsupported_command":reason})
            } else {
                recording::baseline_in(
                    &mapped,
                    context.baseline,
                    context.replay,
                    out,
                    ordinal,
                    context.old,
                )
                .and_then(|mut output| {
                    execution::assess_exit(&mut output, mapped.baseline_exit)?;
                    Ok(output)
                })
                .unwrap_or_else(|reason| json!({"assessment_error":reason}))
            });
        }
        Ok(value)
    });
    let mut current = exchanges.first().cloned().unwrap_or(json!({}));
    match result {
        Ok(value) => execution::field(&mut current, "parsed", value)?,
        Err(reason) => execution::field(&mut current, "assessment_error", json!(reason))?,
    }
    if selection.baseline_walk_once.is_none() && exchanges.len() != selection.pages.len() + 1 {
        execution::field(
            &mut current,
            "assessment_error",
            json!("not every planned page executed"),
        )?;
    }
    let mcp = exchanges
        .iter()
        .filter_map(|exchange| exchange.get("mcp").cloned())
        .collect::<Vec<_>>();
    if !mcp.is_empty() {
        let reply = aggregate(&mcp, step.walk.as_ref().map(|walk| walk.items.as_str()))?;
        execution::field(&mut current, "mcp", reply)?;
    }
    execution::field(&mut current, "exchanges", json!(exchanges))?;
    let mut baseline = aggregate(&released, selection.baseline_items.as_deref())?;
    execution::field(
        &mut baseline,
        "walk_once",
        json!(selection.baseline_walk_once),
    )?;
    Ok((current, baseline))
}
fn mapped_command(
    selection: &Selection,
    step: &super::Step,
    command: &[String],
    ordinal: usize,
) -> Result<Selection, String> {
    if ordinal == 0 {
        let expected = selection
            .candidate_command
            .as_ref()
            .or(step.run.as_ref())
            .or(step.mcp.as_ref())
            .ok_or("follow requires a reviewed candidate command")?;
        if command != expected {
            return Err("follow differs from reviewed command".into());
        }
        Ok(selection.clone())
    } else if selection.baseline_walk_once.is_some() {
        Ok(selection.clone())
    } else {
        let page = selection
            .pages
            .get(ordinal - 1)
            .ok_or("unplanned walk page")?;
        if command != page.candidate_command {
            return Err("walk differs from reviewed command".into());
        }
        Ok(page_selection(selection, page))
    }
}
fn candidate(
    context: &Context<'_>,
    step: &super::Step,
    out: &Path,
    ordinal: usize,
) -> Result<Value, String> {
    let globals = context
        .candidate
        .globals(context.candidate_replay, step.offline);
    let mut actual = if let Some(command) = &step.run {
        execution::invoke(
            context.candidate,
            Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &super::super::launch::arguments(command, &globals),
            &out.join(format!("{ordinal}-candidate")),
        )?
    } else {
        json!({"exit":step.exit})
    };
    execution::assess_exit(&mut actual, step.exit)?;
    if let Some(command) = &step.mcp {
        let directory = out.join(format!("{ordinal}-mcp"));
        fs::create_dir(&directory).map_err(|e| e.to_string())?;
        let reply = super::super::mcp::call_retained(
            context.candidate,
            command,
            step.is_error,
            &globals,
            Some(&directory),
        )?;
        if step.run.is_none() {
            execution::field(&mut actual, "parsed", reply.clone())?;
        }
        execution::field(
            &mut actual,
            "mcp",
            json!({"parsed":reply,"raw_directory":directory}),
        )?;
    }
    Ok(actual)
}
fn aggregate(exchanges: &[Value], pointer: Option<&str>) -> Result<Value, String> {
    let mut result = exchanges
        .first()
        .cloned()
        .unwrap_or(json!({"assessment_error":"no command executed"}));
    if let Some(failed) = exchanges.iter().find_map(|e| e.get("assessment_error")) {
        execution::field(&mut result, "assessment_error", failed.clone())?;
    }
    if let Some(pointer) = pointer {
        let mut items = Vec::new();
        for exchange in exchanges {
            let Some(page) = exchange
                .get("parsed")
                .and_then(|value| value.pointer(pointer))
                .and_then(Value::as_array)
            else {
                execution::field(
                    &mut result,
                    "assessment_error",
                    json!("released walk array missing"),
                )?;
                return Ok(result);
            };
            items.extend(page.iter().cloned());
        }
        *result
            .get_mut("parsed")
            .and_then(|value| value.pointer_mut(pointer))
            .ok_or("released aggregate pointer missing")? = json!(items);
    }
    execution::field(&mut result, "exchanges", json!(exchanges))?;
    Ok(result)
}
