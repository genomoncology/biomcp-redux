//! Dynamic pool identifiers stay within the ordinary command launch boundary.
use super::{Step, assertions, launch::Environment, schema::Registry};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn check() -> Result<(), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let replay = crate::runner::launch::fixtures().join("replies/article/search/synthetic-small");
    let query = vec!["search", "article", "AKT1 E17K", "--limit", "2"];
    let first = run(
        &environment,
        &replay,
        &query,
        json!({"/status":"complete"}),
        0,
    )?;
    let old = pool_id(&first)?;
    let second = run(
        &environment,
        &replay,
        &["search", "article", "AKT1 E17K", "--limit", "2", "--fresh"],
        json!({"/status":"complete"}),
        0,
    )?;
    if first.pointer("/page/pool/id") == second.pointer("/page/pool/id") {
        return Err("fresh search reused the old snapshot".into());
    }
    let retained = run(
        &environment,
        &replay,
        &[
            "search",
            "article",
            "AKT1 E17K",
            "--limit",
            "2",
            "--pool",
            old,
        ],
        json!({"/status":"complete"}),
        0,
    )?;
    for pointer in ["/page/pool", "/result/items"] {
        if first.pointer(pointer) != retained.pointer(pointer) {
            return Err(format!("explicit old snapshot changed {pointer}"));
        }
    }
    let current = run(
        &environment,
        &replay,
        &query,
        json!({"/status":"complete"}),
        0,
    )?;
    if current.pointer("/page/pool") != second.pointer("/page/pool") {
        return Err("fresh search did not replace current snapshot".into());
    }
    run(
        &environment,
        &replay,
        &[
            "search",
            "article",
            "AKT1 E17K",
            "--limit",
            "2",
            "--pool",
            old,
            "--offset",
            "4294967295",
        ],
        json!({"/page/returned":0}),
        0,
    )?;
    run(
        &environment,
        &replay,
        &["search", "article", "different selectors", "--pool", old],
        json!({"/error/code":"usage.pool_mismatch"}),
        2,
    )?;
    Ok(())
}

fn pool_id(first: &Value) -> Result<&str, String> {
    let created = first
        .pointer("/page/pool/created")
        .and_then(Value::as_i64)
        .ok_or("missing created")?;
    let expires = first
        .pointer("/page/pool/expires")
        .and_then(Value::as_i64)
        .ok_or("missing expires")?;
    if expires.checked_sub(created) != Some(86_400) {
        return Err("snapshot did not retain its fixed lifetime".into());
    }
    first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing pool".into())
}

fn run(
    environment: &Environment,
    replay: &Path,
    args: &[&str],
    expect: Value,
    exit: i32,
) -> Result<Value, String> {
    let step: Step = serde_json::from_value(json!({"run":args, "expect":expect, "exit":exit}))
        .map_err(|e| e.to_string())?;
    let expectations = assertions::prepare(&step)?;
    super::run_step(
        &step,
        environment,
        &expectations,
        &Registry::embedded()?,
        Some(replay),
    )
}
