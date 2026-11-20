//! A drug lookup that could not read another source's names, or a stale market copy, never says
//! the input is wrong or the record absent; it names the source and offers its step (1087).
use crate::runner::{launch::Environment, response_cases::copy_capture, trial_get::command};
use serde_json::{Value, json};
use std::path::Path;

/// With MyChem current each name is absent from openFDA labels; with MyChem's cached replies
/// expired and MyChem unreachable, the lookup fails naming MyChem and offers the network retry.
/// "Orphan code" has no other source record once MyChem is unread; "Ambiguous therapy" still
/// has ChEMBL candidates from MyChem's stale copy.
pub(super) fn other_names() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let current = env.root().join("current");
    let down = env.root().join("down");
    copy_capture("drug/source-aliases", &current, |_, _| {}).map_err(|e| e.to_string())?;
    copy_capture("drug/source-aliases", &down, |source, record| {
        if source == "mychem" {
            unreachable(record);
        }
    })
    .map_err(|e| e.to_string())?;
    for name in ["Orphan code", "Ambiguous therapy"] {
        let args = [
            "get",
            "drug",
            name,
            "label",
            "--label-section",
            "boxed_warning",
            "--source",
            "openfda-label",
        ]
        .map(str::to_owned);
        let today = command(&env, &args, &env.globals(Some(&current), false), 1)?;
        if today.get("status") != Some(&json!("absent")) {
            return Err(format!("{name} with every source current: {today}"));
        }
        expire_mychem(env.root())?;
        let reply = command(&env, &args, &env.globals(Some(&down), false), 4)?;
        unread(&reply, "mychem", "fresh")?;
    }
    Ok(())
}

/// A market read by name from a stale EMA copy that holds no such name is a failed row naming
/// EMA, with the sync step, not a current absence.
pub(super) fn market() -> Result<(), String> {
    let env = super::stale_ema()?;
    let args = ["get", "drug", "zzznotadrug", "markets", "--source", "ema"].map(str::to_owned);
    let reply = command(&env, &args, &env.globals(None, true), 4)?;
    if reply.pointer("/result/markets/0/state") != Some(&json!("failed")) {
        return Err(format!("stale EMA market row read as no record: {reply}"));
    }
    unread(&reply, "ema", "sync")
}

/// Every recorded exchange becomes a server error, as when the source is unreachable.
fn unreachable(record: &mut Value) {
    if let Some(exchanges) = record.get_mut("exchanges").and_then(Value::as_array_mut)
        && let Some(mut failure) = exchanges.first().cloned()
    {
        failure["status"] = json!(500);
        *exchanges = vec![failure.clone(), failure.clone(), failure];
    }
}

fn expire_mychem(root: &Path) -> Result<(), String> {
    let db = rusqlite::Connection::open(root.join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE reply SET fresh_until=0 WHERE source='mychem'", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// The reply is neither a usage refusal nor an absence, lists the source as stale or failed,
/// and offers a step of the given kind.
fn unread(reply: &Value, source: &str, step: &str) -> Result<(), String> {
    let rows = |key: &str| {
        reply
            .get(key)
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    };
    let named = rows("sources").iter().any(|s| {
        s.get("source") == Some(&json!(source))
            && matches!(
                s.get("state").and_then(Value::as_str),
                Some("stale" | "failed")
            )
    });
    let offered = rows("next")
        .iter()
        .any(|s| s.get("kind") == Some(&json!(step)));
    let status = reply.get("status").and_then(Value::as_str);
    if matches!(status, Some("usage" | "absent")) || !named || !offered {
        return Err(format!(
            "lookup without a current {source} must fail naming it and offer {step}: {reply}"
        ));
    }
    Ok(())
}
