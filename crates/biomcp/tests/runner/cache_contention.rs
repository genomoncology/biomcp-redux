//! A checked provider reply survives a competing cache writer.
use super::{
    configuration::{cached, invoke, root},
    launch::Environment,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "cache/checked-gene-survives-write-contention",
        || checked_gene().map_err(Into::into),
    )]
}

fn checked_gene() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("case");
    let args = cached(&["get", "gene", "BRAF"], &case, Some(&root()));
    let initial = invoke(&env, args.clone(), 0)?;
    let db = rusqlite::Connection::open(case.join("replay.sqlite")).map_err(|e| e.to_string())?;
    db.execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| e.to_string())?;
    let answer = invoke(&env, args, 0)?;
    if answer.pointer("/result/items") != initial.pointer("/result/items") {
        return Err(format!(
            "cache contention changed the checked gene: {answer}"
        ));
    }
    let sources = answer
        .get("sources")
        .and_then(Value::as_array)
        .ok_or("sources missing")?;
    let mygene = sources
        .iter()
        .find(|s| s.get("source") == Some(&json!("mygene")))
        .ok_or("mygene outcome missing")?;
    if mygene.get("state") != Some(&json!("ok"))
        || mygene.pointer("/notices/0/kind") != Some(&json!("cache_problem"))
    {
        return Err(format!("checked source lost its cache problem: {answer}"));
    }
    Ok(())
}
