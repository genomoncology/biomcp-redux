//! Features pathway-wikipathways and diagnostic-cdx: external locks retain typed failures.
use super::super::{diagnostic_cdx, launch::Environment, mcp::request, publication::Peer, wikipathways};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("wikipathways/held-store-lock-names-local-failure", || {
            locked(false).map_err(Into::into)
        }),
        Trial::test("diagnostic-cdx/held-store-lock-names-local-failure", || {
            locked(true).map_err(Into::into)
        }),
    ]
}

fn locked(cdx: bool) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = if cdx {
        diagnostic_cdx::seed(&env)?
    } else {
        wikipathways::seed(&env, wikipathways::catalog(true, false), wikipathways::catalog(false, false), 200)?
    };
    let mut peer = Peer::with_globals(&env, &env.globals(Some(&replay), false))?;
    let source = if cdx { "fda-cdx" } else { "wikipathways" };
    let sync = vec!["sync", source];
    let initial = call(&mut peer, &sync)?;
    if initial.pointer("/result/isError") != Some(&json!(false)) {
        return Err(format!("initial sync failed: {initial}"));
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute_batch("BEGIN IMMEDIATE").map_err(|e| e.to_string())?;
    if !cdx {
        let reader = call(&mut peer, &["search", "pathway", "Atlas", "--source", source])?;
        failure(&reader)?;
        if reader.pointer("/result/structuredContent/sources/0/state") != Some(&json!("ok")) {
            return Err(format!("reader lost its checked provider state: {reader}"));
        }
    }
    failure(&call(&mut peer, &sync)?)?;
    db.execute_batch("ROLLBACK").map_err(|e| e.to_string())?;
    peer.finish()
}

fn call(peer: &mut Peer, args: &[&str]) -> Result<Value, String> {
    peer.send(&request(json!(1), "tools/call", json!({"name":"biomcp", "arguments":{"args":args}})))?;
    peer.next()
}

fn failure(reply: &Value) -> Result<(), String> {
    if reply.pointer("/result/isError") != Some(&json!(true))
        || reply.pointer("/result/structuredContent/local_error") != Some(&json!("busy"))
        || reply.pointer("/result/structuredContent/status") != Some(&json!("failed"))
    {
        return Err(format!("held external writer lacks typed local failure: {reply}"));
    }
    Ok(())
}
