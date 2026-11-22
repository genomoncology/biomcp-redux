//! Feature disease-gene-validity: distinguish publisher backoff from local bundle state.
use super::super::{
    bundle_pinned::hosted, bundle_sync::database, configuration::invoke, mcp::request,
};
use super::{Check, configured, live, retry_peer, words};
use libtest_mimic::Trial;
use serde_json::{Value, json};

const READ: &[&str] = &[
    "get",
    "disease",
    "MONDO:0019391",
    "gene-validity",
    "--source",
    "gencc",
];

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::super::transcript::trial("disease-gene-validity/cold-publisher-backoff", || {
            cold().map_err(Into::into)
        }),
        super::super::transcript::trial("disease-gene-validity/serve-stale-gencc", || {
            stale().map_err(Into::into)
        }),
    ]
}

fn cold() -> Check {
    let (env, stub) = configured(retry_peer("thegencc.org")?, json!({}))?;
    let first = invoke(&env, words(READ), 4)?;
    let repeated = invoke(&env, words(READ), 4)?;
    // Expire only the local cooldown in the shared store without sleeping for a minute.
    database(&env)?
        .execute(
            "UPDATE bundle_first_read SET until=unixepoch()-1 WHERE name='gencc-submissions'",
            [],
        )
        .map_err(|e| e.to_string())?;
    let reopened = invoke(&env, words(READ), 4)?;
    let mut server = hosted(&env)?;
    let frame = request(
        json!("hosted"),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":READ}}),
    );
    let result = server.request_with_headers(
        "POST",
        "/mcp",
        &frame.to_string(),
        "x-biomcp-use: academic\r\n",
    );
    server.stop()?;
    let (_, hosted) = result?;
    let served = hosted
        .pointer("/result/structuredContent")
        .ok_or_else(|| format!("missing hosted reply: {hosted}"))?;

    for reply in [&first, &repeated, &reopened, served] {
        assert_backoff(reply)?;
    }
    let requests = stub.finish()?;
    if requests.len() != 1 {
        return Err(format!(
            "expected one publisher request across four reads: {requests:?}"
        ));
    }
    Ok(())
}

fn assert_backoff(reply: &Value) -> Check {
    let guidance = reply
        .pointer("/sources/0/notices")
        .and_then(Value::as_array);
    let retry = guidance.into_iter().flatten().any(|notice| {
        notice
            .get("retry_after_seconds")
            .and_then(Value::as_u64)
            .is_some_and(|delay| (240..=300).contains(&delay))
            && notice
                .get("message")
                .and_then(Value::as_str)
                .is_some_and(|message| message.to_lowercase().contains("retry"))
    });
    let has_sync = reply
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|step| step.get("kind") == Some(&json!("sync")));
    if reply.pointer("/sources/0/state") != Some(&json!("failed"))
        || reply.pointer("/sources/0/reason") != Some(&json!("rate-limited"))
        || !retry
        || has_sync
    {
        return Err(format!(
            "expected publisher backoff and retry guidance: {reply}"
        ));
    }
    Ok(())
}

fn stale() -> Check {
    let (env, stub) = live("thegencc.org", "disease/gene-validity/gencc", json!({}))?;
    invoke(&env, words(READ), 0)?;
    database(&env)?
        .execute(
            "UPDATE bundle SET fresh_until=0 WHERE name='gencc-submissions'",
            [],
        )
        .map_err(|e| e.to_string())?;
    let reply = invoke(&env, words(READ), 0)?;
    let requests = stub.finish()?;
    if reply.pointer("/sources/0/state") != Some(&json!("stale"))
        || reply.pointer("/page/returned") != Some(&json!(2))
        || requests.len() != 1
    {
        return Err(format!(
            "expected stored GenCC rows marked stale: {reply}; {requests:?}"
        ));
    }
    Ok(())
}
