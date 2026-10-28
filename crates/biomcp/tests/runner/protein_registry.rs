//! Feature source-controls: protein publishers have one open declaration each.
use crate::runner::transcript::trial as transcript_trial;
use super::{configuration::invoke, launch::Environment};
use biomcp_adapters::registry;
use biomcp_types::{Reason, SourceState};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod admission;
mod health;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        transcript_trial("protein-registry/bounded-health", || {
            health::run().map_err(Into::into)
        }),
        transcript_trial("protein-registry/policy-before-io", || {
            admission::run().map_err(Into::into)
        }),
        transcript_trial("protein-registry/configuration-and-cli", || {
            switches().map_err(Into::into)
        }),
        transcript_trial("protein-registry/no-string", || {
            if registry::SOURCES
                .iter()
                .any(|s| s.source.to_string() == "string")
            {
                Err("STRING must not be registered".into())
            } else {
                Ok(())
            }
        }),
    ];
    for (name, usage, terms, url) in [
        (
            "interpro",
            "open",
            "CC0",
            "https://interpro-documentation.readthedocs.io/en/latest/license.html",
        ),
        (
            "complex-portal",
            "open",
            "CC0",
            "https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/",
        ),
        (
            "uniprot",
            "attribution",
            "CC BY 4.0",
            "https://www.uniprot.org/help/license",
        ),
    ] {
        trials.push(transcript_trial(
            format!("protein-registry/{name}-declaration"),
            move || declaration(name, usage, terms, url).map_err(Into::into),
        ));
    }
    trials
}
fn declaration(name: &str, usage: &str, terms: &str, url: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let row = rows(&health)?
        .iter()
        .find(|r| r["source"] == name)
        .ok_or("missing protein source")?;
    if row["use"] != usage
        || row["terms"] != terms
        || row["terms_url"] != url
        || row["disabled"] != false
        || row["reason"] != "offline"
    {
        return Err(format!("wrong publisher declaration: {row}"));
    }
    let specs = registry::SOURCES
        .iter()
        .filter(|s| s.source.to_string() == name)
        .collect::<Vec<_>>();
    let [spec] = specs.as_slice() else {
        return Err("source must be unique".into());
    };
    if !matches!(spec.key, biomcp_fetch::KeyUse::None) {
        return Err("source must be keyless".into());
    }
    for state in [
        SourceState::Ok,
        SourceState::Stale,
        SourceState::Skipped,
        SourceState::Failed,
    ] {
        let mut outcome = registry::outcome(spec.source, state, Some(Reason::Disabled));
        outcome.count = Some(0);
        let value = serde_json::to_value(outcome).map_err(|e| e.to_string())?;
        if value.get("use") != Some(&json!(usage))
            || value.get("terms") != Some(&json!(terms))
            || value.get("terms_url") != Some(&json!(url))
        {
            return Err(format!("lost outcome terms: {value}"));
        }
    }
    Ok(())
}
fn rows(health: &Value) -> Result<&Vec<Value>, String> {
    health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .ok_or("missing health rows".into())
}
fn switches() -> Check {
    for configured in [false, true] {
        for name in ["interpro", "complex-portal", "restricted", "noncommercial"] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let mut args = vec!["health".into(), "--offline".into()];
            if configured {
                env.config(&json!({"disable_sources":[name]}))
                    .map_err(|e| e.to_string())?;
            } else {
                args.extend(["--disable-sources".into(), name.into()]);
            }
            let health = invoke(&env, args, 0)?;
            for source in ["interpro", "complex-portal", "uniprot"] {
                let row = rows(&health)?
                    .iter()
                    .find(|r| r["source"] == source)
                    .ok_or("missing source")?;
                if row["disabled"] != (name == source)
                    || row["reason"]
                        != if name == source {
                            "disabled"
                        } else {
                            "offline"
                        }
                {
                    return Err(format!("wrong admission: {row}"));
                }
            }
        }
    }
    Ok(())
}
