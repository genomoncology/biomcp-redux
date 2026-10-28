//! Feature source-controls: direct and aggregated HPO share publisher policy.
use crate::runner::transcript::trial as transcript_trial;
use super::{configuration::invoke, launch::Environment};
use biomcp_adapters::registry;
use biomcp_types::{Reason, Source, SourceState};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
mod admission;
mod health;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("hpo-registry/exact-term-health", || {
            health::run().map_err(Into::into)
        }),
        transcript_trial("hpo-registry/declaration", || {
            declaration().map_err(Into::into)
        }),
        transcript_trial("hpo-registry/policy-before-io", || {
            admission::run().map_err(Into::into)
        }),
        transcript_trial("hpo-registry/configuration-union", || {
            switches().map_err(Into::into)
        }),
    ]
}
fn declaration() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let row = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .and_then(|rows| rows.iter().find(|r| r["source"] == "hpo"))
        .ok_or("missing direct HPO")?;
    if row["use"] != "attribution"
        || row["disabled"] != false
        || row["reason"] != "offline"
        || row["terms"] != "HPO project custom license: attribution, version and integrity"
        || row["terms_url"] != "https://human-phenotype-ontology.github.io/license.html"
    {
        return Err(format!("wrong HPO declaration: {row}"));
    }
    let specs = registry::SOURCES
        .iter()
        .filter(|s| s.source == Source::Hpo)
        .collect::<Vec<_>>();
    let [spec] = specs.as_slice() else {
        return Err("HPO declaration must be unique".into());
    };
    if !matches!(spec.key, biomcp_fetch::KeyUse::None) {
        return Err("HPO must be keyless".into());
    }
    for component in registry::components().filter(|c| c.native_source == Some(Source::Hpo)) {
        if component.terms.name != spec.terms.name
            || component.terms.url != spec.terms.url
            || component.terms.usage != spec.terms.usage
        {
            return Err("HPO terms diverged".into());
        }
    }
    for state in [
        SourceState::Ok,
        SourceState::Stale,
        SourceState::Skipped,
        SourceState::Failed,
    ] {
        let mut outcome = registry::outcome(Source::Hpo, state, Some(Reason::Disabled));
        outcome.count = Some(0);
        let value = serde_json::to_value(outcome).map_err(|e| e.to_string())?;
        if value.get("use") != row.get("use")
            || value.get("terms") != row.get("terms")
            || value.get("terms_url") != row.get("terms_url")
            || value.get("fetched").is_some_and(|v| !v.is_null())
        {
            return Err(format!("lost source metadata: {value}"));
        }
    }
    Ok(())
}
fn switches() -> Check {
    for configured in [false, true] {
        for switch in ["hpo", "monarch", "restricted"] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let mut args = vec!["health".into(), "--offline".into()];
            if configured {
                env.config(&json!({"disable_sources":[switch]}))
                    .map_err(|e| e.to_string())?;
            } else {
                args.extend(["--disable-sources".into(), switch.into()]);
            }
            let health = invoke(&env, args, 0)?;
            let rows = health
                .pointer("/result/sources")
                .and_then(Value::as_array)
                .ok_or("missing health")?;
            for name in ["hpo", "monarch", "mydisease"] {
                let row = rows
                    .iter()
                    .find(|r| r["source"] == name)
                    .ok_or("missing source")?;
                if row["disabled"] != (name == switch) {
                    return Err(format!("bad switch: {row}"));
                }
                if name == "hpo"
                    && row["reason"]
                        != if switch == "hpo" {
                            "disabled"
                        } else {
                            "offline"
                        }
                {
                    return Err(format!("wrong direct HPO admission: {row}"));
                }
                if name != "hpo" {
                    let component = row["components"]
                        .as_array()
                        .and_then(|cs| cs.iter().find(|c| c["name"] == "hpo"))
                        .ok_or("missing HPO component")?;
                    if component["disabled"] != (switch == "hpo" || switch == name) {
                        return Err(format!("wrong component: {component}"));
                    }
                }
            }
        }
    }
    Ok(())
}
