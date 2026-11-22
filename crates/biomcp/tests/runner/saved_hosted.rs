//! Hosted saved work (sdlc/planning/saved-work.md, Hosted points 2-3): a server started with
//! a saved-work folder keeps each account's file at `<folder>/<account>/saved.sqlite`, named
//! by the account header alone. A call without the header is refused, a header that could
//! name a path gets HTTP 400, no tool argument names a path, and one account never sees
//! another's names. A first search page offers a save step only to a call with an account.
//! A re-check runs a stored row's words under hosted policy, and only a first search page.
//! live_check: replayed trial search; no network.
use super::{http::Server, launch::fixtures};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        super::pool_contract::SAVED_FOLDER.0,
        || hosted().map_err(Into::into),
    )]
}

const HEADER: &str = "x-biomcp-account";

/// One tools/call, with these extra header lines; the HTTP status and the tool's JSON reply.
fn call(
    server: &Server,
    name: &str,
    arguments: Value,
    headers: &str,
) -> Result<(u16, Value), String> {
    let body = super::mcp::request(
        json!(1),
        "tools/call",
        json!({"name":name,"arguments":arguments}),
    );
    let (status, reply) =
        server.request_with_headers("POST", "/mcp", &body.to_string(), headers)?;
    let value = reply
        .pointer("/result/structuredContent")
        .cloned()
        .or_else(|| {
            reply
                .pointer("/result/content/0/text")
                .and_then(Value::as_str)
                .and_then(|text| serde_json::from_str(text).ok())
        })
        .unwrap_or(reply);
    Ok((status, value))
}

fn account(name: &str) -> String {
    format!("{HEADER}: {name}\r\n")
}

fn hosted() -> Result<(), String> {
    let folder = tempfile::tempdir().map_err(|error| error.to_string())?;
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"mcp_tool_mode":"typed","mcp_reply_form":"both"})),
        Some(fixtures().join("replies/trial/condition")),
        None,
        None,
        &[
            "--hosted".into(),
            "--saved-folder".into(),
            folder.path().to_string_lossy().into_owned(),
        ],
    )?;
    let list = super::mcp::request(json!(1), "tools/list", json!({}));
    let (_, tools) = server.request("POST", "/mcp", &list.to_string())?;
    let offered = tools
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .is_some_and(|tools| {
            tools
                .iter()
                .any(|tool| tool.get("name") == Some(&json!("saved")))
        });
    if !offered {
        return Err(format!(
            "a hosted server with a saved-work folder offers saved: {tools}"
        ));
    }
    let add = json!({"action":"list_add","list_add":{"name":"braf-genes","entity":"gene","ids":["BRAF"]}});
    let (_, refused) = call(&server, "saved", add.clone(), "")?;
    if refused.pointer("/error/code") != Some(&json!("usage.mcp_unavailable")) {
        return Err(format!(
            "without the account header saved is refused: {refused}"
        ));
    }
    if std::fs::read_dir(folder.path())
        .map_err(|e| e.to_string())?
        .next()
        .is_some()
    {
        return Err("a refused call writes nothing".into());
    }
    let (_, added) = call(&server, "saved", add, &account("alice"))?;
    if added.pointer("/result/lists/0/name") != Some(&json!("braf-genes")) {
        return Err(format!("alice's list add: {added}"));
    }
    if !folder.path().join("alice/saved.sqlite").is_file() {
        return Err("the account's file is <folder>/<account>/saved.sqlite".into());
    }
    let show = json!({"action":"show"});
    let (_, other) = call(&server, "saved", show.clone(), &account("bob"))?;
    if other.get("result") != Some(&json!({"searches":[],"lists":[]})) {
        return Err(format!("bob never sees alice's names: {other}"));
    }
    for bad in ["../alice", "alice/x", ".."] {
        let (status, _) = call(&server, "saved", show.clone(), &account(bad))?;
        if status != 400 {
            return Err(format!("account {bad:?} answered HTTP {status}, not 400"));
        }
    }
    let twice = format!("{}{}", account("alice"), account("bob"));
    if call(&server, "saved", show.clone(), &twice)?.0 != 400 {
        return Err("a repeated account header answers HTTP 400".into());
    }
    let path = json!({"action":"show","show":{"name":"braf-genes","path":"/tmp/other.sqlite"}});
    let (_, pathed) = call(&server, "saved", path, &account("alice"))?;
    if pathed.pointer("/error/option") != Some(&json!("path")) {
        return Err(format!("no saved argument names a path: {pathed}"));
    }
    save_step(&server)?;
    stored_rechecks(&server, folder.path())
}

/// The save step reaches a caller with an account, and runs as written.
fn save_step(server: &Server) -> Result<(), String> {
    let search = json!({"action":"search","search":{"condition":"melanoma"}});
    let (_, anonymous) = call(server, "trial", search.clone(), "")?;
    let save = |step: &Value| step.get("kind") == Some(&json!("save"));
    if steps(&anonymous).iter().any(save) {
        return Err("no save step without an account".into());
    }
    let (_, page) = call(server, "trial", search, &account("carol"))?;
    let step = steps(&page)
        .into_iter()
        .find(save)
        .ok_or_else(|| format!("a first search page offers save: {page}"))?;
    let name = step
        .get("name")
        .and_then(Value::as_str)
        .ok_or("save step tool")?;
    let arguments = step.get("arguments").cloned().unwrap_or_default();
    let (_, saved) = call(server, name, arguments, &account("carol"))?;
    if saved.pointer("/result/searches/0/args/1") != Some(&json!("trial")) {
        return Err(format!("the save step runs as written: {saved}"));
    }
    Ok(())
}

fn steps(reply: &Value) -> Vec<Value> {
    reply
        .get("next")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

/// Rows written straight into the file, as a tampered or foreign file could hold, are refused
/// by `changed` before any source read: a search hosted policy refuses, a non-search, and a
/// search on a source the call leaves out, which is refused as the live call is.
fn stored_rechecks(server: &Server, folder: &std::path::Path) -> Result<(), String> {
    let db = rusqlite::Connection::open(folder.join("carol/saved.sqlite"))
        .map_err(|error| error.to_string())?;
    let args: String = db
        .query_row("SELECT args FROM saved_search", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    let saved: Vec<String> = serde_json::from_str(&args).map_err(|e| e.to_string())?;
    let with = |extra: &[&str]| -> Vec<String> {
        saved
            .iter()
            .cloned()
            .chain(extra.iter().map(|word| (*word).to_owned()))
            .collect()
    };
    let carol = account("carol");
    let leaving_nci = format!("{carol}x-biomcp-exclude-sources: nci\r\n");
    let live = json!({"action":"search","search":{"condition":"melanoma","source":"nci"}});
    let (_, live) = call(server, "trial", live, &leaving_nci)?;
    let live_code = live
        .pointer("/error/code")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("a live search on a left-out source is refused: {live}"))?
        .to_owned();
    let rows = [
        (
            "fresh-trials",
            with(&["--fresh"]),
            &carol,
            "usage.mcp_unavailable",
        ),
        (
            "one-trial",
            vec!["get".into(), "trial".into(), "NCT02576665".into()],
            &carol,
            "usage.arguments",
        ),
        (
            "nci-trials",
            with(&["--source", "nci"]),
            &leaving_nci,
            live_code.as_str(),
        ),
    ];
    for (name, words, headers, code) in rows {
        db.execute(
            "INSERT INTO saved_search(id,project,name,args,schema,request_key,created) \
             SELECT ?1,project,?1,?2,schema,request_key,created FROM saved_search LIMIT 1",
            rusqlite::params![
                name,
                serde_json::to_string(&words).map_err(|e| e.to_string())?
            ],
        )
        .map_err(|error| error.to_string())?;
        let changed = json!({"action":"changed","changed":{"name":name}});
        let (_, reply) = call(server, "saved", changed, headers)?;
        if reply.pointer("/error/code") != Some(&json!(code)) {
            return Err(format!(
                "a stored {name} re-check is refused with {code}: {reply}"
            ));
        }
        let kept: i64 = db
            .query_row(
                "SELECT count(*) FROM saved_snapshot WHERE search=?1",
                [name],
                |row| row.get(0),
            )
            .map_err(|error| error.to_string())?;
        if kept != 0 {
            return Err(format!("a refused {name} re-check keeps no snapshot"));
        }
    }
    Ok(())
}
