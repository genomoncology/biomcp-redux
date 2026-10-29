//! Source settings through the installed command boundary, isolated from user state.
use super::{
    configuration::{cached, invoke, root},
    launch::Environment,
    mcp,
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("source-controls/article-resolver-retries-respect-disabled", || {
            article_resolver_retries().map_err(Into::into)
        }),
        transcript_trial(
            "source-controls/next-preserves-selected-config-and-cli-policy",
            || next_config().map_err(Into::into),
        ),
        transcript_trial("source-controls/keyed-health-uses-declared-api-route", || {
            keyed_health().map_err(Into::into)
        }),
        transcript_trial("source-controls/disabled-leg-preserves-enabled-fold", || {
            enabled_fold().map_err(Into::into)
        }),
        transcript_trial("source-controls/health-keeps-local-recording-errors-typed", || {
            health_local().map_err(Into::into)
        }),
        transcript_trial("source-controls/disabled-before-warm-cache-and-sync", || {
            disabled().map_err(Into::into)
        }),
        transcript_trial(
            "source-controls/health-terms-disable-union-and-unknown-refusal",
            || source_health().map_err(Into::into),
        ),
        transcript_trial("source-controls/disabled-pools-never-restore-or-hydrate", || {
            stored_pool().map_err(Into::into)
        }),
        transcript_trial("source-controls/disabled-recording-sends-no-request", || {
            no_request().map_err(Into::into)
        }),
    ]
}

fn article_resolver_retries() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for command in [
        vec!["article", "text", "33123537", "--part", "1"],
        vec!["article", "supplement", "33123537", "file.csv"],
    ] {
        let mut args: Vec<String> = command.into_iter().map(str::to_owned).collect();
        args.extend([
            "--offline".into(),
            "--disable-sources".into(),
            "pubmed".into(),
        ]);
        let result = invoke(&env, args.clone(), 4)?;
        if result.get("next").is_some()
            || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
        {
            return Err(format!("disabled PubMed resolver offers a retry: {result}"));
        }
        if mcp::call_with_globals(&env, &args, true, &[])? != result {
            return Err("disabled article resolver differs over MCP".into());
        }
    }
    Ok(())
}

fn disabled() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("disabled-case");
    invoke(
        &env,
        cached(&["get", "gene", "BRAF"], &case, Some(&root())),
        0,
    )?;
    env.config(&json!({"disable_sources":["hgnc"]}))
        .map_err(|e| e.to_string())?;
    let result = invoke(&env, cached(&["get", "gene", "BRAF"], &case, None), 4)?;
    if result.pointer("/sources/0/state") != Some(&json!("skipped"))
        || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || result.pointer("/result/items/0/answer").is_some()
    {
        return Err("disabled HGNC restored a cached gene".into());
    }
    let reader = invoke(
        &env,
        vec![
            "article".into(),
            "text".into(),
            "33123537".into(),
            "--part".into(),
            "1".into(),
            "--disable-sources".into(),
            "pubmed".into(),
            "--offline".into(),
        ],
        4,
    )?;
    if reader.pointer("/result/content/source") != Some(&json!("pubmed"))
        || reader.pointer("/result/content/state") != Some(&json!("skipped"))
        || reader.pointer("/result/content/reason") != Some(&json!("disabled"))
    {
        return Err("reader attributed a disabled resolver to its enabled text source".into());
    }
    let result = invoke(&env, cached(&["sync", "hgnc"], &case, None), 4)?;
    if result.pointer("/sources/0/state") != Some(&json!("skipped"))
        || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err("sync ignored disabled source".into());
    }
    Ok(())
}

fn source_health() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"disable_sources":["hgnc"]}))
        .map_err(|e| e.to_string())?;
    let health = invoke(
        &env,
        vec![
            "health".into(),
            "--offline".into(),
            "--disable-sources".into(),
            "restricted,hpa".into(),
        ],
        0,
    )?;
    let rows = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .ok_or("missing health rows")?;
    let names = rows
        .iter()
        .filter_map(|r| r.get("source").and_then(Value::as_str))
        .collect::<std::collections::BTreeSet<_>>();
    if rows.len() != biomcp_adapters::registry::SOURCES.len() || names.len() != rows.len() {
        return Err("health omitted or duplicated registered sources".into());
    }
    for spec in biomcp_adapters::registry::SOURCES {
        let row = rows
            .iter()
            .find(|r| r.get("source") == Some(&json!(spec.source)))
            .ok_or("source missing")?;
        if row.get("terms") != Some(&json!(spec.terms.name))
            || row.get("terms_url") != Some(&json!(spec.terms.url))
        {
            return Err("health lost publisher terms".into());
        }
        let disabled = biomcp_adapters::registry::disabled(&[
            "hgnc".into(),
            "hpa".into(),
            "restricted".into(),
        ])
        .map_err(|e| e.to_string())?
        .contains(&spec.source);
        if disabled
            && (row.get("state") != Some(&json!("skipped"))
                || row.get("reason") != Some(&json!("disabled"))
                || row.get("latency_ms").is_some())
        {
            return Err(format!("disabled source probed: {row}"));
        }
    }
    invoke(
        &env,
        vec![
            "health".into(),
            "--disable-sources".into(),
            "not-implemented".into(),
            "--offline".into(),
        ],
        2,
    )?;
    env.config(&json!({})).map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    super::schema::Registry::embedded()?.validate(&health, &["health".into()], true)?;
    let oncokb = health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .and_then(|r| r.iter().find(|v| v.get("source") == Some(&json!("oncokb"))))
        .ok_or("OncoKB absent")?;
    if oncokb.get("reason") != Some(&json!("no-key"))
        || oncokb.get("state") != Some(&json!("skipped"))
    {
        return Err("missing key did not skip health".into());
    }
    Ok(())
}

fn stored_pool() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("pool-case");
    let fixture = crate::runner::launch::fixtures().join("replies/article/search/synthetic-small");
    let base = ["search", "article", "AKT1 E17K", "--limit", "2"];
    invoke(&env, cached(&base, &case, Some(&fixture)), 0)?;
    let args = cached(
        &[
            "search",
            "article",
            "AKT1 E17K",
            "--limit",
            "2",
            "--disable-sources",
            "pubmed",
        ],
        &case,
        None,
    );
    let result = invoke(&env, args.clone(), 4)?;
    if result.get("result").is_some()
        || result.pointer("/sources/0/state") != Some(&json!("skipped"))
        || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err(format!("disabled pool restored rows: {result}"));
    }
    mcp::call_with_globals(
        &env,
        &[
            "search".into(),
            "article".into(),
            "AKT1 E17K".into(),
            "--disable-sources".into(),
            "pubmed".into(),
        ],
        true,
        &env.globals(Some(&fixture), false),
    )?;
    Ok(())
}
fn no_request() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}", listener.local_addr().map_err(|e| e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    let record = env.root().join("record");
    let args = vec![
        "get".into(),
        "gene".into(),
        "BRAF".into(),
        "--disable-sources".into(),
        "hgnc".into(),
        "--record".into(),
        record.to_string_lossy().into(),
    ];
    let result = invoke(&env, args, 4)?;
    if result.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err("source did not report disabled".into());
    }
    let mut disabled = biomcp_adapters::registry::SOURCES
        .iter()
        .filter(|s| s.terms.usage != biomcp_types::Use::Restricted)
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>();
    disabled.extend([
        "restricted".into(),
        "noncommercial".into(),
        "myvariant".into(),
    ]);
    let health = invoke(
        &env,
        vec![
            "health".into(),
            "--disable-sources".into(),
            disabled.join(","),
            "--record".into(),
            record.to_string_lossy().into(),
        ],
        0,
    )?;
    if !health
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .is_some_and(|rows| {
            rows.iter()
                .all(|row| row.get("reason") == Some(&json!("disabled")))
        })
    {
        return Err("terms group failed to skip its declared sources".into());
    }
    if listener.accept().is_ok() {
        return Err("disabled source opened a network connection".into());
    }
    for source in fs::read_dir(record).map_err(|e| e.to_string())? {
        let source = source.map_err(|e| e.to_string())?;
        if source.file_type().map_err(|e| e.to_string())?.is_dir() {
            return Err("disabled source wrote a recording".into());
        }
    }
    Ok(())
}

fn health_local() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let missing = env.root().join("missing-recording");
    fs::create_dir(&missing).map_err(|e| e.to_string())?;
    let disabled = biomcp_adapters::registry::SOURCES
        .iter()
        .filter(|s| s.source != biomcp_types::Source::MyGene)
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let result = invoke(
        &env,
        vec![
            "health".into(),
            "--replay".into(),
            missing.to_string_lossy().into(),
            "--disable-sources".into(),
            disabled,
        ],
        5,
    )?;
    if result.pointer("/error/code") != Some(&json!("local.failure")) {
        return Err("health lost its typed local error".into());
    }
    Ok(())
}

fn enabled_fold() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let fixture = crate::runner::launch::fixtures().join("replies/article/fold/synthetic-fold");
    let case = env.root().join("enabled-fold");
    let args = [
        "search",
        "article",
        "AKT1 E17K",
        "--source",
        "pubmed,europepmc",
        "--disable-sources",
        "pubmed",
        "--limit",
        "2",
    ];
    for recording in [Some(fixture.as_path()), None] {
        let result = invoke(&env, cached(&args, &case, recording), 0)?;
        let sources = result
            .get("sources")
            .and_then(Value::as_array)
            .ok_or("missing sources")?;
        let pubmed = sources
            .iter()
            .find(|s| s.get("source") == Some(&json!("pubmed")))
            .ok_or("missing disabled source")?;
        let europe = sources
            .iter()
            .find(|s| s.get("source") == Some(&json!("europepmc")))
            .ok_or("missing enabled source")?;
        if pubmed.get("state") != Some(&json!("skipped"))
            || pubmed.get("reason") != Some(&json!("disabled"))
            || europe.get("state") != Some(&json!("ok"))
            || result.pointer("/result/items/0/pmid") != Some(&json!("2222"))
        {
            return Err(format!("disabled leg hid enabled source rows: {result}"));
        }
        if result.get("next").is_some() {
            return Err("mixed-source continuation still calls disabled PubMed".into());
        }
    }
    Ok(())
}

fn keyed_health() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let disabled = biomcp_adapters::registry::SOURCES
        .iter()
        .filter(|s| s.source != biomcp_types::Source::Oncokb)
        .map(|s| s.source.to_string())
        .collect::<Vec<_>>();
    env.config(&json!({"disable_sources": disabled,
        "keys":{"oncokb_api_key":"synthetic-placeholder"}}))
        .map_err(|e| e.to_string())?;
    let fixture = crate::runner::launch::fixtures().join("replies/gene/oncokb-synthetic/data");
    let result = invoke(
        &env,
        vec![
            "health".into(),
            "--replay".into(),
            fixture.to_string_lossy().into(),
        ],
        0,
    )?;
    let rows = result
        .pointer("/result/sources")
        .and_then(Value::as_array)
        .ok_or("missing health rows")?;
    let oncokb = rows
        .iter()
        .find(|s| s.get("source") == Some(&json!("oncokb")))
        .ok_or("missing keyed source")?;
    if oncokb.get("state") != Some(&json!("ok")) || oncokb.get("latency_ms").is_none() {
        return Err("keyed probe did not use the declared API endpoint".into());
    }
    Ok(())
}

fn next_config() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let config = env.root().join("selected config.json");
    fs::write(&config, r#"{"disable_sources":["semantic-scholar"]}"#).map_err(|e| e.to_string())?;
    let mut globals = env.globals(
        Some(&super::launch::fixtures().join("replies/article/get/synthetic-derived")),
        false,
    );
    globals.extend([
        "--config".into(),
        config.to_string_lossy().into(),
        "--disable-sources".into(),
        "litsense".into(),
    ]);
    let mut args = globals.clone();
    args.extend(["get".into(), "article".into(), "33123537".into()]);
    let result = invoke(&env, args, 0)?;
    if result.pointer("/next/0/args")
        != Some(&json!([
            "--config",
            config,
            "--disable-sources",
            "litsense",
            "article",
            "citations",
            "33123537"
        ]))
    {
        return Err(format!(
            "follow-up lost config selection or repeated config exclusions: {result}"
        ));
    }
    if mcp::call_with_globals(
        &env,
        &["get".into(), "article".into(), "33123537".into()],
        false,
        &globals,
    )? != result
    {
        return Err("selected config follow-up differs over MCP".into());
    }
    fs::write(
        config,
        r#"{"disable_sources":["semantic-scholar","icite"]}"#,
    )
    .map_err(|e| e.to_string())?;
    let mut next = env.globals(
        Some(&super::launch::fixtures().join("replies/article/get/synthetic-derived")),
        false,
    );
    next.extend(
        result
            .pointer("/next/0/args")
            .and_then(Value::as_array)
            .ok_or("missing follow-up")?
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned),
    );
    let skipped = invoke(&env, next, 4)?;
    if skipped.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err(format!(
            "follow-up did not reuse selected configuration: {skipped}"
        ));
    }
    Ok(())
}
