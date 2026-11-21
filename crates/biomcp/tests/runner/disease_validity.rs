//! Feature disease-gene-validity: curations by native disease name or ID.
use super::{
    launch::{Environment, fixtures},
    transcript::trial,
    trial_get::command,
};
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    vec![trial("disease-gene-validity/source-assertions", || {
        check().map_err(Into::into)
    })]
}

fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = fixtures().join("replies/disease/gene-validity");
    let globals = env.globals(Some(&replay), false);
    command(&env, &["sync".into(), "gencc".into()], &globals, 0)?;
    let combined = ["get", "disease", "MONDO:0019391", "gene-validity"];
    let reply = read(&env, &globals, &combined)?;
    assert_fields(
        &reply,
        &[
            ("/status", json!("complete")),
            ("/page/total", json!(4)),
            ("/sources/0/source", json!("clingen")),
            ("/sources/0/count", json!(2)),
            ("/sources/1/source", json!("gencc")),
            ("/sources/1/count", json!(2)),
            ("/result/items/0/source", json!("clingen")),
            ("/result/items/0/gene_id", json!("HGNC:7127")),
            ("/result/items/0/assertion/classification", json!("Definitive")),
            ("/result/items/0/assertion/mode_of_inheritance", json!("AD")),
            ("/result/items/0/assertion/expert_panel", json!("Fixture Expert Panel")),
            ("/result/items/2/source", json!("gencc")),
            ("/result/items/2/gene_id", json!("HGNC:7127")),
            ("/result/items/2/assertion/classification", json!("Strong")),
            ("/result/items/2/assertion/mode_of_inheritance", json!("Autosomal dominant")),
            ("/result/items/2/assertion/submitter", json!("Fixture Submitter")),
        ],
    )?;
    let filtered = read(
        &env,
        &globals,
        &[
            "get", "disease", "Fixture disease", "gene-validity", "--source", "clingen",
            "--classification", "limited", "--submitter", "Other Expert Panel",
        ],
    )?;
    assert_fields(
        &filtered,
        &[
            ("/page/total", json!(1)),
            ("/result/items/0/gene_id", json!("HGNC:7325")),
            ("/sources/0/count", json!(1)),
        ],
    )?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute_batch(
        "UPDATE bundle SET fresh_until=0 WHERE name='gencc-submissions';
         UPDATE bundle_content SET retrieved_at=1704067200 WHERE name='gencc-submissions';",
    )
    .map_err(|e| e.to_string())?;
    let stale = read(&env, &env.globals(None, true), &combined)?;
    assert_fields(
        &stale,
        &[
            ("/status", json!("partial")),
            ("/sources/1/state", json!("stale")),
            ("/result/coverage/1/retrieved", json!("2024-01-01")),
            ("/next/0/args", json!(["sync", "gencc"])),
        ],
    )
}

fn read(env: &Environment, globals: &[String], args: &[&str]) -> Result<Value, String> {
    command(env, &args.iter().map(|arg| (*arg).into()).collect::<Vec<_>>(), globals, 0)
}

fn assert_fields(reply: &Value, expected: &[(&str, Value)]) -> Result<(), String> {
    for (path, value) in expected {
        if reply.pointer(path) != Some(value) {
            return Err(format!("Expected {path} to be {value}: {reply}"));
        }
    }
    Ok(())
}
