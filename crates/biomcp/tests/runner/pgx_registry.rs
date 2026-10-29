//! Feature source-controls: public pgx declarations before runtime dispatch exists.
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
        "cpic",
        "noncommercial",
        "https://www.clinpgx.org/page/dataUsagePolicy",
    ),
    (
        "clinpgx",
        "noncommercial",
        "https://www.clinpgx.org/page/dataUsagePolicy",
    ),
];
pub(super) fn trials() -> Vec<Trial> {
    SOURCES
        .iter()
        .map(|&(source, usage, url)| {
            transcript_trial(format!("pgx-registry/{source}-declaration"), move || {
                declaration(source, usage, url).map_err(Into::into)
            })
        })
        .chain([
            transcript_trial("source-controls/retired-source-before-io", || {
                retired().map_err(Into::into)
            }),
            transcript_trial("source-controls/policy-before-io", || {
                admission::run().map_err(Into::into)
            }),
            transcript_trial("source-controls/configuration-union", || {
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
        || row.get("terms").and_then(Value::as_str).is_none_or(|t| {
            !t.contains("noncommercial access restriction") || !t.contains("JavaScript shell")
        })
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
    for alias in ["pharmgkb", "pharmvar", "pharmcat", "dpwg"] {
        if registry::disabled(&[alias.into()]).is_ok() {
            return Err(format!("accepted alias {alias}"));
        }
    }
    Ok(())
}
fn switches() -> Check {
    for (configured, command) in [
        ("cpic", "cpic"),
        ("clinpgx", "clinpgx"),
        ("noncommercial", "noncommercial"),
        ("clinpgx", "cpic"),
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
                command.into(),
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
            let disabled = name == command || name == configured || configured == "noncommercial";
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

fn retired() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}", listener.local_addr().map_err(|e| e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    for args in [
        vec!["search", "pgx", "DPYD", "--source", "pharmgkb"],
        vec!["health", "--disable-sources", "cpic,pharmgkb"],
        vec!["health", "--disable-sources=pharmgkb"],
        vec!["search", "pgx", "DPYD", "--source=pharmgkb"],
    ] {
        let result = invoke(&env, args.into_iter().map(String::from).collect(), 2)?;
        if result.pointer("/error/code") != Some(&json!("usage.retired"))
            || result
                .pointer("/error/message")
                .and_then(Value::as_str)
                .is_none_or(|text| !text.contains("clinpgx"))
        {
            return Err(format!("missing replacement guidance: {result}"));
        }
    }
    if listener.accept().is_ok() {
        return Err("retired source opened a provider connection".into());
    }
    Ok(())
}
