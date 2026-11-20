//! Every recorded get or subcommand case whose rows take `--fields` gives, with one chosen
//! name, a reply that still validates against its schema and holds the same rows in the same
//! order, each a part of its whole row (1071).
use super::{case_data, launch::Environment, response_cases};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn trial() -> libtest_mimic::Trial {
    super::transcript::trial("section-fields/sections/narrowed-rows-validate", || {
        sweep().map_err(Into::into)
    })
}

/// Fewer distinct calls than this means the sweep lost its cases, not that all passed.
const LEAST: usize = 20;

fn sweep() -> Result<(), String> {
    let root = super::launch::fixtures().join("cases");
    let mut paths = Vec::new();
    case_data::collect(&root, &mut paths).map_err(|error| error.to_string())?;
    paths.sort();
    let mut seen = BTreeSet::new();
    let mut narrowed = BTreeSet::new();
    for path in paths {
        let Ok(case) = case_data::read_case(&path) else {
            continue;
        };
        if case.replay.is_none() {
            continue;
        }
        for args in case.steps.iter().filter_map(candidate) {
            if !seen.insert(shape(args)) {
                continue;
            }
            let name = format!("{}: {}", path.display(), args.join(" "));
            if check(&case, args).map_err(|error| format!("{name}: {error}"))? {
                narrowed.insert(shape(args));
            }
        }
    }
    if narrowed.len() < LEAST {
        return Err(format!(
            "only {} calls narrowed: {narrowed:?}",
            narrowed.len()
        ));
    }
    Ok(())
}

/// A recorded call that succeeds and names a get or a subcommand, not a search.
fn candidate(step: &case_data::Step) -> Option<&Vec<String>> {
    let args = step.run.as_ref()?;
    let verb = args.first()?;
    (step.exit == 0
        && !step.is_error
        && !step.offline
        && step.walk.is_none()
        && step.follow.is_none()
        && !verb.starts_with('-')
        && args.get(1).is_none_or(|entity| entity != "search")
        && !matches!(
            verb.as_str(),
            "search" | "help" | "schema" | "sync" | "skill" | "saved"
        )
        && !args
            .iter()
            .any(|arg| arg.starts_with("--fields") || arg.starts_with("--format")))
    .then_some(args)
}

/// The command words and section words of a call, without its identifiers, options and values.
fn shape(args: &[String]) -> Vec<String> {
    let mut shape = args.iter().take(2).cloned().collect::<Vec<_>>();
    let mut value = false;
    for arg in args.iter().skip(3) {
        if arg.starts_with("--") {
            value = !arg.contains('=');
        } else if !std::mem::take(&mut value)
            && arg.chars().all(|c| c.is_ascii_lowercase() || c == '-')
        {
            shape.push(arg.clone());
        }
    }
    shape
}

fn run(
    env: &Environment,
    args: &[String],
    globals: &[String],
) -> Result<(Option<i32>, Value), String> {
    let mut all = args.to_vec();
    all.extend_from_slice(globals);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &all)
        .output()
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    Ok((output.status.code(), value))
}

/// Run a call whole and with its first offered field; true when the call has rows.
fn check(case: &case_data::Case, args: &[String]) -> Result<bool, String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    env.case_config(case).map_err(|error| error.to_string())?;
    let replay = response_cases::replay(&env, case.replay.as_deref(), case.cache_acquired)?;
    let globals = env.globals(replay.as_deref(), false);
    let asked = [&["--fields".to_owned(), "1071-none".to_owned()], args].concat();
    let (_, refusal) = run(&env, &asked, &globals)?;
    let Some(name) = refusal
        .pointer("/error/valid_values/0")
        .and_then(Value::as_str)
    else {
        return Ok(false);
    };
    let (code, whole) = run(&env, args, &globals)?;
    if !matches!(
        whole.get("status").and_then(Value::as_str),
        Some("complete" | "partial")
    ) {
        return Ok(false);
    }
    let asked = [&["--fields".to_owned(), name.to_owned()], args].concat();
    let (narrow_code, narrow) = run(&env, &asked, &globals)?;
    if narrow_code != code {
        return Err(format!("exit {narrow_code:?}, whole call {code:?}"));
    }
    super::schema::Registry::embedded()?.validate(&narrow, args, true)?;
    if narrow.get("fields") != Some(&Value::from(vec![name])) {
        return Err(format!(
            "reply fields {:?}, asked {name}",
            narrow.get("fields")
        ));
    }
    let (part, full) = (&narrow["result"], &whole["result"]);
    if !within(part, full) {
        return Err("narrowed rows are not the whole rows, in order, with keys left out".into());
    }
    Ok(true)
}

/// Values a second run may stamp anew.
const STAMPS: [&str; 1] = ["fetched"];

/// Whether every object of `part` keeps only keys of the same object of `full`, with every
/// list the same length in the same order.
fn within(part: &Value, full: &Value) -> bool {
    match (part, full) {
        (Value::Object(part), Value::Object(full)) => part.iter().all(|(key, value)| {
            full.get(key)
                .is_some_and(|whole| STAMPS.contains(&key.as_str()) || within(value, whole))
        }),
        (Value::Array(part), Value::Array(full)) => {
            part.len() == full.len() && part.iter().zip(full).all(|(part, full)| within(part, full))
        }
        _ => part == full,
    }
}
