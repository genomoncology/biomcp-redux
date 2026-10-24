//! Source settings through the installed command boundary, isolated from user state.
use super::{
    configuration::{cached, invoke, root},
    launch::Environment,
    mcp,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::PathBuf};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("sources/health-keeps-local-recording-errors-typed", || {
            health_local().map_err(Into::into)
        }),
        Trial::test("sources/disabled-before-warm-cache-and-sync", || {
            disabled().map_err(Into::into)
        }),
        Trial::test(
            "sources/health-terms-disable-union-and-unknown-refusal",
            || source_health().map_err(Into::into),
        ),
        Trial::test("sources/disabled-pools-never-restore-or-hydrate", || {
            stored_pool().map_err(Into::into)
        }),
        Trial::test("sources/disabled-recording-sends-no-request", || {
            no_request().map_err(Into::into)
        }),
    ]
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
        let disabled = spec.source == biodata::Source::Hgnc
            || spec.source == biodata::Source::Hpa
            || spec.terms.usage == biodata::Use::Restricted;
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
    if oncokb.get("reason") != Some(&json!("no_key"))
        || oncokb.get("state") != Some(&json!("skipped"))
    {
        return Err("missing key did not skip health".into());
    }
    Ok(())
}

fn stored_pool() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("pool-case");
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/article/search/synthetic-small");
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
        .filter(|s| s.source != biodata::Source::MyGene)
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
