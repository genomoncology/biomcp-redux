//! Features gene-constraint and disease-gene-validity: a first read stopped by the local size
//! or time limit names that limit and offers the sync that fills the bundle, never `syncing` (1247).
use super::{Check, Stub, configured, invoke, words};
use crate::runner::{configuration::cached, launch::fixtures, transcript};
use libtest_mimic::Trial;
use serde_json::{Value, json};

const CONSTRAINT: &[&str] = &[
    "get",
    "gene",
    "TP53",
    "constraint",
    "--constraint-release",
    "v4.1",
    "--section-only",
];
const CONSTRAINT_SYNC: &[&str] = &["sync", "gnomad-constraint-v4.1"];
const CONSTRAINT_REPLAY: &str = "gene/constraint/release-selection";
const VALIDITY: &[&str] = &[
    "get",
    "disease",
    "MONDO:0019391",
    "gene-validity",
    "--source",
    "gencc",
];
const GENCC_SYNC: &[&str] = &["sync", "gencc"];
const GENCC_REPLAY: &str = "disease/gene-validity";

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript::trial("gene-constraint/oversized-first-read-asks-for-sync", || {
            constraint_oversized().map_err(Into::into)
        }),
        transcript::trial("gene-constraint/timed-out-first-read-asks-for-sync", || {
            constraint_timed_out().map_err(Into::into)
        }),
        transcript::trial(
            "disease-gene-validity/oversized-first-read-asks-for-sync",
            || validity_oversized().map_err(Into::into),
        ),
        transcript::trial(
            "disease-gene-validity/timed-out-first-read-asks-for-sync",
            || validity_timed_out().map_err(Into::into),
        ),
    ]
}

/// The recorded HGNC files and gnomAD table, by URL path.
fn constraint_bodies() -> Result<Vec<(String, String)>, String> {
    let mut found = Vec::new();
    for folder in ["hgnc", "gnomad"] {
        found.extend(super::bodies(&format!("{CONSTRAINT_REPLAY}/{folder}"))?);
    }
    Ok(found)
}

fn served(bodies: &[(String, String)], request: &str) -> Result<String, String> {
    bodies
        .iter()
        .find(|(path, _)| request.contains(path.as_str()))
        .map(|(_, body)| body.clone())
        .ok_or_else(|| format!("unexpected request: {request}"))
}

/// The gnomAD table declares its published 95,546,041 bytes; HGNC is served whole.
fn constraint_oversized() -> Check {
    let bodies = constraint_bodies()?;
    let stub = Stub::with_headers(
        &["storage.googleapis.com"],
        |_| "text/tab-separated-values",
        move |request| served(&bodies, request),
        |request, bytes| {
            if request.contains("constraint_metrics") {
                95_546_041
            } else {
                bytes
            }
        },
        |_| 200,
        |_| String::new(),
    )?;
    let (env, stub) = configured(stub, json!({"auto_sync_max_bytes":1_000_000}))?;
    // A partial gene card exits 0.
    let first = invoke(&env, words(CONSTRAINT), 0)?;
    let again = invoke(&env, words(CONSTRAINT), 0)?;
    let fresh = invoke(&env, words(&[CONSTRAINT, &["--fresh"]].concat()), 0)?;
    let requests = stub.finish()?;
    let tables = requests
        .iter()
        .filter(|(request, _)| request.contains("constraint_metrics"))
        .count();
    if tables != 1 {
        return Err(format!("expected one first table fetch: {requests:?}"));
    }
    for reply in [&first, &again, &fresh] {
        limited(reply, "gnomad", "too-large", "size limit", CONSTRAINT_SYNC)?;
    }
    served_after_sync(
        &env,
        CONSTRAINT_REPLAY,
        CONSTRAINT_SYNC,
        CONSTRAINT,
        |reply| reply.to_string().contains("ENST00000269305"),
    )
}

/// The gnomAD publisher never answers, so the first read runs out of time.
fn constraint_timed_out() -> Check {
    let bodies = constraint_bodies()?;
    let stub = Stub::for_source_with_held_response(
        "storage.googleapis.com",
        "text/tab-separated-values",
        move |request| {
            if request.contains("constraint_metrics") {
                Ok(None)
            } else {
                served(&bodies, request).map(Some)
            }
        },
    )?;
    let (env, stub) = configured(stub, json!({"command_timeout_seconds":3}))?;
    // The read that spent its budget also reports the store busy; the next one fails at once.
    let first = invoke(&env, words(CONSTRAINT), 5)?;
    let again = invoke(&env, words(CONSTRAINT), 0)?;
    stub.finish()?;
    for reply in [&first, &again] {
        limited(
            reply,
            "gnomad",
            "deadline-exceeded",
            "ran out of time",
            CONSTRAINT_SYNC,
        )?;
    }
    Ok(())
}

fn validity_oversized() -> Check {
    let stub = Stub::with_length(
        "thegencc.org",
        |_| "text/html",
        |_| Ok(String::new()),
        |_, _| 27_143_351,
    )?;
    let (env, stub) = configured(stub, json!({"auto_sync_max_bytes":1_000_000}))?;
    let first = invoke(&env, words(VALIDITY), 4)?;
    let again = invoke(&env, words(VALIDITY), 4)?;
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!("expected one first fetch: {requests:?}"));
    }
    for reply in [&first, &again] {
        limited(reply, "gencc", "too-large", "size limit", GENCC_SYNC)?;
    }
    served_after_sync(&env, GENCC_REPLAY, GENCC_SYNC, VALIDITY, |reply| {
        reply
            .pointer("/page/returned")
            .and_then(Value::as_u64)
            .is_some_and(|rows| rows > 0)
    })
}

fn validity_timed_out() -> Check {
    let stub = Stub::for_source_with_held_response("thegencc.org", "text/html", |_| Ok(None))?;
    let (env, stub) = configured(stub, json!({"command_timeout_seconds":2}))?;
    let first = invoke(&env, words(VALIDITY), 5)?;
    let again = invoke(&env, words(VALIDITY), 4)?;
    stub.finish()?;
    for reply in [&first, &again] {
        limited(
            reply,
            "gencc",
            "deadline-exceeded",
            "ran out of time",
            GENCC_SYNC,
        )?;
    }
    Ok(())
}

/// The source failed for the named limit, a size refusal names no retry delay, and the first
/// follow-up is the bundle's sync.
fn limited(reply: &Value, source: &str, reason: &str, cause: &str, sync: &[&str]) -> Check {
    let outcome = reply
        .get("sources")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|outcome| outcome.get("source") == Some(&json!(source)))
        .ok_or_else(|| format!("missing {source} outcome: {reply}"))?;
    let notices = outcome
        .get("notices")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let notice = notices.iter().find(|notice| {
        notice
            .get("message")
            .and_then(Value::as_str)
            .is_some_and(|message| message.contains("first-read fetch") && message.contains(cause))
    });
    let delay = notice.and_then(|notice| notice.get("retry_after_seconds"));
    let delay_fits = if reason == "too-large" {
        delay.is_none()
    } else {
        delay.and_then(Value::as_u64).is_some_and(|delay| delay > 0)
    };
    if outcome.get("state") != Some(&json!("failed"))
        || outcome.get("reason") != Some(&json!(reason))
        || notice.is_none()
        || !delay_fits
        || reply.pointer("/next/0/args") != Some(&json!(sync))
    {
        return Err(format!(
            "expected {reason} and the sync call first: {reply}"
        ));
    }
    Ok(())
}

/// The offered sync fills the bundle, and the same read then answers.
fn served_after_sync(
    env: &crate::runner::launch::Environment,
    replay: &str,
    sync: &[&str],
    read: &[&str],
    answered: impl Fn(&Value) -> bool,
) -> Check {
    let recording = fixtures().join("replies").join(replay);
    let cache = env.root().join("case-cache");
    invoke(env, cached(sync, &cache, Some(&recording)), 0)?;
    let reply = invoke(env, cached(read, &cache, Some(&recording)), 0)?;
    if reply.get("status") != Some(&json!("complete")) || !answered(&reply) {
        return Err(format!("the sync did not serve the read: {reply}"));
    }
    Ok(())
}
