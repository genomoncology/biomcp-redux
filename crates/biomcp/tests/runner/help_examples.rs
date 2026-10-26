//! Verify installed help examples against their owning replay case steps.
use super::{Case, assertions, launch, run_sequence, schema};
use serde_json::Value;
use std::{collections::BTreeSet, fs};

pub(super) fn check() -> Result<(), String> {
    let environment = launch::Environment::new().map_err(|e| e.to_string())?;
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &["help".into()])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err("help process failed".into());
    }
    let help: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let commands = help
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("help commands missing")?;
    source_description(commands)?;
    let mut printed = collected_examples(commands)?;
    let witnesses: Vec<(String, usize)> = serde_json::from_str(include_str!(concat!(
        env!("OUT_DIR"),
        "/example-cases.json"
    )))
    .map_err(|e| e.to_string())?;
    let registry = schema::Registry::embedded()?;
    for (file, index) in witnesses {
        let path = launch::fixtures().join("cases").join(file);
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut case: Case = serde_saphyr::from_str(&text).map_err(|e| e.to_string())?;
        super::load_floors(&mut case, &path)?;
        let step = case.steps.get(index).ok_or("example step missing")?;
        let args = step.run.as_ref().ok_or("example must be a run step")?;
        if step.exit != 0 || !printed.remove(args) {
            return Err("example differs from successful case run step".into());
        }
        let environment = launch::Environment::new().map_err(|e| e.to_string())?;
        if let Some(config) = &case.config {
            environment.config(config).map_err(|e| e.to_string())?;
        }
        let replay = case
            .replay
            .as_ref()
            .map(|name| launch::fixtures().join("replies").join(name));
        prepare_bundles(&case, index, &environment, &registry, replay.as_deref())?;
        for step in example_steps(step)? {
            run_sequence(
                &step,
                &environment,
                &assertions::prepare(&step)?,
                &registry,
                replay.as_deref(),
                None,
            )?;
        }
    }
    if !printed.is_empty() {
        return Err("printed example has no owning case witness".into());
    }
    Ok(())
}

fn collected_examples(commands: &[Value]) -> Result<BTreeSet<Vec<String>>, String> {
    let mut printed = BTreeSet::new();
    for examples in commands
        .iter()
        .filter_map(|command| command.get("examples").and_then(Value::as_array))
        .filter(|examples| !examples.is_empty())
    {
        if examples.len() != 2 {
            return Err("query help requires two examples".into());
        }
        for example in examples {
            let args: Vec<String> =
                serde_json::from_value(example.clone()).map_err(|e| e.to_string())?;
            if !printed.insert(args) {
                return Err("help repeats an example".into());
            }
        }
    }
    Ok(printed)
}

fn source_description(commands: &[Value]) -> Result<(), String> {
    let arguments = commands
        .iter()
        .find(|c| c["command"] == serde_json::json!(["search", "article"]))
        .and_then(|c| c["arguments"].as_array())
        .ok_or("article search help missing")?;
    for name in ["--since", "--until"] {
        let description = arguments
            .iter()
            .find(|v| v["name"] == name)
            .and_then(|v| v["description"].as_str())
            .ok_or("date help missing")?;
        if !description.contains("either electronic or print date") {
            return Err(format!("{name} help omits PubMed's two dates"));
        }
    }
    let description = arguments
        .iter()
        .find(|v| v["name"] == "--source")
        .and_then(|v| v["description"].as_str())
        .ok_or("source description missing")?;
    for word in [
        "pubmed:",
        "dropped words",
        "europepmc:",
        "incidental mentions",
        "litvar:",
        "--variant",
        "candidates",
        "semantic-scholar:",
        "429",
        "optional",
        "litsense:",
        "resemblance, not a match",
        "timeouts",
        "pubtator:",
        "HGNC Entrez",
    ] {
        if !description.contains(word) {
            return Err(format!("source help omits job or failure mode: {word}"));
        }
    }
    Ok(())
}

// Stored-source examples need their owning case's explicit synchronization.
fn prepare_bundles(
    case: &Case,
    index: usize,
    environment: &launch::Environment,
    registry: &schema::Registry,
    replay: Option<&std::path::Path>,
) -> Result<(), String> {
    for step in case.steps.iter().take(index).filter(|step| {
        step.run
            .as_ref()
            .is_some_and(|args| args.first().is_some_and(|verb| verb == "sync"))
    }) {
        run_sequence(
            step,
            environment,
            &assertions::prepare(step)?,
            registry,
            replay,
            None,
        )?;
    }
    Ok(())
}

// Saves have independent destinations; replacement remains a CLI example.
fn example_steps(original: &super::Step) -> Result<Vec<super::Step>, String> {
    let mut step = original.clone();
    step.mcp = step.run.clone();
    let args = step.mcp.as_mut().ok_or("example arguments")?;
    let Some(save) = args.iter().position(|arg| arg == "--save") else {
        return Ok(vec![step]);
    };
    let path = args.get_mut(save + 1).ok_or("save destination")?;
    *path = format!("stdio-{path}");
    args.retain(|arg| arg != "--replace");
    let mut cli = original.clone();
    cli.mcp = None;
    step.run = None;
    Ok(vec![cli, step])
}
