//! A WHO name lookup needs a current copy to establish a miss.
use super::{Check, Environment, command, fixtures, stale_failure};
use crate::runner::action;
use serde_json::{Value, json};

pub(super) fn market() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/drug/markets");
    command(
        &env,
        &["sync".into(), "who-pq".into()],
        &env.globals(Some(&replay), false),
        0,
    )?;
    let miss = ["get", "drug", "imatinib", "markets", "--source", "who-pq"].map(str::to_owned);
    let found = [
        "get",
        "drug",
        "dolutegravir",
        "markets",
        "--source",
        "who-pq",
    ]
    .map(str::to_owned);
    let offline = env.globals(None, true);
    let current = command(&env, &miss, &offline, 1)?;
    expect_market(&current, "no-record")?;
    expire(&env, "UPDATE bundle SET fresh_until=0 WHERE name='who-pq'")?;

    let stale = command(&env, &miss, &offline, 4)?;
    stale_failure(&stale, "who-pq", Some("failed"))?;
    if action(&stale, "sync")? != ["sync", "who-pq"] {
        return Err(format!("WHO PQ miss lost its sync step: {stale}"));
    }

    let stale = command(&env, &found, &offline, 0)?;
    stale_failure(&stale, "who-pq", Some("partial"))?;
    expect_market(&stale, "found")?;

    // With a current EMA hit, the partial all-markets reply also shows the WHO row.
    command(
        &env,
        &["sync".into(), "ema".into()],
        &env.globals(Some(&replay), false),
        0,
    )?;
    let mixed = ["get", "drug", "imatinib", "markets"].map(str::to_owned);
    let mixed = command(&env, &mixed, &offline, 0)?;
    stale_failure(&mixed, "who-pq", Some("partial"))?;
    if mixed.pointer("/result/markets/2/state") != Some(&json!("failed"))
        || mixed.pointer("/result/markets/2/matches") != Some(&json!(0))
    {
        return Err(format!("WHO PQ lost its failed row or count: {mixed}"));
    }
    Ok(())
}

pub(super) fn essential_medicines() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/drug/who-eml");
    let recorded = env.globals(Some(&replay), false);
    let miss = ["get", "drug", "pradaxa", "essential-medicines"].map(str::to_owned);
    let found = ["get", "drug", "dabigatran", "essential-medicines"].map(str::to_owned);
    command(&env, &miss, &recorded, 1)?;
    command(&env, &found, &recorded, 0)?;
    expire(
        &env,
        "UPDATE reply SET fresh_until=0 WHERE source='who-eml'",
    )?;

    let offline = env.globals(None, true);
    let stale = command(&env, &miss, &offline, 4)?;
    stale_failure(&stale, "who-eml", Some("failed"))?;
    let fresh = action(&stale, "fresh")?;
    if !fresh.iter().any(|arg| arg == "--fresh") {
        return Err(format!("WHO eEML miss has no fresh read: {stale}"));
    }
    let stale_hit = command(&env, &found, &offline, 0)?;
    stale_failure(&stale_hit, "who-eml", Some("partial"))?;
    if stale_hit.pointer("/result/identity/id") != Some(&json!("479")) {
        return Err(format!("stale WHO eEML hit lost its medicine: {stale_hit}"));
    }

    let current = command(&env, &fresh, &recorded, 1)?;
    if current.get("query") != stale.get("query")
        || current.pointer("/sources/0/state") != Some(&json!("ok"))
    {
        return Err(format!(
            "WHO eEML fresh read did not repeat the lookup: {current}"
        ));
    }
    Ok(())
}

fn expire(env: &Environment, sql: &str) -> Check {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute(sql, []).map_err(|e| e.to_string())?;
    Ok(())
}

fn expect_market(reply: &Value, state: &str) -> Check {
    if reply.pointer("/result/markets/0/source") != Some(&json!("who-pq"))
        || reply.pointer("/result/markets/0/state") != Some(&json!(state))
    {
        return Err(format!("expected WHO PQ market state {state}: {reply}"));
    }
    Ok(())
}
