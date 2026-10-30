//! Changed typed owners remap raw cache replies while unrelated snapshots keep hitting.
use super::{Check, Value, json};
use crate::runner::launch::{Environment, arguments, fixtures};
use std::path::Path;

pub(super) fn check(previous: &Path) -> Check {
    let env = Environment::new()?;
    let mut untouched = None;
    for (capture, words) in [
        (
            "author/synthetic",
            vec!["search", "author", "Louis Williams"],
        ),
        (
            "author/papers",
            vec!["author", "papers", "semantic-scholar:123", "details"],
        ),
        (
            "pathway/native",
            vec!["search", "pathway", "MAPK signaling"],
        ),
        (
            "orcid/visibility",
            vec!["author", "papers", "orcid:0000-0002-4299-3018"],
        ),
    ] {
        let args: Vec<_> = words.into_iter().map(String::from).collect();
        let replay = fixtures().join("replies").join(capture);
        let before = command(
            &env,
            previous,
            &arguments(&args, &env.globals(Some(&replay), false)),
            true,
        )?;
        let pool = pool_id(&before)?;
        let original = snapshot(&env, &pool)?;
        let retained = rows(&env)?;
        let after = if capture != "author/synthetic" {
            remap(&env, capture, &args, &pool, &original.2)?
        } else {
            let mut selected = args;
            selected.extend(["--pool".into(), pool.clone()]);
            let after = current(&env, &selected, true)?;
            untouched = Some((selected, pool.clone(), after.clone()));
            for pointer in [
                "/result",
                "/page/pool/id",
                "/page/pool/created",
                "/page/pool/expires",
            ] {
                if before.pointer(pointer) != after.pointer(pointer) {
                    return Err(
                        format!("Untouched {capture} snapshot changed {pointer}: {after}").into(),
                    );
                }
            }
            after
        };
        identities(capture, &before, &after)?;
        if snapshot(&env, &pool)? != original {
            return Err(format!("Previous {capture} pool metadata or payload changed").into());
        }
        preserved(&retained, &rows(&env)?)?;
        accepted(capture, &after)?;
    }
    let (args, pool, before) = untouched.ok_or("missing untouched owner witness")?;
    let after = current(&env, &args, true)?;
    if pool_id(&after)? != pool || before.get("result") != after.get("result") {
        return Err("untouched author owner stopped hitting after the owner remaps".into());
    }
    Ok(())
}

fn remap(
    env: &Environment,
    capture: &str,
    args: &[String],
    pool: &str,
    payload: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let old: Value = serde_json::from_str(payload)?;
    let (pointer, namespace, id, old_id, miss) = contract(capture);
    let external = old
        .pointer(pointer)
        .ok_or("missing previous mapped identifier")?;
    let old_namespace = match capture {
        "pathway/native" => external.get("source"),
        "orcid/visibility" => external.get("kind"),
        _ => None,
    };
    if external.get("namespace").is_some()
        || external.get(old_id) != Some(&json!(id))
        || capture != "author/papers" && old_namespace != Some(&json!(namespace))
    {
        return Err(
            format!("previous binary must retain the old mapped {capture} identifier").into(),
        );
    }
    let mut selected = args.to_vec();
    selected.extend(["--pool".into(), pool.into()]);
    let refused = current(env, &selected, false)?;
    if refused.pointer("/error/code") != Some(&json!(miss))
        || refused.pointer("/next/0/kind") != Some(&json!("restart"))
    {
        return Err(format!("Old typed selection did not miss cleanly: {refused}").into());
    }
    // Offline success after an owner-key miss requires remapping the existing
    // raw reply. No replay directory or provider request is available here.
    let after = current(env, args, true)?;
    let new_pool = pool_id(&after)?;
    let mapped: Value = serde_json::from_str(&snapshot(env, &new_pool)?.2)?;
    let external = mapped
        .pointer(pointer)
        .ok_or("missing remapped identifier")?;
    if new_pool == pool
        || external.get("namespace") != Some(&json!(namespace))
        || external.get("id") != Some(&json!(id))
    {
        return Err(format!("Old typed {capture} payload was not remapped: {after}").into());
    }
    // The newly mapped owner must itself hit on the next offline selection.
    selected.truncate(args.len());
    selected.extend(["--pool".into(), new_pool.clone()]);
    let hit = current(env, &selected, true)?;
    if hit.get("result") != after.get("result") || pool_id(&hit)? != new_pool {
        return Err("remapped owner snapshot did not hit".into());
    }
    Ok(after)
}

fn contract(capture: &str) -> (&str, &str, &str, &str, &str) {
    match capture {
        "author/papers" => (
            "/0/0/data/0/detail/header/external_ids/0",
            "CorpusId",
            "CorpusId:123",
            "id",
            "usage.pool_mismatch",
        ),
        "pathway/native" => (
            "/0/items/0/id",
            "reactome",
            "R-HSA-9652817",
            "id",
            "pool.expired",
        ),
        "orcid/visibility" => (
            "/0/0/assertions/0/external_ids/0",
            "pmid",
            "123",
            "value",
            "usage.pool_mismatch",
        ),
        _ => unreachable!("snapshot owner is declared by check"),
    }
}

fn identities(capture: &str, before: &Value, after: &Value) -> Check {
    let (items, pointer) = match capture {
        "author/papers" => ("/result/items", "/header/paper_id"),
        "pathway/native" => ("/result/items", "/id/id"),
        "orcid/visibility" => ("/result/groups", "/assertions"),
        _ => ("/result/items", "/id"),
    };
    let ids = |value: &Value| -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        value
            .pointer(items)
            .and_then(Value::as_array)
            .ok_or("missing retained items")?
            .iter()
            .map(|item| {
                let identity = item.pointer(pointer).ok_or("missing retained identity")?;
                if capture == "orcid/visibility" {
                    let assertions = identity.as_array().ok_or("missing retained assertions")?;
                    Ok(Value::Array(
                        assertions.iter().map(|a| a["put_code"].clone()).collect(),
                    ))
                } else {
                    Ok(identity.clone())
                }
            })
            .collect()
    };
    if ids(before)? != ids(after)?
        || before.pointer("/result/author") != after.pointer("/result/author")
    {
        return Err(
            format!("Previous {capture} identities, order or count changed: {after}").into(),
        );
    }
    Ok(())
}

fn current(
    env: &Environment,
    args: &[String],
    success: bool,
) -> Result<Value, Box<dyn std::error::Error>> {
    command(
        env,
        Path::new(env!("CARGO_BIN_EXE_biomcp")),
        &arguments(args, &env.globals(None, true)),
        success,
    )
}

fn command(
    env: &Environment,
    program: &Path,
    args: &[String],
    success: bool,
) -> Result<Value, Box<dyn std::error::Error>> {
    let output = env.command(program, args).output()?;
    if output.status.success() != success {
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
        return Err("previous snapshot proof requires query schema version 6".into());
    }
    Ok(db.query_row(
        "SELECT COALESCE(p.created,0),s.expires,s.payload FROM trial_snapshot s LEFT JOIN pool p ON s.id=p.id WHERE s.id=?1",
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
        // The current parent already stores the field refusal beside mapped
        // metadata. Remapping must preserve that refusal and the valid sibling.
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
    } else if capture == "author/synthetic"
        && (value.get("status") != Some(&json!("complete"))
            || value.get("problems").is_some()
            || value.pointer("/result/items/0/id") != Some(&json!("semantic-scholar:2162869982"))
            || value.pointer("/result/items/1/id") != Some(&json!("semantic-scholar:113751091")))
    {
        return Err(format!("Previous author candidates changed: {value}").into());
    }
    Ok(())
}

// Retain all existing rows, including transport replies and any bundle rows.
// New owner keys may add rows but must never reset or replace old payloads.
type Rows = Vec<(String, Vec<Vec<rusqlite::types::Value>>)>;
fn rows(env: &Environment) -> Result<Rows, Box<dyn std::error::Error>> {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))?;
    let names = db.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?
        .query_map([], |row| row.get::<_, String>(0))?.collect::<Result<Vec<_>, _>>()?;
    let mut result = Vec::new();
    for name in names {
        let mut statement =
            db.prepare(&format!("SELECT * FROM \"{}\"", name.replace('"', "\"\"")))?;
        let count = statement.column_count();
        let values = statement
            .query_map([], |row| {
                (0..count)
                    .map(|i| row.get(i))
                    .collect::<Result<Vec<_>, _>>()
            })?
            .collect::<Result<Vec<_>, _>>()?;
        result.push((name, values));
    }
    Ok(result)
}
fn preserved(before: &Rows, after: &Rows) -> Check {
    for (name, rows) in before {
        let current = after
            .iter()
            .find(|(table, _)| table == name)
            .ok_or("stored table dropped")?;
        if rows.iter().any(|row| !current.1.contains(row)) || name == "reply" && rows != &current.1
        {
            return Err(format!("Stored {name} rows changed during owner-key remap").into());
        }
    }
    Ok(())
}
