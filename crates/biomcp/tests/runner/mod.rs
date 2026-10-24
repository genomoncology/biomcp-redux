mod assertions;
mod launch;
mod mcp;
mod negative_cases;
mod process;
pub(super) mod proof;
mod protocol;
mod schema;
mod schema_proof;

use libtest_mimic::Trial;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path, sync::Arc};

// Maintained serde YAML parser; only deserialization is needed. Its typed parser
// rejects unknown fields so a misspelled assertion cannot silently pass.
// https://docs.rs/serde-saphyr/latest/serde_saphyr/#using-serializer-or-deserializer-specifically
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    #[serde(rename = "feature", default)]
    _feature: Option<String>,
    #[serde(rename = "covers", default)]
    _covers: Vec<String>,
    story: String,
    #[serde(rename = "harvest", default)]
    _harvest: Option<String>,
    #[serde(rename = "verified", default)]
    _verified: Option<String>,
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Step {
    run: Option<Vec<String>>,
    mcp: Option<Vec<String>>,
    #[serde(default)]
    is_error: bool,
    #[serde(default)]
    exit: i32,
    max_bytes: Option<usize>,
    max_tokens: Option<usize>,
    #[serde(default)]
    expect: BTreeMap<String, Value>,
}

pub(super) fn discover(root: &Path, include_proof: bool) -> Result<Vec<Trial>, String> {
    let mut paths = Vec::new();
    collect(root, &mut paths).map_err(|error| format!("{}: {error}", root.display()))?;
    paths.sort();
    if paths.is_empty() {
        return Err(format!("{}: no YAML cases", root.display()));
    }
    let registry = Arc::new(schema::Registry::embedded()?);
    let mut trials = paths
        .into_iter()
        .map(|path| {
            let name = path
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .iter()
                .map(|part| part.to_str().ok_or("case path must be UTF-8"))
                .collect::<Result<Vec<_>, _>>()?
                .join("/");
            let registry = Arc::clone(&registry);
            Ok(Trial::test(name, move || {
                run_case(&path, &registry).map_err(Into::into)
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if include_proof {
        trials.extend(schema_proof::trials(registry));
    }
    Ok(trials)
}

fn collect(root: &Path, paths: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect(&path, paths)?;
        } else if path
            .extension()
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            paths.push(path);
        }
    }
    Ok(())
}

fn run_case(path: &Path, registry: &schema::Registry) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let case: Case = serde_saphyr::from_str(&text).map_err(|error| error.to_string())?;
    if case.story.trim().is_empty() || case.steps.is_empty() {
        return Err("case requires a story and at least one step".into());
    }
    let expectations = case
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            if step.run.is_none() && step.mcp.is_none() {
                return Err("step requires run or mcp".into());
            }
            assertions::prepare(step)
                .map_err(|error| format!("{}; step {}: {error}", case.story, index + 1))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let environment = launch::Environment::new().map_err(|error| error.to_string())?;
    for (index, (step, expectations)) in case.steps.iter().zip(&expectations).enumerate() {
        run_step(step, &environment, expectations, registry)
            .map_err(|error| format!("{}; step {}: {error}", case.story, index + 1))?;
    }
    Ok(())
}

fn run_step(
    step: &Step,
    environment: &launch::Environment,
    expectations: &assertions::Expectations<'_>,
    registry: &schema::Registry,
) -> Result<(), String> {
    let cli = step
        .run
        .as_ref()
        .map(|args| {
            let output = environment
                .command(env!("CARGO_BIN_EXE_biomcp"), args)
                .output()
                .map_err(|error| error.to_string())?;
            if output.status.code() != Some(step.exit) {
                return Err(format!(
                    "exit: expected {}, actual {:?}; stderr: {}",
                    step.exit,
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            if !output.stderr.is_empty() {
                return Err("error output: expected empty stderr".into());
            }
            let value = serde_json::from_slice::<Value>(&output.stdout)
                .map_err(|error| error.to_string())?;
            registry.validate(&value, args, output.status.success())?;
            assertions::check(&output.stdout, step, expectations)?;
            Ok(value)
        })
        .transpose()?;
    if let Some(args) = &step.mcp {
        let result = mcp::call(environment, args, step.is_error)?;
        registry.validate(&result, args, !step.is_error)?;
        assertions::check(
            &serde_json::to_vec(&result).map_err(|error| error.to_string())?,
            step,
            expectations,
        )?;
        if cli.as_ref().is_some_and(|cli| cli != &result) {
            return Err("CLI/MCP parity: unequal results".into());
        }
    }
    Ok(())
}

pub(super) fn protocol_trials() -> Vec<Trial> {
    protocol::trials()
}
