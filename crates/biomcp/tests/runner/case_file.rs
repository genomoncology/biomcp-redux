//! Load one command case and preserve its owning output-budget boundary.
use super::{Case, launch, load_floors, run_sequence, schema};
use serde_json::Value;
use std::{fs, path::Path};

pub(super) fn run_case(path: &Path, registry: &schema::Registry) -> Result<Value, String> {
    run_budgeted_case(path, registry, false)
}

// Only generated phenotype parity cases have an independently checked fixed ceiling.
pub(super) fn run_phenotype_native_case(
    path: &Path,
    registry: &schema::Registry,
) -> Result<Value, String> {
    run_budgeted_case(path, registry, true)
}

fn run_budgeted_case(
    path: &Path,
    registry: &schema::Registry,
    phenotype_native_budget: bool,
) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut case: Case = serde_saphyr::from_str(&text).map_err(|error| error.to_string())?;
    if case.story.trim().is_empty() || case.steps.is_empty() {
        return Err("case requires a story and at least one step".into());
    }
    load_floors(&mut case, path)?;
    for step in &mut case.steps {
        step.phenotype_native_budget = phenotype_native_budget;
    }
    let expectations = case
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            if step.run.is_none() && step.mcp.is_none() && step.follow.is_none() {
                return Err("step requires run or mcp".into());
            }
            assertions::prepare(step)
                .map_err(|error| format!("{}; step {}: {error}", case.story, index + 1))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let environment = launch::Environment::new().map_err(|error| error.to_string())?;
    if let Some(config) = &case.config {
        environment
            .config(config)
            .map_err(|error| error.to_string())?;
    }
    let replay = case
        .replay
        .as_ref()
        .map(|name| launch::fixtures().join("replies").join(name));
    let mut previous = None;
    for (index, (step, expectations)) in case.steps.iter().zip(&expectations).enumerate() {
        previous = Some(
            run_sequence(
                step,
                &environment,
                expectations,
                registry,
                replay.as_deref(),
                previous.as_ref(),
            )
            .map_err(|error| format!("{}; step {}: {error}", case.story, index + 1))?,
        );
    }
    previous.ok_or_else(|| "case requires at least one step".into())
}
