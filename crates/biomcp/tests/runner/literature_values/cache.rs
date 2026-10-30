//! Stored native author snapshots must survive a change of adapter representation.
use super::{Check, Value};
use crate::runner::launch::{Environment, arguments, fixtures};
use std::path::Path;

pub(super) fn check(previous: &Path) -> Check {
    for (capture, words) in [
        (
            "author/synthetic",
            vec!["search", "author", "Louis Williams"],
        ),
        (
            "author/papers",
            vec!["author", "papers", "semantic-scholar:123", "details"],
        ),
    ] {
        let env = Environment::new()?;
        let args: Vec<_> = words.into_iter().map(String::from).collect();
        let replay = fixtures().join("replies").join(capture);
        let before = command(
            &env,
            previous,
            &arguments(&args, &env.globals(Some(&replay), false)),
        )?;
        let pool = pool_id(&before)?;
        let original = snapshot(&env, &pool)?;
        let mut offline = args;
        offline.extend(["--pool".into(), pool.clone()]);
        let after = command(
            &env,
            Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &arguments(&offline, &env.globals(None, true)),
        )?;
        for pointer in [
            "/result",
            "/page/pool/id",
            "/page/pool/created",
            "/page/pool/expires",
        ] {
            if before.pointer(pointer) != after.pointer(pointer) {
                return Err(
                    format!("Previous {capture} snapshot changed {pointer}: {after}").into(),
                );
            }
        }
        if snapshot(&env, &pool)? != original {
            return Err(format!("Previous {capture} pool metadata or payload changed").into());
        }
        if after.get("status").and_then(Value::as_str) != Some("complete") {
            return Err(format!("Previous {capture} snapshot was refused: {after}").into());
        }
    }
    Ok(())
}
fn command(
    env: &Environment,
    program: &Path,
    args: &[String],
) -> Result<Value, Box<dyn std::error::Error>> {
    let output = env.command(program, args).output()?;
    if !output.status.success() {
        return Err(format!(
            "Snapshot command failed: {}",
            String::from_utf8_lossy(&output.stdout)
        )
        .into());
    }
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn pool_id(value: &Value) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(id) = value.pointer("/page/pool/id").and_then(Value::as_str) {
        return Ok(id.to_owned());
    }
    // Native papers pages may omit total, which also omits PageInfo. Their
    // checked continuation still names the existing pool explicitly.
    value
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|step| step.get("args").and_then(Value::as_array))
        .flat_map(|args| args.windows(2))
        .find(|pair| pair.first().and_then(Value::as_str) == Some("--pool"))
        .and_then(|pair| pair.get(1))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| "old CLI did not publish a pool".into())
}
// Read the actual prior payload and query owner version so a schema reset or
// replacement pool cannot pass a same-version warm continuity proof.
fn snapshot(
    env: &Environment,
    pool: &str,
) -> Result<(i64, i64, String), Box<dyn std::error::Error>> {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))?;
    let version: u32 = db.query_row(
        "SELECT version FROM meta WHERE owner='biomcp-query'",
        [],
        |row| row.get(0),
    )?;
    if version != 6 {
        return Err("previous author proof requires query schema version 6".into());
    }
    Ok(db.query_row(
        "SELECT p.created,p.expires,s.payload FROM pool p JOIN trial_snapshot s ON s.id=p.id WHERE p.id=?1",
        [pool], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?)
}
