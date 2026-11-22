//! Features diagnostic-cdx and disease-orphadata: a live first read fetches a missing small list;
//! an offline one never fetches.
mod ema;
mod parity;
mod publisher;

use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

type Check = Result<(), String>;

pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        super::transcript::trial("diagnostic-cdx/first-read-fetches-the-list", || {
            companion().map_err(Into::into)
        }),
        super::transcript::trial("diagnostic-cdx/over-limit-first-read-asks-for-sync", || {
            backoff().map_err(Into::into)
        }),
        super::transcript::trial("diagnostic-cdx/first-read-timeout-names-the-wait", || {
            timeout().map_err(Into::into)
        }),
        super::transcript::trial(
            "diagnostic-cdx/first-read-keeps-publisher-retry-after",
            || publisher_retry_after().map_err(Into::into),
        ),
        super::transcript::trial("diagnostic-cdx/offline-first-read-never-fetches", || {
            offline().map_err(Into::into)
        }),
        super::transcript::trial("disease-orphadata/first-read-fetches-the-files", || {
            prevalence().map_err(Into::into)
        }),
    ];
    trials.extend(ema::trials());
    trials.extend(parity::trials());
    trials.extend(publisher::trials());
    trials
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
        let url = url::Url::parse(url).map_err(|error| error.to_string())?;
        found.push((url.path().to_owned(), body.to_owned()));
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
    configured(stub, extra)
}

fn configured(stub: Stub, extra: Value) -> Result<(Environment, Stub), String> {
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

const COMPANION: &str = "diagnostic/fda-cdx-synthetic/fda-cdx";
const SEARCH: &[&str] = &[
    "search",
    "diagnostic",
    "--source",
    "fda-cdx",
    "--biomarker",
    "KRAS",
];

fn companion() -> Check {
    let (env, stub) = live("www.fda.gov", COMPANION, json!({}))?;
    let reply = invoke(&env, words(SEARCH), 0)?;
    stub.finish()?;
    let answered = reply.get("status") == Some(&json!("complete"))
        && reply.pointer("/sources/0/state") == Some(&json!("ok"))
        && reply
            .pointer("/page/returned")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            > 0;
    if !answered {
        return Err(format!("first companion read: {reply}"));
    }
    Ok(())
}

/// Separate CLI readers share a failed first fetch and explicit sync can recover immediately.
fn backoff() -> Check {
    let (env, stub) = live(
        "www.fda.gov",
        COMPANION,
        json!({"auto_sync_max_bytes":1000}),
    )?;
    let size = bodies(COMPANION)?
        .first()
        .ok_or("missing companion recording")?
        .1
        .len();
    let mut replies = Vec::new();
    for _ in 0..3 {
        replies.push(invoke(&env, words(SEARCH), 4)?);
    }
    invoke(&env, words(&["sync", "fda-cdx"]), 0)?;
    invoke(&env, words(SEARCH), 0)?;
    let requests = stub.finish()?;
    if requests.len() != 2 {
        return Err(format!(
            "expected one first fetch and one explicit sync, saw {} requests: {replies:?}",
            requests.len()
        ));
    }
    for reply in replies {
        check_failure(&reply, "size limit", &format!("{size} bytes"))?;
        if reply.pointer("/sources/0/state") != Some(&json!("syncing")) {
            return Err(format!("over-limit first fetch is not syncing: {reply}"));
        }
    }
    Ok(())
}

fn check_failure(reply: &Value, cause: &str, size: &str) -> Check {
    let message = reply
        .pointer("/sources/0/notices/0/message")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let delay = reply
        .pointer("/sources/0/notices/0/retry_after_seconds")
        .and_then(Value::as_u64);
    if !message.contains("fetch")
        || !message.contains(cause)
        || !message.contains(size)
        || delay.unwrap_or(0) == 0
        || reply.pointer("/next/0/args") != Some(&json!(["sync", "fda-cdx"]))
    {
        return Err(format!("missing first-fetch guidance: {reply}"));
    }
    Ok(())
}

/// The publisher accepted the request but never sent headers, so its size is unknown.
fn timeout() -> Check {
    let stub = Stub::for_source_with_held_response("www.fda.gov", "text/html", |_| Ok(None))?;
    let (env, stub) = configured(stub, json!({"command_timeout_seconds":2}))?;
    let first = invoke(&env, words(SEARCH), 4)?;
    let again = invoke(&env, words(SEARCH), 4)?;
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!(
            "expected one timed-out request, got {}",
            requests.len()
        ));
    }
    for reply in [first, again] {
        check_failure(&reply, "ran out of time", "size: unknown")?;
    }
    Ok(())
}

fn retry_peer(host: &str) -> Result<Stub, String> {
    Stub::with_headers(
        &[host],
        |_| "text/html",
        |_| Ok(String::new()),
        |_, bytes| bytes,
        |_| 429,
        |_| "Retry-After: 300\r\n".to_owned(),
    )
}

fn publisher_retry_after() -> Check {
    let (env, stub) = configured(retry_peer("www.fda.gov")?, json!({}))?;
    let first = invoke(&env, words(SEARCH), 4)?;
    let again = invoke(&env, words(SEARCH), 4)?;
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!(
            "expected one rate-limited request, got {}",
            requests.len()
        ));
    }
    for reply in [first, again] {
        check_failure(
            &reply,
            "Publisher Retry-After was 300 seconds",
            "size: unknown",
        )?;
        if reply
            .pointer("/sources/0/notices/0/retry_after_seconds")
            .and_then(Value::as_u64)
            > Some(60)
        {
            return Err(format!("publisher delay replaced the local wait: {reply}"));
        }
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

/// Two first readers share the same complete pair of Orphadata files.
fn prevalence() -> Check {
    let (env, stub) = live(
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
    let read = || invoke(&env, get.clone(), 0);
    let (one, two) = std::thread::scope(|scope| {
        let other = scope.spawn(read);
        (read(), other.join())
    });
    let requests = stub.finish()?;
    for reply in [one?, two.map_err(|_| "second reader panicked")??] {
        if requests.len() != 2
            || reply.get("status") != Some(&json!("complete"))
            || reply.pointer("/result/items/0/prevalence/1/value") != Some(&json!("1.5"))
        {
            return Err(format!(
                "two first readers made {} requests: {reply}",
                requests.len()
            ));
        }
    }
    Ok(())
}
