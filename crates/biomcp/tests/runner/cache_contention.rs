//! A checked provider reply survives a competing cache writer.
use super::{configuration::root, launch::Environment, mcp::request, publication::Peer};
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
    let records = env.root().join("records");
    recordings(&root(), &records)?;
    let mut peer = Peer::with_globals(&env, &env.globals(Some(&records), false))?;
    let frame = request(
        json!(1),
        "tools/call",
        json!({
            "name":"biomcp", "arguments":{"args":["get","gene","BRAF"]}
        }),
    );
    peer.send(&frame)?;
    let initial = content(peer.next()?)?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute_batch("BEGIN IMMEDIATE")
        .map_err(|e| e.to_string())?;
    peer.send(&frame)?;
    let answer = content(peer.next()?)?;
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
    db.execute_batch("ROLLBACK").map_err(|e| e.to_string())?;
    peer.finish()
}

fn content(reply: Value) -> Result<Value, String> {
    if reply.pointer("/result/isError") != Some(&json!(false)) {
        return Err(format!("checked gene command failed: {reply}"));
    }
    reply
        .pointer("/result/structuredContent")
        .cloned()
        .ok_or("content missing".into())
}

fn recordings(from: &std::path::Path, to: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for entry in std::fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let target = to.join(entry.file_name());
        if path.is_dir() {
            recordings(&path, &target)?;
        } else if path.extension().is_some_and(|v| v == "json") {
            let mut value: Value =
                serde_json::from_slice(&std::fs::read(&path).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            if let Some(exchanges) = value.get_mut("exchanges").and_then(Value::as_array_mut) {
                let copy = exchanges.clone();
                exchanges.extend(copy);
            }
            std::fs::write(
                target,
                serde_json::to_vec(&value).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
