//! The scrub option uses the configured file without ordinary writable cache startup.
use super::launch::Environment;
use rusqlite::Connection;
use serde_json::Value;
use std::fs;

type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn run() -> Check {
    let env = Environment::new()?;
    let original = env.root().join(format!(
        "cache/biomcp/cache-{}-{}.sqlite",
        biomcp_fetch::CACHE_NAMESPACE_VERSION,
        biomcp_query::store::CACHE_NAMESPACE_VERSION
    ));
    let args = ["store".into(), "clean".into(), "--offline".into()];
    let initial = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    if !initial.status.success() {
        return Err("could not create synthetic store".into());
    }
    let db = Connection::open(&original)?;
    db.execute_batch(
        "PRAGMA wal_checkpoint(TRUNCATE); PRAGMA journal_mode=DELETE;
        INSERT INTO reply(key,source,url,status,body,fetched_at,fresh_until)
        VALUES (x'01','litsense','https://example.test/',200,x'5b5d',0,9999999999);",
    )?;
    drop(db);
    let before = fs::read(&original)?;
    let destination = env.root().join("checked-copy.sqlite");
    let args = vec![
        "store".into(),
        "clean".into(),
        "--scrub-to-copy".into(),
        destination.to_string_lossy().into_owned(),
        "--offline".into(),
    ];
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    let value: Value = serde_json::from_slice(&output.stdout)?;
    if !output.status.success() || value["status"] != "complete" || !destination.exists() {
        return Err(format!("scrub option failed: {value}").into());
    }
    if fs::read(&original)? != before {
        return Err("scrub changed source database".into());
    }
    // A normal Fetch startup would switch DELETE to WAL; the scrub must bypass it.
    let db = Connection::open(&original)?;
    let journal: String = db.pragma_query_value(None, "journal_mode", |row| row.get(0))?;
    if journal != "delete" {
        return Err("scrub opened the ordinary writable transport".into());
    }
    drop(db);
    let checked = fs::read(&destination)?;
    let second = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    if second.status.success() || fs::read(&destination)? != checked {
        return Err("scrub replaced an existing destination".into());
    }
    for globals in [vec![], vec!["--hosted".into()]] {
        let response = super::mcp::call_with_globals(&env, &args, true, &globals)?;
        if response.pointer("/error/code").and_then(Value::as_str) != Some("usage.mcp_unavailable")
        {
            return Err(format!("MCP exposed local scrub: {response}").into());
        }
    }
    Ok(())
}
