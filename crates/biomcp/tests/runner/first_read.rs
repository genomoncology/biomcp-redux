//! Features diagnostic-cdx and disease-orphadata: a live first read fetches a missing small list;
//! an offline one never fetches.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::transcript::trial("diagnostic-cdx/first-read-fetches-the-list", || {
            companion(None).map_err(Into::into)
        }),
        super::transcript::trial("diagnostic-cdx/over-limit-first-read-asks-for-sync", || {
            companion(Some(1000)).map_err(Into::into)
        }),
        super::transcript::trial("diagnostic-cdx/offline-first-read-never-fetches", || {
            offline().map_err(Into::into)
        }),
        super::transcript::trial("disease-orphadata/first-read-fetches-the-files", || {
            prevalence().map_err(Into::into)
        }),
    ]
}

/// The synthetic publisher bodies from one replay folder, by URL path.
fn bodies(capture: &str) -> Result<Vec<(String, String)>, String> {
    let folder = super::launch::fixtures().join("replies").join(capture);
    let mut found = Vec::new();
    for file in fs::read_dir(folder).map_err(|e| e.to_string())? {
        let path = file.map_err(|e| e.to_string())?.path();
        let record: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let url = record
            .get("url")
            .and_then(Value::as_str)
            .ok_or("capture URL")?;
        let body = record
            .pointer("/exchanges/0/body/value")
            .and_then(Value::as_str)
            .ok_or("capture body")?;
        let path = url
            .split_once(".com")
            .or_else(|| url.split_once(".gov"))
            .ok_or("capture host")?;
        found.push((path.1.to_owned(), body.to_owned()));
    }
    Ok(found)
}

/// A fresh home whose only route to the publisher is the synthetic stub.
fn live(host: &'static str, capture: &str, extra: Value) -> Result<(Environment, Stub), String> {
    let bodies = bodies(capture)?;
    let stub = Stub::for_sources(
        &[host],
        |r| {
            if r.contains(".xml") {
                "application/xml"
            } else {
                "text/html"
            }
        },
        move |r| {
            bodies
                .iter()
                .find(|(path, _)| r.contains(path.as_str()))
                .map(|(_, body)| body.clone())
                .ok_or_else(|| format!("unexpected request: {r}"))
        },
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    stub.retain(env.shared_root());
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    if let (Some(config), Some(extra)) = (config.as_object_mut(), extra.as_object()) {
        config.extend(extra.clone());
    }
    env.config(&config).map_err(|e| e.to_string())?;
    Ok((env, stub))
}

fn words(args: &[&str]) -> Vec<String> {
    args.iter().map(|word| (*word).to_owned()).collect()
}

/// Without a sync, the first companion search answers; over the size limit it still asks.
fn companion(limit: Option<u64>) -> Check {
    let extra = limit.map_or_else(|| json!({}), |max| json!({"auto_sync_max_bytes": max}));
    let (env, _stub) = live("www.fda.gov", "diagnostic/fda-cdx-synthetic/fda-cdx", extra)?;
    let search = words(&[
        "search",
        "diagnostic",
        "--source",
        "fda-cdx",
        "--biomarker",
        "KRAS",
    ]);
    let reply = invoke(&env, search, if limit.is_some() { 4 } else { 0 })?;
    let answered = reply.get("status") == Some(&json!("complete"))
        && reply.pointer("/sources/0/state") == Some(&json!("ok"))
        && reply
            .pointer("/page/returned")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0;
    let asks = reply.pointer("/sources/0/state") == Some(&json!("syncing"))
        && reply.pointer("/next/0/args") == Some(&json!(["sync", "fda-cdx"]));
    if limit.is_none() && !answered || limit.is_some() && !asks {
        return Err(format!(
            "first companion read with limit {limit:?}: {reply}"
        ));
    }
    Ok(())
}

/// Offline, a first companion search asks for a sync and no request reaches the publisher.
fn offline() -> Check {
    let (env, stub) = live(
        "www.fda.gov",
        "diagnostic/fda-cdx-synthetic/fda-cdx",
        json!({}),
    )?;
    let search = words(&[
        "--offline",
        "search",
        "diagnostic",
        "--source",
        "fda-cdx",
        "--biomarker",
        "KRAS",
    ]);
    let reply = invoke(&env, search, 4)?;
    let requests = stub.finish()?;
    if reply.pointer("/sources/0/state") != Some(&json!("syncing"))
        || reply.pointer("/next/0/args") != Some(&json!(["sync", "fda-cdx"]))
        || !requests.is_empty()
    {
        return Err(format!(
            "offline first read fetched or answered: {reply}; {requests:?}"
        ));
    }
    Ok(())
}

/// Without a sync, the first prevalence read fetches both Orphadata files and answers.
fn prevalence() -> Check {
    let (env, _stub) = live(
        "www.orphadata.com",
        "disease/orphadata-synthetic/orphadata",
        json!({}),
    )?;
    let get = words(&[
        "get",
        "disease",
        "Orphanet:900001",
        "prevalence",
        "--source",
        "orphadata",
    ]);
    let reply = invoke(&env, get, 0)?;
    if reply.get("status") != Some(&json!("complete"))
        || reply.pointer("/result/items/0/prevalence/1/value") != Some(&json!("1.5"))
    {
        return Err(format!("first prevalence read did not answer: {reply}"));
    }
    Ok(())
}
