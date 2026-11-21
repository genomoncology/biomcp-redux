//! Feature disease-gene-validity: the date and refresh action for a stale copy.
use super::{Environment, command, fixtures};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/disease/gene-validity");
    let globals = env.globals(Some(&replay), false);
    command(&env, &["sync".into(), "gencc".into()], &globals, 0)?;
    read(
        &env,
        &globals,
        &[
            "get",
            "disease",
            "MONDO:0019391",
            "gene-validity",
            "--source",
            "clingen",
        ],
    )?;
    stale_copy(&env)
}

const GET_ID: [&str; 4] = ["get", "disease", "MONDO:0019391", "gene-validity"];

fn stale_copy(env: &Environment) -> Result<(), String> {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    // Updating bundle freshness clears its stored content metadata.
    db.execute_batch(
        "CREATE TEMP TABLE validity_copy AS SELECT name,revision FROM bundle_content WHERE name='gencc-submissions';
         UPDATE bundle SET fresh_until=0 WHERE name='gencc-submissions';
         INSERT INTO bundle_content SELECT name,revision,1704067200 FROM validity_copy;
         DELETE FROM bundle_key WHERE kind=4 AND bundle IN('gencc-submissions','clingen-validity');",
    )
    .map_err(|e| e.to_string())?;
    let stale = read(env, &env.globals(None, true), &GET_ID)?;
    assert_fields(
        &stale,
        &[
            ("/status", json!("partial")),
            ("/sources/1/state", json!("stale")),
            ("/page/total", json!(4)),
            ("/result/coverage/1/retrieved", json!("2024-01-01")),
            ("/next/0/args", json!(["sync", "gencc"])),
        ],
    )
}

fn read(env: &Environment, globals: &[String], args: &[&str]) -> Result<Value, String> {
    command(
        env,
        &args.iter().map(|arg| (*arg).into()).collect::<Vec<_>>(),
        globals,
        0,
    )
    .map_err(|error| format!("{args:?}: {error}"))
}

fn assert_fields(reply: &Value, expected: &[(&str, Value)]) -> Result<(), String> {
    for (path, value) in expected {
        if reply.pointer(path) != Some(value) {
            return Err(format!("Expected {path} to be {value}: {reply}"));
        }
    }
    Ok(())
}
