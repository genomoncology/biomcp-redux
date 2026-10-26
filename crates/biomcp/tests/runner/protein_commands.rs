//! Feature protein-search: dynamic pool bindings and lifetimes through the shipped command.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("protein-search/pool-binding-and-lifetime", || {
            pools().map_err(Into::into)
        }),
        Trial::test("protein-search/native-continuation-admission", || {
            links().map_err(|e| e.to_string().into())
        }),
        Trial::test("protein-search/row-cache-admission", || {
            cache_admission().map_err(|e| e.to_string().into())
        }),
    ]
}
fn call(env: &Environment, args: &[&str], exit: i32) -> Result<Value, String> {
    let replay = super::launch::fixtures().join("replies/protein/synthetic");
    let mut words = env.globals(Some(&replay), false);
    words.extend(args.iter().map(|s| (*s).into()));
    invoke(env, words, exit)
}
fn id(value: &Value) -> Result<&str, String> {
    value
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing protein pool".into())
}
fn pools() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["search", "protein", "--gene", "BRAF"];
    let first = call(&env, &args, 0)?;
    let pool = id(&first)?;
    let retained = call(
        &env,
        &["search", "protein", "--gene", "BRAF", "--pool", pool],
        0,
    )?;
    if first.pointer("/page/pool") != retained.pointer("/page/pool") {
        return Err("retained protein pool changed its publication metadata".into());
    }
    let fresh = call(&env, &["search", "protein", "--gene", "BRAF", "--fresh"], 0)?;
    if pool == id(&fresh)? {
        return Err("fresh protein search reused a published pool".into());
    }
    let mismatch = call(
        &env,
        &["search", "protein", "--gene", "EGFR", "--pool", pool],
        2,
    )?;
    if mismatch.pointer("/error/code").and_then(Value::as_str) != Some("usage.pool_mismatch") {
        return Err("changed protein selectors did not reject the old pool".into());
    }
    let terminal = call(
        &env,
        &[
            "search", "protein", "--gene", "BRAF", "--pool", pool, "--offset", "1",
        ],
        0,
    )?;
    if terminal.pointer("/page/returned").and_then(Value::as_u64) != Some(0) {
        return Err("terminal protein page returned a duplicate".into());
    }
    let invalid = call(
        &env,
        &[
            "search", "protein", "--gene", "BRAF", "--pool", pool, "--offset", "2",
        ],
        3,
    )?;
    if invalid.pointer("/next/0/kind").and_then(Value::as_str) != Some("restart") {
        return Err("invalid retained protein offset omitted restart".into());
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE trial_snapshot SET expires=0 WHERE id=?1", [pool])
        .map_err(|e| e.to_string())?;
    let expired = call(
        &env,
        &["search", "protein", "--gene", "BRAF", "--pool", pool],
        3,
    )?;
    if expired.pointer("/error/code").and_then(Value::as_str) != Some("pool.expired") {
        return Err("expired protein pool refetched instead of restarting".into());
    }
    Ok(())
}

fn links() -> Result<(), Box<dyn std::error::Error>> {
    for (before, after) in [
        ("rest.uniprot.org", "example.invalid"),
        ("%28EGFR%29", "%28OTHER%29"),
        ("size=5", "size=6"),
        (
            "cursor=native-second",
            "cursor=native-second&unexpected=field",
        ),
        ("uniprotkb/search", "other/search"),
    ] {
        super::response_cases::case(
            "protein/pages",
            &["search", "protein", "EGFR"],
            |_, record| {
                if record
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|u| !u.contains("cursor="))
                    && let Some(next) = record.pointer_mut("/exchanges/0/pagination/next")
                    && let Some(text) = next.as_str()
                {
                    *next = serde_json::json!(text.replace(before, after));
                }
            },
            serde_json::json!({"/status":"failed", "/sources/0/reason":"bad_reply"}),
            4,
        )?;
    }
    Ok(())
}

fn cache_admission() -> Result<(), Box<dyn std::error::Error>> {
    for (pointer, value) in [
        ("/organism/taxonId", serde_json::json!(10090)),
        (
            "/entryType",
            serde_json::json!("UniProtKB unreviewed (TrEMBL)"),
        ),
    ] {
        super::response_cases::checked_steps(
            "protein/pages",
            |_, record| {
                if record
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|u| !u.contains("cursor="))
                    && let Some(row) = record.pointer_mut("/exchanges/0/body/value/results/0")
                    && let Some(field) = row.pointer_mut(pointer)
                {
                    *field = value.clone();
                }
            },
            serde_json::json!([
                {"run":["search","protein","EGFR"], "exit":0,
                 "expect":{"/status":"partial", "/problems/0/code":"value_unknown"}},
                {"run":["search","protein","EGFR","--fresh"], "offline":true,"exit":4,
                 "expect":{"/status":"failed"}}
            ]),
        )?;
    }
    Ok(())
}
