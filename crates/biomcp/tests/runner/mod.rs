mod adverse_event_failures;
mod adverse_event_registry;
mod adverse_event_retirements;
mod article_save;
mod assertions;
mod author;
mod execution;
mod orcid;
use execution::run_step;
pub(super) mod cell_line;
mod clinpgx;
mod clinvar_key;
mod configuration;
mod corporate;
mod cpic;
mod credential_containment;
mod diagnostic_510k;
mod diagnostic_bundles;
mod diagnostic_cdx;
mod diagnostic_gtr;
mod diagnostic_pma;
mod diagnostic_registry;
mod diagnostic_retirements;
mod diagnostic_who;
mod disease_monarch;
mod disease_mydisease;
mod disease_ols;
mod disease_registry;
mod disease_seer;
mod drug_civic;
mod drug_civic_fixtures;
mod drug_ddinter;
mod drug_ema;
mod drug_failures;
mod drug_fda;
mod drug_identity;
mod drug_label;
mod drug_mychem;
mod drug_native;
mod drug_native_identity;
mod drug_next;
mod drug_registry;
mod drug_retirements;
mod drug_who;
mod evidence;
mod faers_get;
mod faers_search;
mod gene_tables;
mod held_proxy;
mod help_examples;
mod hpo_registry;
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
mod markdown_variant_sources;
mod markdown_variant_trial;
mod maude;
mod maude_budget;
mod maude_policy;
mod maude_snapshot;
mod mcp;
mod named_search;
mod nci;
mod nci_walk;
mod negative_cases;
mod network_stub;
mod next_navigation;
mod normalization;
mod oncokb;
mod oncokb_context;
mod pathway_budgets;
mod pathway_commands;
mod pathway_failures;
mod pathway_registry;
mod pathway_retirements;
mod pathway_sections;
mod pdf;
mod pgx_registry;
mod pgx_retirements;
mod phenotype;
mod pmc_tables;
mod pool_tables;
mod process;
pub(super) mod proof;
mod protein_budgets;
mod protein_commands;
mod protein_domains;
mod protein_registry;
mod protein_sections;
mod protocol;
mod publication;
mod recurrence;
mod recurrence_errors;
mod recurrence_metadata;
mod research;
mod response_cases;
mod schema;
mod schema_proof;
mod scholar;
mod semantic_tables;
mod source_controls;
mod source_terms;
mod store_clean;
mod study_catalog;
mod supplement_tables;
mod trial_condition;
mod trial_documents;
mod trial_get;
mod trial_group;
mod trial_groups;
mod trial_sites;
mod trial_tables;
mod trial_wider;
mod vaers;
mod variant;
mod variant_civic;
mod variant_filters;
mod variant_hotspots;
mod variant_population;
mod variant_population_release;
mod variant_search;
mod wikipathways;

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
    #[serde(default)]
    full_output_bytes: bool,
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

pub(super) fn protocol_trials() -> Vec<Trial> {
    let mut trials = protocol::trials();
    trials.extend(research::trials());
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
    trials.extend(protein_commands::trials());
    trials.extend(pathway_retirements::trials());
    trials.extend(pathway_failures::trials());
    trials.extend(protein_sections::trials());
    trials.extend(protein_domains::trials());
    trials.extend(trial_group::trials());
    trials.extend(clinvar_key::trials());
    trials.push(Trial::test("article-fulltext/source-boundaries", || {
        pmc_tables::check().map_err(|e| e.to_string().into())
    }));
    trials.push(Trial::test("article-search/snapshot-replacement", || {
        pool_tables::check().map_err(Into::into)
    }));
    trials.extend(configuration::trials());
    trials.extend(credential_containment::trials());
    trials.extend(key_registration::trials());
    trials.extend(scholar::trials());
    trials.extend(evidence::trials());
    trials.extend(pdf::trials());
    trials.extend(source_controls::trials());
    trials.extend(trial_groups::drug_trials());
    trials.extend(disease_trials());

    trials.extend(source_terms::trials());

    trials.extend(http::trials());
    trials.extend(corporate::trials());
    trials.extend(variant::trials());
    trials.extend(variant_assertion_wire::trials());
    trials.extend(variant_assertions::trials());
    trials.extend(variant_condition_relationship::trials());
    trials.extend(variant_population::trials());
    trials.extend(variant_population_release::trials());
    trials.extend(oncokb::trials());
    trials.extend(oncokb_context::trials());
    trials.extend(variant_civic::trials());
    trials.extend(variant_hotspots::trials());
    trials.extend(recurrence::trials());
    trials.extend(recurrence_errors::trials());
    trials.extend(recurrence_metadata::trials());
    trials.extend(normalization::trials());
    trials.extend(variant_search::trials());
    trials.extend(next_navigation::trials());
    trials.extend(variant_filters::trials());
    trials.extend(markdown::trials());
    trials.extend(markdown_shapes::trials());
    trials.extend(markdown_variant_trial::trials());
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

// Run installed HTTP ownership checks without resolving a build-checkout case directory.
pub(super) fn installed_http_trials() -> Vec<Trial> {
    http::trials()
}

mod variant_assertion_wire;
mod variant_assertions;
mod variant_condition_relationship;

pub(super) fn disease_native(path: &Path) -> Result<(), String> {
    disease_ols::native(path)
}
pub(super) fn mydisease_native(path: &Path) -> Result<(), String> {
    disease_mydisease::native(path)
}
pub(super) fn seer_native(path: &Path) -> Result<(), String> {
    disease_seer::native(path)
}

pub(super) fn trial_condition_native(path: &Path) -> Result<(), String> {
    trial_condition::native(path)
}

pub(super) fn monarch_native(path: &Path) -> Result<(), String> {
    disease_monarch::native(path)
}

fn disease_trials() -> Vec<Trial> {
    disease_registry::trials()
        .into_iter()
        .chain(disease_ols::trials())
        .chain(disease_mydisease::trials())
        .chain(disease_monarch::trials())
        .chain(phenotype::trials())
        .chain(cell_line::trials())
        .chain(author::trials())
        .chain(orcid::trials())
        .chain(disease_seer::trials())
        .collect()
}

pub(crate) fn faers_search_examples(directory: &std::path::Path) -> Result<(), String> {
    faers_search::record_examples(directory)
}
mod gwas_budgets;
mod gwas_get;
mod gwas_search;
pub(super) fn phenotype_native(path: &Path) -> Result<(), String> {
    phenotype::native(path)
}

pub(super) fn protein_native(path: &Path) -> Result<(), String> {
    protein_commands::native(path)
}
mod wikipathways_genes;
