mod assertions;
mod launch;
mod negative_cases;
pub(super) mod proof;

use libtest_mimic::Trial;
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

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
    run: Vec<String>,
    #[serde(default)]
    exit: i32,
    max_bytes: Option<usize>,
    max_tokens: Option<usize>,
    #[serde(default)]
    expect: BTreeMap<String, Value>,
}

pub(super) fn discover(root: &Path) -> Result<Vec<Trial>, String> {
    let mut paths = Vec::new();
    collect(root, &mut paths).map_err(|error| format!("{}: {error}", root.display()))?;
    paths.sort();
    if paths.is_empty() {
        return Err(format!("{}: no YAML cases", root.display()));
    }
    paths
        .into_iter()
        .map(|path| {
            let name = path
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .iter()
                .map(|part| part.to_str().ok_or("case path must be UTF-8"))
                .collect::<Result<Vec<_>, _>>()?
                .join("/");
            Ok(Trial::test(name, move || {
                run_case(&path).map_err(Into::into)
            }))
        })
        .collect()
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

fn run_case(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let case: Case = serde_saphyr::from_str(&text).map_err(|error| error.to_string())?;
    if case.story.trim().is_empty() || case.steps.is_empty() {
        return Err("case requires a story and at least one step".into());
    }
    let environment = launch::Environment::new().map_err(|error| error.to_string())?;
    for (index, step) in case.steps.iter().enumerate() {
        run_step(step, &environment)
            .map_err(|error| format!("{}; step {}: {error}", case.story, index + 1))?;
    }
    Ok(())
}

fn run_step(step: &Step, environment: &launch::Environment) -> Result<(), String> {
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &step.run)
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
    if step.exit != 0
        && (!output.stdout.is_empty()
            || output.stderr != b"biomcp: expected schema <known artifact id>\n")
    {
        return Err("error output: expected empty stdout and fixed diagnostic".into());
    }
    assertions::check(&output.stdout, step)
}
