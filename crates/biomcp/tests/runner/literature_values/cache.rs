//! Stored native author snapshots must survive a change of adapter representation.
use super::{Check, Value, json};
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
        accepted(capture, &after)?;
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

fn accepted(capture: &str, value: &Value) -> Check {
    if capture == "author/papers" {
        let paper = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let problems = json!([{
            "source":"semantic-scholar", "record":paper,
            "field":"open_access_pdf.url", "code":"value.unknown"
        }]);
        // The retained native row has an empty optional PDF URL. The old mapper
        // omitted it silently; the current mapper also discloses that refusal.
        if value.get("status") != Some(&json!("partial"))
            || value.get("problems") != Some(&problems)
            || value.pointer("/result/author") != Some(&json!("semantic-scholar:123"))
            || value.pointer("/result/items/0/header/paper_id") != Some(&json!(paper))
            || value
                .pointer("/result/items/0/open_access_pdf/url")
                .is_some()
            || value.pointer("/result/items/0/open_access_pdf/status") != Some(&json!("CLOSED"))
            || value.pointer("/sources/0/state") != Some(&json!("ok"))
        {
            return Err(format!(
                "Previous papers payload lost field isolation or identity: {value}"
            )
            .into());
        }
    } else if value.get("status") != Some(&json!("complete"))
        || value.get("problems").is_some()
        || value.pointer("/result/items/0/id") != Some(&json!("semantic-scholar:2162869982"))
        || value.pointer("/result/items/1/id") != Some(&json!("semantic-scholar:113751091"))
    {
        return Err(format!("Previous author candidates changed: {value}").into());
    }
    Ok(())
}
