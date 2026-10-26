//! Feature source-controls: adverse event publisher declarations precede dispatch.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    ["faers", "maude"]
        .into_iter()
        .map(|source| {
            Trial::test(
                format!("adverse-event-registry/{source}-declaration"),
                move || declaration(source).map_err(Into::into),
            )
        })
        .chain([
            Trial::test("adverse-event-registry/policy-before-io", || {
                admission::run().map_err(Into::into)
            }),
            Trial::test("adverse-event-registry/health-switch-union", || {
                health_switches().map_err(Into::into)
            }),
            Trial::test("adverse-event-registry/health-before-io", || {
                health_before_io().map_err(Into::into)
            }),
            Trial::test("adverse-event-registry/outcome-terms", || {
                outcomes().map_err(Into::into)
            }),
        ])
        .collect()
}
fn declaration(source: &str) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let row = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .and_then(|rows| rows.iter().find(|row| row["source"] == source))
        .ok_or_else(|| format!("missing {source}"))?;
    if row["usage"] != "open"
        || row["disabled"] != false
        || row["terms_url"] != "https://open.fda.gov/terms/"
    {
        return Err(format!("wrong publisher: {row}"));
    }
    let (name, url) = if source == "faers" {
        (
            "meddra",
            "https://admin.meddra.org/sites/default/files/page/documents/000198_statement_on_meddra_data_sharing_1_1.pdf",
        )
    } else {
        ("gmdn", "https://open.fda.gov/license/")
    };
    let component = row["components"]
        .as_array()
        .and_then(|rows| rows.iter().find(|row| row["name"] == name))
        .ok_or("missing component")?;
    if component["disabled"] != false
        || component.pointer("/terms/usage").and_then(Value::as_str) != Some("restricted")
        || component.pointer("/terms/url").and_then(Value::as_str) != Some(url)
    {
        return Err(format!("wrong component: {component}"));
    }
    Ok(())
}

mod admission;
fn outcomes() -> Check {
    use biomcp_adapters::registry;
    use biomcp_types::{Reason, Source, SourceState};
    for source in [Source::Faers, Source::Maude, Source::Meddra, Source::Gmdn] {
        let terms = registry::terms(source).ok_or("missing declaration")?;
        for state in [
            SourceState::Ok,
            SourceState::Stale,
            SourceState::Skipped,
            SourceState::Failed,
        ] {
            let mut row = registry::outcome(source, state, Some(Reason::Disabled));
            row.count = Some(0);
            if row.usage != terms.usage
                || row.terms.as_ref() != terms.name
                || row.terms_url.as_ref() != terms.url
            {
                return Err(format!("outcome lost terms: {row:?}"));
            }
        }
    }
    Ok(())
}
fn health_switches() -> Check {
    use serde_json::json;
    for switch in ["faers", "maude", "meddra", "gmdn", "restricted"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&json!({"disable_sources":[switch]}))
            .map_err(|e| e.to_string())?;
        let cli_disabled = if switch == "gmdn" { "faers" } else { "maude" };
        let health = invoke(
            &env,
            vec![
                "health".into(),
                "--offline".into(),
                "--disable-sources".into(),
                cli_disabled.into(),
            ],
            0,
        )?;
        let rows = health
            .pointer("/result/sources")
            .and_then(Value::as_array)
            .ok_or("missing health")?;
        for source in ["faers", "maude"] {
            let row = rows
                .iter()
                .find(|r| r["source"] == source)
                .ok_or("missing source")?;
            let disabled = source == cli_disabled || switch == source;
            let component_disabled = disabled
                || switch == "restricted"
                || (source == "faers" && switch == "meddra")
                || (source == "maude" && switch == "gmdn");
            let component = row["components"]
                .as_array()
                .and_then(|r| r.first())
                .ok_or("missing component")?;
            if row["disabled"] != disabled
                || component["disabled"] != component_disabled
                || row["terms_url"] != "https://open.fda.gov/terms/"
            {
                return Err(format!("policy union failed: {switch}: {row}"));
            }
        }
    }
    Ok(())
}

fn health_before_io() -> Check {
    use biomcp_adapters::registry;
    use biomcp_types::Source;
    use serde_json::json;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    let replay = env.root().join("empty-replay");
    std::fs::create_dir(&replay).map_err(|e| e.to_string())?;
    let others = registry::SOURCES
        .iter()
        .filter(|s| !matches!(s.source, Source::Faers | Source::Maude))
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>();
    for switches in [
        vec!["faers", "maude"],
        vec!["meddra", "gmdn"],
        vec!["restricted"],
    ] {
        let mut disabled = others.clone();
        disabled.extend(switches.iter().map(|s| (*s).to_owned()));
        env.config(&json!({"disable_sources": disabled,
 "proxy": format!("http://{}",
 listener.local_addr().map_err(|e| e.to_string())?)}))
            .map_err(|e| e.to_string())?;
        for replay_mode in [false, true] {
            let mut args = vec!["health".into()];
            if replay_mode {
                args.extend(["--replay".into(), replay.to_string_lossy().into()]);
            }
            let health = invoke(&env, args, 0)?;
            let rows = health
                .pointer("/result/sources")
                .and_then(Value::as_array)
                .ok_or("missing health")?;
            for source in ["faers", "maude"] {
                let row = rows
                    .iter()
                    .find(|r| r["source"] == source)
                    .ok_or("missing source")?;
                if row["state"] != "skipped"
                    || row["reason"] != "disabled"
                    || row["terms"].as_str().is_none_or(str::is_empty)
                {
                    return Err(format!("health executed restricted record probe: {row}"));
                }
            }
        }
    }
    if listener.accept().is_ok() {
        return Err("disabled health opened HTTP".into());
    }
    Ok(())
}
