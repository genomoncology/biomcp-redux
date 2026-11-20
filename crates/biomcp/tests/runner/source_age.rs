//! Feature source-data-age: retained source dates survive paging and hosted restrictions.
#[path = "source_age/name_recovery.rs"]
mod name_recovery;
#[path = "source_age/named_calls.rs"]
mod named_calls;
#[path = "source_age/names.rs"]
mod names;
#[path = "source_age/oncokb.rs"]
mod oncokb;
use super::super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp,
    trial_get::command,
};
use serde_json::{Value, json};
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    vec![
        crate::runner::transcript::trial("source-data-age/oncokb-release-date", || {
            oncokb::check().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/restored-age", || {
            restored().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/hosted-stale-ema", || {
            hosted().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/stale-ema-market", || {
            market().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/stale-name-not-found", || {
            unfound().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/stale-chembl-name", || {
            chembl_unfound().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/capped-alias", || {
            name_recovery::capped_alias().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/capped-product", || {
            name_recovery::capped_product().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/short-chembl-copy", || {
            name_recovery::short_chembl_copy().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/deadline", || {
            name_recovery::deadline().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/label-headings", || {
            name_recovery::label_headings().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/chembl-match", || {
            name_recovery::chembl_match().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/chembl-sections", || {
            name_recovery::chembl_sections().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/name-recovery/market-retry", || {
            name_recovery::market_retry().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/reported-drug-names/sti", || {
            named_calls::sti().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/reported-drug-names/markets", || {
            named_calls::markets().map_err(Into::into)
        }),
        crate::runner::transcript::trial(
            "source-data-age/reported-drug-names/blarcamesine",
            || named_calls::blarcamesine().map_err(Into::into),
        ),
        crate::runner::transcript::trial("source-data-age/unread-other-names", || {
            names::other_names().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/stale-market-unfound", || {
            names::market().map_err(Into::into)
        }),
    ]
}
fn restored() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"source_data_max_age_days":1000000}))
        .map_err(|e| e.to_string())?;
    let args = ["get", "trial", "NCT03734029", "locations", "--limit", "1"].map(str::to_owned);
    let first = command(
        &env,
        &args,
        &env.globals(Some(&fixtures().join("replies/trial/country")), false),
        0,
    )?;
    let next = super::super::action(&first, "page")?;
    let original = dated_notice(&first)?;
    if original.to_string().contains("Old data") {
        return Err(format!("large age limit ignored: {first}"));
    }
    env.config(&json!({"source_data_max_age_days":0}))
        .map_err(|e| e.to_string())?;
    let restored = command(&env, &next, &env.globals(None, true), 0)?;
    let reassessed = dated_notice(&restored)?;
    if original.get("data_date") != reassessed.get("data_date")
        || !reassessed.to_string().contains("Old data")
    {
        return Err(format!(
            "stored date was lost or its age was not reassessed: {restored}"
        ));
    }
    legacy_snapshot(&env)?;
    let legacy = command(&env, &next, &env.globals(None, true), 0)?;
    let recovered = dated_notice(&legacy)?;
    if original.get("data_date") != recovered.get("data_date")
        || !recovered.to_string().contains("Old data")
    {
        return Err(format!(
            "older snapshot lost its retained publisher date: {legacy}"
        ));
    }
    Ok(())
}
fn legacy_snapshot(env: &Environment) -> Check {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    let payload: String = db
        .query_row("SELECT payload FROM trial_snapshot", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let mut payload: Value = serde_json::from_str(&payload).map_err(|e| e.to_string())?;
    if let Some(sources) = payload.get_mut("sources").and_then(Value::as_array_mut) {
        for source in sources {
            if let Some(notices) = source.get_mut("notices").and_then(Value::as_array_mut) {
                notices.retain(|notice| notice.get("data_date").is_none());
            }
        }
    }
    db.execute(
        "UPDATE trial_snapshot SET payload=?1",
        [payload.to_string()],
    )
    .map_err(|e| e.to_string())?;
    db.execute("DELETE FROM reply", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}
/// A stored EMA copy past its refresh date.
fn stale_ema() -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    command(
        &env,
        &["sync".into(), "ema".into()],
        &env.globals(Some(&fixtures().join("replies/drug/ema-synthetic")), false),
        0,
    )?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE bundle SET fresh_until=0 WHERE name='ema'", [])
        .map_err(|e| e.to_string())?;
    Ok(env)
}
/// A market read by name from a stale EMA copy finds its record and says the copy is stale,
/// as the call that reads every market does, not that EMA has no record (1081).
fn market() -> Check {
    let env = stale_ema()?;
    let args = ["get", "drug", "Glivec", "markets", "--source", "ema"].map(str::to_owned);
    let reply = command(&env, &args, &env.globals(None, true), 0)?;
    if reply.get("status") != Some(&json!("partial"))
        || reply.pointer("/result/markets/0/state") != Some(&json!("found"))
        || reply.pointer("/sources/0/state") != Some(&json!("stale"))
    {
        return Err(format!(
            "stale EMA market was not a partial found row: {reply}"
        ));
    }
    Ok(())
}
/// A name a stale lookup cannot find is a failed read naming the stale source, never a wrong
/// input: a stale EMA copy, and expired ChEMBL replies with no network (1081).
fn unfound() -> Check {
    let env = stale_ema()?;
    let args = [
        "get",
        "drug",
        "zzznotadrug",
        "regulatory",
        "--source",
        "ema",
    ]
    .map(str::to_owned);
    stale_failure(
        &command(&env, &args, &env.globals(None, true), 4)?,
        "ema",
        Some("failed"),
    )
}
fn chembl_unfound() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["get", "drug", "EX-66", "--source", "chembl"].map(str::to_owned);
    let replay = fixtures().join("replies/drug/source-aliases");
    command(&env, &args, &env.globals(Some(&replay), false), 0)?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE reply SET fresh_until=0", [])
        .map_err(|e| e.to_string())?;
    // Expired replies with no network answer from the stale copy, or fail naming it.
    let mut all = args.to_vec();
    all.extend(env.globals(None, true));
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &all)
        .output()
        .map_err(|e| e.to_string())?;
    let reply: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if output.status.code() == Some(2) || reply.get("status") == Some(&json!("usage")) {
        return Err(format!(
            "stale ChEMBL lookup answered as wrong input: {reply}"
        ));
    }
    stale_failure(&reply, "chembl", None)
}
fn stale_failure(reply: &Value, source: &str, status: Option<&str>) -> Check {
    let stale = reply
        .get("sources")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|s| {
            s.get("source") == Some(&json!(source)) && s.get("state") == Some(&json!("stale"))
        });
    if !stale || status.is_some_and(|status| reply.get("status") != Some(&json!(status))) {
        return Err(format!(
            "stale lookup lost its stale {source} source: {reply}"
        ));
    }
    Ok(())
}
fn hosted() -> Check {
    let env = stale_ema()?;
    let globals = env.globals(None, true);
    let (reply, _) = mcp::call_text_hosted(
        &env,
        &["search", "drug", "Glivec", "--source", "ema"].map(str::to_owned),
        false,
        &globals,
    )?;
    // A hosted pinned snapshot answers complete and states the local copy's date.
    recovery(&reply, "ok")?;
    if !reply.to_string().contains("Local copy retrieved ") {
        return Err(format!("hosted reply omits the local copy date: {reply}"));
    }
    let server = Server::configured_globals("127.0.0.1:0", None, None, None, None, &globals)?;
    let frame = mcp::request(
        json!(1),
        "tools/call",
        json!({"name":"biomcp", "arguments":{"args":["search","drug","Glivec","--source","ema"]}}),
    );
    let (_, ordinary) = server.request("POST", "/mcp", &frame.to_string())?;
    recovery(
        ordinary
            .pointer("/result/structuredContent")
            .ok_or("HTTP source reply missing")?,
        "stale",
    )
}
fn recovery(reply: &Value, state: &str) -> Check {
    if reply.pointer("/sources/0/state") != Some(&json!(state)) {
        return Err(format!("expected {state} retained EMA: {reply}"));
    }
    dated_notice(reply)?;
    if reply
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|step| {
            step.get("args")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .any(|arg| arg == "sync")
        })
    {
        return Err(format!("hosted reply offers sync: {reply}"));
    }
    Ok(())
}
fn dated_notice(value: &Value) -> Result<&Value, String> {
    value
        .get("sources")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .flat_map(|source| {
            source
                .get("notices")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
        })
        .find(|notice| notice.get("data_date").is_some())
        .ok_or_else(|| format!("publisher date missing: {value}"))
}
