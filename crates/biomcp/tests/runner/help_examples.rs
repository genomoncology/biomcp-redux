//! Verify installed help examples against their owning replay case steps.
use super::{Case, assertions, launch, run_step, schema};
use serde_json::Value;
use std::{collections::BTreeSet, fs};

pub(super) fn check() -> Result<(), String> {
    let environment = launch::Environment::new().map_err(|e| e.to_string())?;
    let output = environment.command(env!("CARGO_BIN_EXE_biomcp"), &["help".into()])
        .output().map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err("help process failed".into());
    }
    let help: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let commands = help.pointer("/result/commands").and_then(Value::as_array)
        .ok_or("help commands missing")?;
    let mut printed = BTreeSet::new();
    for command in commands.iter().filter(|c| c["command"].as_array().is_some_and(|words| words.len() == 2 && words.iter().any(|v| v == "article" || v == "gene"))) {
        let examples = command["examples"].as_array().filter(|v| v.len() == 2)
            .ok_or("query help requires two examples")?;
        for example in examples {
            let args: Vec<String> = serde_json::from_value(example.clone()).map_err(|e| e.to_string())?;
            if !printed.insert(args) {
                return Err("help repeats an example".into());
            }
        }
    }
    let witnesses: Vec<(String, usize)> = serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/example-cases.json")))
        .map_err(|e| e.to_string())?;
    let registry = schema::Registry::embedded()?;
    for (file, index) in witnesses {
        let text = fs::read_to_string(launch::fixtures().join("cases").join(file)).map_err(|e| e.to_string())?;
        let case: Case = serde_saphyr::from_str(&text).map_err(|e| e.to_string())?;
        let step = case.steps.get(index).ok_or("example step missing")?;
        let args = step.run.as_ref().ok_or("example must be a run step")?;
        if step.exit != 0 || !printed.remove(args) {
            return Err("example differs from successful case run step".into());
        }
        let environment = launch::Environment::new().map_err(|e| e.to_string())?;
        if let Some(config) = &case.config {
            environment.config(config).map_err(|e| e.to_string())?;
        }
        let replay = case.replay.as_ref().map(|name| launch::fixtures().join("replies").join(name));
        let mut step = step.clone();
        step.mcp = step.run.clone();
        run_step(&step, &environment, &assertions::prepare(&step)?, &registry, replay.as_deref())?;
    }
    if !printed.is_empty() {
        return Err("printed example has no owning case witness".into());
    }
    Ok(())
}
