//! Feature source-data-age: retained source dates survive paging and hosted restrictions.
use super::super::{
    launch::{Environment, fixtures},
    mcp,
    trial_get::command,
};
use serde_json::{Value, json};
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    vec![
        crate::runner::transcript::trial("source-data-age/restored-age", || {
            restored().map_err(Into::into)
        }),
        crate::runner::transcript::trial("source-data-age/hosted-stale-ema", || {
            hosted().map_err(Into::into)
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
    Ok(())
}
fn hosted() -> Check {
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
    drop(db);
    let mut globals = env.globals(None, true);
    globals.push("--hosted".into());
    let reply = mcp::call_with_globals(
        &env,
        &["search", "drug", "Glivec", "--source", "ema"].map(str::to_owned),
        false,
        &globals,
    )?;
    if reply.pointer("/sources/0/state") != Some(&json!("stale")) {
        return Err(format!("expected stale retained EMA: {reply}"));
    }
    dated_notice(&reply)?;
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
