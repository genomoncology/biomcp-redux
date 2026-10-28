//! Feature source-controls: public diagnostic declarations before runtime dispatch exists.
use super::{configuration::invoke, launch::Environment};
use crate::runner::transcript::trial as transcript_trial;
use biomcp_adapters::registry;
use biomcp_types::{Reason, SourceState};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod admission;
const SOURCES: &[(&str, &str, &str)] = &[
    (
        "gtr",
        "open",
        "https://www.ncbi.nlm.nih.gov/home/about/policies/",
    ),
    (
        "who-ivd",
        "noncommercial",
        "https://www.who.int/about/policies/terms-of-use",
    ),
    (
        "fda-cdx",
        "open",
        "https://www.fda.gov/about-fda/about-website/website-policies",
    ),
    ("fda-pma", "open", "https://open.fda.gov/terms/"),
    ("fda-510k", "open", "https://open.fda.gov/terms/"),
];
pub(super) fn trials() -> Vec<Trial> {
    SOURCES
        .iter()
        .map(|&(source, usage, url)| {
            transcript_trial(
                format!("diagnostic-registry/{source}-declaration"),
                move || declaration(source, usage, url).map_err(Into::into),
            )
        })
        .chain([
            transcript_trial("diagnostic-registry/policy-before-io", || {
                admission::run().map_err(Into::into)
            }),
            transcript_trial("diagnostic-registry/configuration-union", || {
                switches().map_err(Into::into)
            }),
        ])
        .collect()
}
fn declaration(name: &str, usage: &str, url: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
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
    for alias in ["GTR", "openfda-device"] {
        if registry::disabled(&[alias.into()]).is_ok() {
            return Err(format!("accepted alias {alias}"));
        }
    }
    Ok(())
}
fn switches() -> Check {
    for configured in [
        "gtr",
        "who-ivd",
        "fda-cdx",
        "fda-pma",
        "fda-510k",
        "noncommercial",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&json!({"disable_sources":[configured]}))
            .map_err(|e| e.to_string())?;
        let health = invoke(
            &env,
            vec![
                "health".into(),
                "--offline".into(),
                "--disable-sources".into(),
                "gtr".into(),
            ],
            0,
        )?;
        let rows = health
            .pointer("/result/sources")
            .and_then(Value::as_array)
            .ok_or("missing sources")?;
        for &(name, _, _) in SOURCES {
            let row = rows
                .iter()
                .find(|r| r.get("source") == Some(&json!(name)))
                .ok_or("missing source")?;
            let disabled = name == "gtr"
                || name == configured
                || configured == "noncommercial" && name == "who-ivd";
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
