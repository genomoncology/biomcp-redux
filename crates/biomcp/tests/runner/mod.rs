mod assertions;
mod configuration;
mod corporate;
mod gene_tables;
mod held_proxy;
mod launch;
pub(crate) mod live;
mod live_proof;
mod markdown;
mod markdown_sources;
mod mcp;
mod negative_cases;
mod network_stub;
mod pmc_tables;
mod pool_tables;
mod process;
pub(super) mod proof;
mod protocol;
mod publication;
mod response_cases;
mod schema;
mod schema_proof;
mod scholar;

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
    trials.extend(live_proof::trials());
    trials.extend(gene_tables::trials());
    trials.push(Trial::test("article-fulltext/source-boundaries", || {
        pmc_tables::check().map_err(|e| e.to_string().into())
    }));
    trials.push(Trial::test("article-search/snapshot-replacement", || {
        pool_tables::check().map_err(Into::into)
    }));
    trials.extend(configuration::trials());
    trials.extend(scholar::trials());
    trials.extend(corporate::trials());
    trials.extend(markdown::trials());
    trials.extend(markdown_sources::trials());
    trials.push(Trial::test("mcp/ping-passes-held-provider-call", || {
        held_proxy::check().map_err(Into::into)
    }));
    trials
}

fn load_floors(case: &mut Case, path: &Path) -> Result<(), String> {
    for step in &mut case.steps {
        for expected in step.expect.values_mut().filter_map(Value::as_object_mut) {
            for (key, value) in expected.iter_mut() {
                if !matches!(key.as_str(), "includes_all_of" | "equals_file") {
                    continue;
                }
                let Some(file) = value.as_str() else {
                    continue;
                };
                if !Path::new(file)
                    .components()
                    .all(|p| matches!(p, std::path::Component::Normal(_)))
                {
                    return Err("floor must be beside its case".into());
                }
                let floor = path.parent().ok_or("case has no parent")?.join(file);
                *value = serde_json::from_slice(&fs::read(floor).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}
fn action(previous: &Value, kind: &str) -> Result<Vec<String>, String> {
    let steps = previous
        .get("next")
        .and_then(Value::as_array)
        .ok_or("follow action absent")?;
    let action = steps
        .iter()
        .find(|s| s.get("kind").and_then(Value::as_str) == Some(kind))
        .ok_or("follow action absent")?;
    action
        .get("args")
        .and_then(Value::as_array)
        .ok_or("follow requires typed arguments")?
        .iter()
        .map(|a| {
            a.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "follow requires typed arguments".into())
        })
        .collect()
}
fn run_sequence(
    step: &Step,
    environment: &launch::Environment,
    expectations: &assertions::Expectations<'_>,
    registry: &schema::Registry,
    replay: Option<&Path>,
    previous: Option<&Value>,
) -> Result<Value, String> {
    let mut current = step.clone();
    if let Some(kind) = &step.follow {
        current.run = Some(action(
            previous.ok_or("follow requires previous output")?,
            kind,
        )?);
        current.mcp = None;
    }
    let Some(walk) = &step.walk else {
        return run_step(&current, environment, expectations, registry, replay);
    };
    if walk.max_pages == 0 || walk.max_pages > 1000 {
        return Err("walk requires a bounded page count".into());
    }
    let mut actions = std::collections::BTreeSet::new();
    let mut offsets = std::collections::BTreeSet::new();
    let mut items = Vec::new();
    let mut aggregate = None;
    for _ in 0..walk.max_pages {
        if !actions.insert(current.run.clone().ok_or("walk requires run")?) {
            return Err("walk repeated action".into());
        }
        let value = run_step(&current, environment, &Vec::new(), registry, replay)?;
        let offset = value
            .pointer("/page/offset")
            .and_then(Value::as_u64)
            .ok_or("walk requires page offset")?;
        if !offsets.insert(offset) {
            return Err("walk repeated offset".into());
        }
        items.extend(
            value
                .pointer(&walk.items)
                .and_then(Value::as_array)
                .ok_or("walk items must select an array")?
                .iter()
                .cloned(),
        );
        if aggregate.is_none() {
            aggregate = Some(value.clone());
        }
        let next = action(&value, "page");
        if let Ok(args) = next {
            current.run = Some(args);
            current.mcp = None;
        } else {
            let mut result = aggregate.ok_or("walk empty")?;
            *result
                .pointer_mut(&walk.items)
                .ok_or("walk missing aggregate pointer")? = Value::Array(items);
            let next = value
                .get("next")
                .cloned()
                .unwrap_or_else(|| Value::Array(Vec::new()));
            result
                .as_object_mut()
                .ok_or("walk result must be object")?
                .insert("next".into(), next);
            let mut checked = step.clone();
            checked.max_bytes = None;
            checked.max_tokens = None;
            assertions::check(
                &serde_json::to_vec(&result).map_err(|e| e.to_string())?,
                &checked,
                expectations,
            )?;
            return Ok(result);
        }
    }
    Err("walk exceeded max_pages".into())
}

pub(super) fn installed_process_proof() -> Result<(), String> {
    configuration::article_processes()
}
