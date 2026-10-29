//! Feature source-controls: public pathway declarations before runtime dispatch exists.
use super::launch::Environment;
use crate::runner::pathway_budgets as budgets;
use crate::runner::transcript::trial as transcript_trial;
use biomcp_adapters::registry;
use biomcp_types::{Reason, SourceState};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod admission;
mod record_urls;
const SOURCES: &[(&str, &str, &str)] = &[
    ("reactome", "open", "https://reactome.org/license"),
    (
        "wikipathways",
        "open",
        "https://www.wikipathways.org/help.html",
    ),
];
pub(super) fn trials() -> Vec<Trial> {
    SOURCES
        .iter()
        .map(|&(source, usage, url)| {
            transcript_trial(
                format!("pathway-registry/{source}-declaration"),
                move || declaration(source, usage, url).map_err(Into::into),
            )
        })
        .chain([
            transcript_trial("source-controls/canonical-record-urls", || {
                record_urls::run().map_err(Into::into)
            }),
            transcript_trial("pathway-reactome/policy-before-io", || {
                admission::run().map_err(Into::into)
            }),
            transcript_trial("pathway-reactome/configuration-union", || {
                switches().map_err(Into::into)
            }),
        ])
        .collect()
}
fn declaration(name: &str, usage: &str, url: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = budgets::invoke(
        "runner/pathway_registry/declaration/1",
        &env,
        vec!["health".into(), "--offline".into()],
        0,
    )?;
    let row = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter()
                .find(|r| r.get("source").and_then(Value::as_str) == Some(name))
        })
        .ok_or_else(|| format!("missing {name}"))?;
    if row.get("use") != Some(&json!(usage))
        || row.get("terms_url") != Some(&json!(url))
        || row.get("disabled") != Some(&json!(false))
        || row
            .get("terms")
            .and_then(Value::as_str)
            .is_none_or(|t| !t.contains("CC0"))
    {
        return Err(format!("wrong declaration: {row}"));
    }
    let source = name.parse().map_err(|e: biodata::Error| e.to_string())?;
    if registry::SOURCES
        .iter()
        .filter(|s| s.source == source)
        .count()
        != 1
    {
        return Err("source must have exactly one declaration".into());
    }
    for state in [
        SourceState::Ok,
        SourceState::Stale,
        SourceState::Skipped,
        SourceState::Failed,
    ] {
        let mut outcome = registry::outcome(source, state, Some(Reason::Disabled));
        outcome.count = Some(0);
        let value = serde_json::to_value(outcome).map_err(|e| e.to_string())?;
        if value.get("use") != row.get("use") {
            return Err(format!("lost use: {value}"));
        }
        for field in ["source", "terms", "terms_url"] {
            if value.get(field) != row.get(field) {
                return Err(format!("lost {field}: {value}"));
            }
        }
    }
    for alias in ["kegg", "gprofiler", "g:Profiler", "WikiPathways"] {
        if registry::disabled(&[alias.into()]).is_ok() {
            return Err(format!("accepted alias {alias}"));
        }
    }
    Ok(())
}
fn switches() -> Check {
    for (configured, command) in [
        ("", "reactome"),
        ("", "wikipathways"),
        ("", "open"),
        ("reactome", ""),
        ("wikipathways", ""),
        ("open", ""),
        ("wikipathways", "cpic"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let configured_sources: Vec<&str> = if configured.is_empty() {
            vec![]
        } else {
            vec![configured]
        };
        env.config(&json!({"disable_sources": configured_sources}))
            .map_err(|e| e.to_string())?;
        let mut args = vec!["health".into(), "--offline".into()];
        if !command.is_empty() {
            args.extend(["--disable-sources".into(), command.into()]);
        }
        let health = budgets::invoke("runner/pathway_registry/switches/1", &env, args, 0)?;
        let rows = health
            .pointer("/result/sources")
            .and_then(Value::as_array)
            .ok_or("missing sources")?;
        for &(name, _, _) in SOURCES {
            let row = rows
                .iter()
                .find(|r| r.get("source") == Some(&json!(name)))
                .ok_or("missing source")?;
            let disabled =
                name == command || name == configured || command == "open" || configured == "open";
            if disabled
                && (row.get("state") != Some(&json!("skipped"))
                    || row.get("reason") != Some(&json!("disabled")))
            {
                return Err(format!("disabled health did not skip: {row}"));
            }
            if row.get("disabled") != Some(&json!(disabled)) {
                return Err(format!("bad union {row}"));
            }
        }
    }
    Ok(())
}
