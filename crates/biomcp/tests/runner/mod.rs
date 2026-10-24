mod article_save;
mod assertions;
mod configuration;
mod corporate;
mod evidence;
mod gene_tables;
mod held_proxy;
mod help_examples;
mod http;
mod http_limits;
mod key_registration;
mod key_reload;
mod launch;
mod litvar_tables;
pub(crate) mod live;
mod live_follow_proof;
mod live_proof;
mod markdown;
mod markdown_shapes;
mod markdown_sources;
mod mcp;
mod named_search;
mod nci;
mod nci_walk;
mod negative_cases;
mod network_stub;
mod normalization;
mod pdf;
mod pmc_tables;
mod pool_tables;
mod process;
pub(super) mod proof;
mod protocol;
mod publication;
mod recurrence;
mod recurrence_errors;
mod recurrence_metadata;
mod response_cases;
mod schema;
mod schema_proof;
mod scholar;
mod semantic_tables;
mod source_controls;
mod supplement_tables;
mod trial_documents;
mod trial_get;
mod trial_sites;
mod trial_tables;
mod variant;
mod variant_search;

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
    live_check: Option<String>,
    #[serde(rename = "feature", default)]
    _feature: Option<String>,
    #[serde(rename = "covers", default)]
    _covers: Vec<String>,
    story: String,
    replay: Option<String>,
    config: Option<Value>,
    #[serde(rename = "harvest", default)]
    _harvest: Option<String>,
    #[serde(rename = "verified", default)]
    _verified: Option<String>,
    steps: Vec<Step>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Step {
    run: Option<Vec<String>>,
    follow: Option<String>,
    walk: Option<Walk>,
    #[serde(default)]
    offline: bool,
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

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Walk {
    items: String,
    max_pages: u32,
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
                run_case(&path, &registry).map(|_| ()).map_err(Into::into)
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

fn run_case(path: &Path, registry: &schema::Registry) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut case: Case = serde_saphyr::from_str(&text).map_err(|error| error.to_string())?;
    if case.story.trim().is_empty() || case.steps.is_empty() {
        return Err("case requires a story and at least one step".into());
    }
    load_floors(&mut case, path)?;
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

fn run_step(
    step: &Step,
    environment: &launch::Environment,
    expectations: &assertions::Expectations<'_>,
    registry: &schema::Registry,
    replay: Option<&std::path::Path>,
) -> Result<Value, String> {
    let globals = environment.globals(replay, step.offline);
    let cli = step
        .run
        .as_ref()
        .map(|args| {
            let mut command_args = args.clone();
            let position = command_args
                .iter()
                .position(|arg| arg == "--")
                .unwrap_or(command_args.len());
            command_args.splice(position..position, globals.iter().cloned());
            let output = environment
                .command(env!("CARGO_BIN_EXE_biomcp"), &command_args)
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
    let mut answer = cli.clone();
    if let Some(args) = &step.mcp {
        let result = mcp::call_with_globals(environment, args, step.is_error, &globals)?;
        registry.validate(&result, args, !step.is_error)?;
        assertions::check(
            &serde_json::to_vec(&result).map_err(|error| error.to_string())?,
            step,
            expectations,
        )?;
        answer = Some(result.clone());
        if cli.as_ref().is_some_and(|cli| cli != &result) {
            return Err("CLI/MCP parity: unequal results".into());
        }
    }
    answer.ok_or_else(|| "step requires run or mcp".into())
}

pub(super) fn protocol_trials() -> Vec<Trial> {
    let mut trials = protocol::trials();
    trials.push(Trial::test("article-help/executable-examples", || {
        help_examples::check().map_err(Into::into)
    }));
    trials.push(Trial::test(
        "article-supplements/ping-after-bounded-refusal",
        || supplement_tables::ping_after_refusal().map_err(Into::into),
    ));
    trials.push(Trial::test("article-saves/explicit-original-files", || {
        article_save::check().map_err(Into::into)
    }));
    trials.extend(live_proof::trials());
    trials.extend(live_follow_proof::trials());
    trials.extend(gene_tables::trials());
    trials.extend(litvar_tables::trials());
    trials.extend(named_search::trials());
    trials.extend(trial_tables::trials());
    trials.extend(trial_get::trials());
    trials.extend(nci::trials());
    trials.extend(nci_walk::trials());
    trials.extend(trial_documents::trials());
    trials.extend(trial_sites::trials());
    trials.push(Trial::test("article-fulltext/source-boundaries", || {
        pmc_tables::check().map_err(|e| e.to_string().into())
    }));
    trials.push(Trial::test("article-search/snapshot-replacement", || {
        pool_tables::check().map_err(Into::into)
    }));
    trials.extend(configuration::trials());
    trials.extend(key_registration::trials());
    trials.extend(scholar::trials());
    trials.extend(evidence::trials());
    trials.extend(pdf::trials());
    trials.extend(source_controls::trials());

    trials.extend(http::trials());
    trials.extend(corporate::trials());
    trials.extend(variant::trials());
    trials.extend(recurrence::trials());
    trials.extend(recurrence_errors::trials());
    trials.extend(recurrence_metadata::trials());
    trials.extend(normalization::trials());
