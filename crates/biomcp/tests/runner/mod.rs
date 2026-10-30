mod adverse_event_failures;
mod adverse_event_registry;
mod adverse_event_retirements;
mod article_recovery;
mod article_save;
mod article_save_reply;
mod assertions;
mod author;
pub(super) mod case_main;
mod execution;
tool_only! { pub(super) mod tools_api; }
mod features;
mod orcid;
tool_only! { pub(super) mod parity; }
tool_only! { mod parity_inventory_proof; }
tool_only! { mod parity_shape_proof; }
use crate::runner::transcript::trial as transcript_trial;
use execution::run_step;
mod alphagenome;
mod alphagenome_tls;
mod cache_contention;
mod capture;
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
mod drug_recall;
mod drug_recall_capture;
mod drug_registry;
mod drug_retirements;
mod drug_who;
mod evidence;
mod faers_get;
mod faers_search;
mod format_selection;
mod gene_tables;
mod health_bundles;
tool_only! { mod hgnc_sync; }
mod held_proxy;
mod help_entry;
mod help_examples;
mod housekeeping;
mod hpo_registry;
mod http;
mod http_limits;
mod key_help;
mod key_registration;
mod key_reload;
pub(super) mod launch;
tool_only! { mod literature_values; }
mod litvar_tables;
tool_only! { pub(crate) mod live; }
tool_only! { mod live_follow_proof; }
tool_only! { mod live_proof; }
mod local_files;
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
mod mcp_boundaries;
mod mcp_hosted;
mod named_search;
mod nci;
mod nci_walk;
tool_only! { mod negative_cases; }
mod network_stub;
mod next_navigation;
mod normalization;
tool_only! { pub(super) mod other_source_value_proof; }
mod oncokb;
mod oncokb_context;
mod openfda_key;
mod openfda_website;
mod output_stream;
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
mod public_grammar;
mod publication;
mod recurrence;
mod recurrence_errors;
mod recurrence_metadata;
mod replay;
mod reply_comparison;
mod research;
mod response_cases;
mod schema;
tool_only! { mod schema_evolution; }
mod schema_proof;
mod scholar;
tool_only! { mod semantic_tables; }
mod skill;
mod source_controls;
mod source_deadlines;
mod source_groups;
mod source_terms;
mod source_text;
mod store_clean;
mod store_coexistence;
mod study_catalog;
mod supplement_tables;
mod terminal_edges;
mod transcript;
mod trial_bad_values;
mod trial_condition;
mod trial_documents;
mod trial_eligibility;
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

// Complete help: j18463 measured 114273 bytes; reviewed ceiling adds fifteen percent.
pub(super) const CLI_HELP_BYTES: usize = 152733;

use libtest_mimic::Trial;
use serde_json::Value;
use std::{fs, path::Path, sync::Arc};

mod case_data;
use case_data::{Case, Step, collect, read_case};

pub(super) fn discover(root: &Path) -> Result<Vec<Trial>, String> {
    let mut paths = Vec::new();
    collect(root, &mut paths).map_err(|error| format!("{}: {error}", root.display()))?;
    paths.sort();
    if paths.is_empty() {
        return Err(format!("{}: no YAML cases", root.display()));
    }
    let registry = Arc::new(schema::Registry::embedded()?);
    let trials = paths
        .into_iter()
        .map(|path| {
            let name = path
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .iter()
                .map(|part| part.to_str().ok_or("case path must be UTF-8"))
                .collect::<Result<Vec<_>, _>>()?
                .join("/");
            let case = read_case(&path)?;
            let name = format!("{}/{name}", case.feature);
            let registry = Arc::clone(&registry);
            Ok(transcript_trial(name, move || {
                run_case(&path, &registry).map(|_| ()).map_err(Into::into)
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(trials)
}

pub(super) fn validate_features(trials: &[Trial], complete: bool) -> Result<(), String> {
    features::validate(
        launch::feature_list(),
        trials.iter().map(Trial::name),
        complete,
    )
}

fn run_case(path: &Path, registry: &schema::Registry) -> Result<Value, String> {
    let mut case = read_case(path)?;
    if case.story.trim().is_empty() || case.steps.is_empty() {
        return Err("case requires a story and at least one step".into());
    }
    load_floors(&mut case, path)?;
    let expectations = case
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            if step.mcp_call.is_some()
                && (step.run.is_some()
                    || step.mcp.is_some()
                    || step.follow.is_some()
                    || step.walk.is_some())
            {
                return Err("named MCP call requires its own step".into());
            }
            if step.run.is_none()
                && step.mcp.is_none()
                && step.mcp_call.is_none()
                && step.follow.is_none()
            {
                return Err("step requires run, mcp or mcp_call".into());
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
    let replay = response_cases::replay(&environment, case.replay.as_deref(), case.cache_acquired)?;
    let mut previous = None;
    for (index, (step, expectations)) in case.steps.iter().zip(&expectations).enumerate() {
        transcript::case_step(index);
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

fn discovery_trials() -> Vec<Trial> {
    let mut trials = housekeeping::trials();
    trials.extend(help_entry::trials());
    trials.extend(skill::trials());
    trials.extend(public_grammar::trials());
    trials
}

pub(super) fn protocol_trials() -> Vec<Trial> {
    let mut trials = protocol::trials();
    trials.extend(output_stream::trials());
    trials.extend(discovery_trials());
    trials.extend(research::trials());
    trials.push(transcript_trial("article-help/executable-examples", || {
        help_examples::check().map_err(Into::into)
    }));
    trials.push(transcript_trial(
        "article-supplements/ping-after-bounded-refusal",
        || supplement_tables::ping_after_refusal().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "article-saves/explicit-original-files",
        || article_save::check().map_err(Into::into),
    ));
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
    trials.push(transcript_trial(
        "article-fulltext/source-boundaries",
        || pmc_tables::check().map_err(|e| e.to_string().into()),
    ));
    trials.push(transcript_trial(
        "article-search/snapshot-replacement",
        || pool_tables::check().map_err(Into::into),
    ));
    trials.extend(configuration::trials());
    trials.extend(replay::trials());
    trials.extend(article_recovery::trials());
    trials.extend(cache_contention::trials());
    trials.extend(credential_containment::trials());
    trials.extend(key_registration::trials());
    trials.extend(openfda_key::trials());
    trials.push(openfda_website::trial());
    trials.extend(key_help::trials());
    trials.extend(scholar::trials());
    trials.extend(evidence::trials());
    trials.extend(pdf::trials());
    trials.extend(source_controls::trials());
    trials.extend(health_bundles::trials());
    trials.extend(source_deadlines::trials());
    trials.extend(source_groups::trials());
    trials.extend(trial_groups::drug_trials());
    trials.extend(disease_trials());

    trials.extend(source_terms::trials());

    trials.extend(http::trials());
    trials.extend(mcp_boundaries::trials());
    trials.extend(mcp_hosted::trials());
    trials.extend(corporate::trials());
    trials.extend(format_selection::trials());
    trials.extend(terminal_edges::trials());
    trials.extend(variant::trials());
    trials.extend(markdown::trials());
    trials.extend(markdown_shapes::trials());
    trials.extend(markdown_variant_trial::trials());
    trials.extend(markdown_sources::trials());
    trials.push(transcript_trial(
        "mcp-parity/ping-passes-held-provider-call",
        || held_proxy::check().map_err(Into::into),
    ));
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
    run_sequence_with(step, expectations, previous, |current, expectations| {
        run_step(current, environment, expectations, registry, replay)
    })
}

fn run_sequence_with(
    step: &Step,
    expectations: &assertions::Expectations<'_>,
    previous: Option<&Value>,
    mut execute: impl FnMut(&Step, &assertions::Expectations<'_>) -> Result<Value, String>,
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
        return execute(&current, expectations);
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
        let value = execute(&current, &Vec::new())?;
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

mod variant_assertion_wire;
mod variant_assertions;
mod variant_condition_relationship;
mod variant_discovery;

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

mod gwas_budgets;
mod gwas_get;
mod gwas_search;

mod wikipathways_genes;

mod reply_readback;

tool_only! {
    pub(super) fn tool_trials() -> Result<Vec<Trial>, String> {
        let registry = Arc::new(schema::Registry::embedded()?);
        let mut trials = proof::trials();
        trials.push(reply_comparison::trial());
        trials.extend(literature_values::trials());
        trials.extend(hgnc_sync::trials());
        trials.extend(gene_tables::tool_trials());
        trials.extend(drug_native_identity::tool_trials());
        trials.extend(wikipathways::tool_trials());
        trials.extend(protein_sections::tool_trials());
        trials.extend(disease_mydisease::tool_trials());
        trials.extend(schema_proof::trials(registry));
        trials.extend(parity::trials());
        trials.push(parity_shape_proof::trial());
        trials.push(parity_inventory_proof::trial());
        trials.extend(live_proof::trials());
        trials.extend(live_follow_proof::trials());
        Ok(trials)
    }
}
