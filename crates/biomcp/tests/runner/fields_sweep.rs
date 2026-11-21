//! Every recorded get or subcommand case whose rows take `--fields` gives, with one chosen
//! name that is not always kept, a smaller reply that still validates against its schema and
//! holds the same rows in the same order, each a part of its whole row; a call naming several
//! sections, narrowed by a name only one of them knows, leaves the others exactly whole (1071).
use super::{case_data, launch::Environment, response_cases};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn trial() -> libtest_mimic::Trial {
    super::transcript::trial("section-fields/sections/narrowed-rows-validate", || {
        sweep().map_err(Into::into)
    })
}

pub(super) fn additional_trial() -> libtest_mimic::Trial {
    super::transcript::trial("section-fields/sections/additional-row-shapes", || {
        additional_sections().map_err(Into::into)
    })
}

/// Fewer distinct calls than this means the sweep lost its cases, not that all passed.
const LEAST: usize = 20;
/// The several-section calls whose untouched sections the sweep proves exactly whole.
const LEAST_SPLITS: usize = 2;

fn sweep() -> Result<(), String> {
    let root = super::launch::fixtures().join("cases");
    let mut paths = Vec::new();
    case_data::collect(&root, &mut paths).map_err(|error| error.to_string())?;
    paths.sort();
    let mut seen = BTreeSet::new();
    let mut narrowed = BTreeSet::new();
    let mut splits = BTreeSet::new();
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
            let (smaller, split) =
                check(&case, args).map_err(|error| format!("{name}: {error}"))?;
            if smaller {
                narrowed.insert(shape(args));
            }
            if split {
                splits.insert(shape(args));
            }
        }
    }
    if narrowed.len() < LEAST {
        return Err(format!(
            "only {} calls narrowed: {narrowed:?}",
            narrowed.len()
        ));
    }
    if splits.len() < LEAST_SPLITS {
        return Err(format!(
            "only {} several-section calls split: {splits:?}",
            splits.len()
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

/// One recorded call run inside its case's replay.
struct Probe {
    env: Environment,
    globals: Vec<String>,
}

impl Probe {
    fn new(case: &case_data::Case) -> Result<Self, String> {
        let env = Environment::new().map_err(|error| error.to_string())?;
        env.case_config(case).map_err(|error| error.to_string())?;
        let replay = response_cases::replay(&env, case.replay.as_deref(), case.cache_acquired)?;
        let globals = env.globals(replay.as_deref(), false);
        Ok(Self { env, globals })
    }

    fn run(&self, fields: Option<&str>, args: &[String]) -> Result<(Option<i32>, Value), String> {
        let mut asked = fields.map_or_else(Vec::new, |fields| {
            vec!["--fields".to_owned(), fields.to_owned()]
        });
        asked.extend_from_slice(args);
        run(&self.env, &asked, &self.globals)
    }

    /// The row names a call offers, from its refusal of a name no row has.
    fn names(&self, args: &[String]) -> Result<Vec<String>, String> {
        let (_, refusal) = self.run(Some("1071-none"), args)?;
        Ok(refusal
            .pointer("/error/valid_values")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect())
    }
}

/// How many offered names the sweep tries before it finds one that is not always kept.
const TRIES: usize = 4;

/// Run a call whole and with an offered name that makes its rows smaller, and split a call
/// naming several sections; whether its rows got smaller, and whether it split.
fn check(case: &case_data::Case, args: &[String]) -> Result<(bool, bool), String> {
    let probe = Probe::new(case)?;
    let names = probe.names(args)?;
    if names.is_empty() {
        return Ok((false, false));
    }
    let (code, whole) = probe.run(None, args)?;
    if !matches!(
        whole.get("status").and_then(Value::as_str),
        Some("complete" | "partial")
    ) {
        return Ok((false, false));
    }
    let mut smaller = false;
    for name in names.iter().take(TRIES) {
        let narrow = narrowed(&probe, args, name, code, &whole)?;
        if size(narrow.get("result")) < size(whole.get("result")) {
            smaller = true;
            break;
        }
    }
    let split = section_words(args).len() > 1 && split(&probe, args, &whole, code)?;
    Ok((smaller, split))
}

/// The narrowed reply, checked: same exit, valid schema, the name echoed, the same rows.
fn narrowed(
    probe: &Probe,
    args: &[String],
    name: &str,
    code: Option<i32>,
    whole: &Value,
) -> Result<Value, String> {
    let (narrow_code, narrow) = probe.run(Some(name), args)?;
    if narrow_code != code {
        return Err(format!(
            "--fields {name}: exit {narrow_code:?}, whole call {code:?}"
        ));
    }
    super::schema::Registry::embedded()?.validate(&narrow, args, true)?;
    if narrow.get("fields") != Some(&Value::from(vec![name])) {
        return Err(format!(
            "reply fields {:?}, asked {name}",
            narrow.get("fields")
        ));
    }
    if !within(narrow.get("result"), whole.get("result")) {
        return Err(format!(
            "--fields {name}: narrowed rows are not the whole rows, in order, with keys left out"
        ));
    }
    carried(probe, &narrow)?;
    Ok(narrow)
}

/// Every next step that carries `--fields` is one its target takes, never a usage refusal.
fn carried(probe: &Probe, reply: &Value) -> Result<(), String> {
    let steps = reply
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|step| step.get("args").and_then(Value::as_array))
        .map(|args| {
            args.iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .filter(|args| args.iter().any(|arg| arg.starts_with("--fields")));
    for args in steps {
        let (_, value) = probe.run(None, &args)?;
        let code = value
            .pointer("/error/code")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if code.starts_with("usage") {
            return Err(format!("next step {} is refused: {code}", args.join(" ")));
        }
    }
    Ok(())
}

fn size(value: Option<&Value>) -> usize {
    value.map_or(0, |value| value.to_string().len())
}

/// The section words of a call: lowercase words after its identifier that are not option values.
fn section_words(args: &[String]) -> Vec<String> {
    shape(args).into_iter().skip(2).collect()
}

/// The same call naming only this one of its sections.
fn alone(args: &[String], word: &str) -> Vec<String> {
    let others = section_words(args);
    let mut value = false;
    let mut kept = Vec::new();
    for (index, arg) in args.iter().enumerate() {
        let option_value = std::mem::take(&mut value);
        if arg.starts_with("--") {
            value = !arg.contains('=');
        }
        if index < 3 || option_value || arg == word || !others.contains(arg) {
            kept.push(arg.clone());
        }
    }
    kept
}

/// For a call naming several sections: a name or path only one section knows narrows that
/// section and leaves every other section exactly whole (1071 review: a path's first name
/// alone must not narrow a section that lacks the path).
fn split(probe: &Probe, args: &[String], whole: &Value, code: Option<i32>) -> Result<bool, String> {
    let words = section_words(args);
    let mut checked = false;
    for word in &words {
        let alone_args = alone(args, word);
        let mut candidates = paths(whole, word);
        candidates.extend(probe.names(&alone_args)?.into_iter().take(TRIES));
        for name in &candidates {
            if checked && !name.contains('.') {
                continue;
            }
            let (alone_code, _) = probe.run(Some(name), &alone_args)?;
            if alone_code != Some(0) {
                continue;
            }
            let mut shared = false;
            for other in words.iter().filter(|other| *other != word) {
                let (other_code, _) = probe.run(Some(name), &alone(args, other))?;
                shared |= other_code == Some(0);
            }
            if shared {
                continue;
            }
            let narrow = narrowed(probe, args, name, code, whole)?;
            for other in words.iter().filter(|other| *other != word) {
                let key = other.replace('-', "_");
                if under(whole, &key) != under(&narrow, &key) {
                    return Err(format!(
                        "--fields {name} changed section {other}, which lacks it"
                    ));
                }
            }
            checked = true;
        }
    }
    Ok(checked)
}

/// Dotted paths into the objects of the first row under a section key.
fn paths(reply: &Value, word: &str) -> Vec<String> {
    let key = word.replace('-', "_");
    let Some(row) = under(reply, &key)
        .into_iter()
        .find_map(|section| first_row(&section))
    else {
        return Vec::new();
    };
    row.as_object()
        .into_iter()
        .flatten()
        .flat_map(|(name, value)| {
            value
                .as_object()
                .into_iter()
                .flatten()
                .map(move |(inner, _)| format!("{name}.{inner}"))
        })
        .collect()
}

fn first_row(value: &Value) -> Option<Value> {
    match value {
        Value::Array(items) if items.first().is_some_and(Value::is_object) => {
            items.first().cloned()
        }
        Value::Array(items) => items.iter().find_map(first_row),
        Value::Object(map) => map.values().find_map(first_row),
        _ => None,
    }
}

/// Every value under this key anywhere in the reply's result, without stamps.
fn under(reply: &Value, key: &str) -> Vec<Value> {
    fn walk(value: &Value, key: &str, out: &mut Vec<Value>) {
        match value {
            Value::Object(map) => {
                for (name, inner) in map {
                    if name == key {
                        out.push(unstamped(inner));
                    } else {
                        walk(inner, key, out);
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|item| walk(item, key, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    if let Some(result) = reply.get("result") {
        walk(result, key, &mut out);
    }
    out
}

fn unstamped(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .filter(|(name, _)| !STAMPS.contains(&name.as_str()))
                .map(|(name, inner)| (name.clone(), unstamped(inner)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(unstamped).collect()),
        other => other.clone(),
    }
}

/// Values a second run may stamp anew.
const STAMPS: [&str; 1] = ["fetched"];

/// Whether every object of `part` keeps only keys of the same object of `full`, with every
/// list the same length in the same order.
fn within(part: Option<&Value>, full: Option<&Value>) -> bool {
    match (part, full) {
        (Some(part), Some(full)) => values(part, full),
        (part, full) => part == full,
    }
}

fn values(part: &Value, full: &Value) -> bool {
    match (part, full) {
        (Value::Object(part), Value::Object(full)) => part.iter().all(|(key, value)| {
            full.get(key)
                .is_some_and(|whole| STAMPS.contains(&key.as_str()) || values(value, whole))
        }),
        (Value::Array(part), Value::Array(full)) => {
            part.len() == full.len() && part.iter().zip(full).all(|(part, full)| values(part, full))
        }
        _ => part == full,
    }
}

/// The recorded sections that use dedicated replies, nested lists, or single data rows.
fn additional_sections() -> Result<(), String> {
    let fixtures = super::launch::fixtures().join("replies");
    for (replay, command, field) in [
        (
            "adverse-event/faers-code-words",
            "get adverse-event 100 drugs",
            "medicinalproduct",
        ),
        (
            "adverse-event/faers-code-words",
            "get adverse-event 100 reactions",
            "term",
        ),
        (
            "drug/markets",
            "get drug dolutegravir markets --source drugsfda",
            "name",
        ),
        ("gene/civic", "get gene ERBB2 civic", "variant_count"),
        ("gene/constraint", "get gene TP53 constraint", "loeuf"),
        (
            "article/fulltext/live-all",
            "get article 33123537 citations",
            "cited_by",
        ),
        ("article/pdf/synthetic", "get article 41351423 pdf", "url"),
        (
            "protein/native-sections",
            "get protein P00533 sequence",
            "length",
        ),
        (
            "cell-line/native-sections",
            "get cell-line CVCL_0004 identity",
            "id",
        ),
        (
            "cell-line/k562-xrefs",
            "get cell-line CVCL_0004 xrefs",
            "id",
        ),
        (
            "cell-line/native-sections",
            "get cell-line CVCL_0004 variants",
            "description",
        ),
        (
            "cell-line/native-sections",
            "get cell-line CVCL_0004 chembl",
            "name",
        ),
        (
            "gwas/synthetic",
            "get gwas GCST000035 associations",
            "p_value",
        ),
        (
            "variant-population/homozygotes-joint",
            "get variant 1-11796321-G-A population",
            "homozygote_count",
        ),
        (
            "variant-population/homozygotes-joint",
            "variant population 1-11796321-G-A --detail --limit 3",
            "homozygote_count",
        ),
    ] {
        let env = Environment::new().map_err(|error| error.to_string())?;
        compare(env, &fixtures.join(replay), command, field)?;
    }
    let env = Environment::new().map_err(|error| error.to_string())?;
    let replay = env.root().join("tldr");
    response_cases::copy_capture("article/fulltext/live-all", &replay, |_, exchange| {
        if exchange
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| url.contains("fields=tldr%2C"))
            && let Some(body) = exchange
                .pointer_mut("/exchanges/0/body/value")
                .and_then(Value::as_object_mut)
        {
            body.insert(
                "tldr".into(),
                serde_json::json!({"text":"Synthetic summary","model":"test-model"}),
            );
            body.insert("citationCount".into(), Value::from(7));
        }
    })
    .map_err(|error| error.to_string())?;
    compare(env, &replay, "get article 33123537 tldr", "text")?;

    let env = Environment::new().map_err(|error| error.to_string())?;
    let key = "synthetic-fields-case";
    env.config(&serde_json::json!({"keys":{"oncokb_api_key":key}}))
        .map_err(|error| error.to_string())?;
    let replay = super::licensed_cache::scoped_gene_fixture(&env, key)?;
    compare(env, &replay, "get gene TP53 oncokb", "version")
}

fn compare(
    env: Environment,
    replay: &std::path::Path,
    command: &str,
    field: &str,
) -> Result<(), String> {
    let globals = env.globals(Some(replay), false);
    let probe = Probe { env, globals };
    let args: Vec<_> = command.split_whitespace().map(str::to_owned).collect();
    let (code, whole) = probe.run(None, &args)?;
    if code != Some(0) {
        return Err(format!("{command}: whole read returned {code:?}"));
    }
    let narrow = narrowed(&probe, &args, field, code, &whole)
        .map_err(|error| format!("{command}: {error}"))?;
    if size(narrow.get("result")) >= size(whole.get("result")) {
        return Err(format!(
            "{command} --fields {field}: result did not get smaller"
        ));
    }
    Ok(())
}
