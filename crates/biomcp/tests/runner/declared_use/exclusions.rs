//! Feature mcp-hosted: a request's source list only narrows, and discovery and stored replies follow it.
use super::{ACADEMIC, CLINPGX, Environment, Server, Stub, call, disabled, source_ok};
use crate::runner::{key_registration::invoke, launch::fixtures, transcript::trial};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

type Check<T = ()> = Result<T, String>;
const HEADER: &str = "x-biomcp-exclude-sources";
const CARD: &[&str] = &["get", "adverse-event", "100"];
const HTTP: &str = "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\n";

pub(super) fn trials() -> Vec<Trial> {
    vec![
        trial("mcp-hosted/exclude/no-call-and-stored-replies", || {
            left_out().map_err(Into::into)
        }),
        trial("mcp-hosted/exclude/only-narrows", || {
            only_narrows().map_err(Into::into)
        }),
        trial("mcp-hosted/exclude/refuses-unknown-names", || {
            refusals().map_err(Into::into)
        }),
        trial("mcp-hosted/exclude/discovery-agrees", || {
            discovery().map_err(Into::into)
        }),
    ]
}

fn list(names: &str) -> String {
    format!("{HEADER}: {names}\r\n")
}

fn left_out() -> Check {
    let assets = Environment::new().map_err(|e| e.to_string())?;
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = calls.clone();
    let stub = Stub::for_source("api.clinpgx.org", "application/json", move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
        Ok(r#"{"status":"success","data":[]}"#.into())
    })?;
    stub.retain(assets.shared_root());
    let ca = assets.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca})),
        None,
        None,
        None,
        &["--hosted".into()],
    )?;
    let narrowed = format!("{ACADEMIC}{}", list("clinpgx"));
    let first = call(&server, CLINPGX, &narrowed, false)?;
    if !disabled(&first, "clinpgx") || calls.load(Ordering::SeqCst) != 0 {
        return Err(format!("left-out source was called: {first}"));
    }
    let served = call(&server, CLINPGX, ACADEMIC, false)?;
    if !source_ok(&served, "clinpgx") {
        return Err(format!("another caller lost the source: {served}"));
    }
    let offline: Vec<_> = CLINPGX.iter().copied().chain(["--offline"]).collect();
    let stored = call(&server, &offline, &narrowed, false)?;
    if !disabled(&stored, "clinpgx") {
        return Err(format!("stored reply ignored the list: {stored}"));
    }
    let retained = call(&server, &offline, ACADEMIC, false)?;
    if !source_ok(&retained, "clinpgx") {
        return Err(format!("the list changed the stored reply: {retained}"));
    }
    server.stop()?;
    stub.finish().map(|_| ())
}

fn only_narrows() -> Check {
    let mut server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"disable_sources":["clinpgx"]})),
        None,
        None,
        None,
        &["--hosted".into()],
    )?;
    let ddinter = [
        "get",
        "drug",
        "DDInter1",
        "interactions",
        "--source",
        "ddinter",
    ];
    for names in ["", "civic", "ddinter,civic", " civic , pubmed "] {
        let header = list(names);
        let operator = call(&server, CLINPGX, &format!("{ACADEMIC}{header}"), false)?;
        if !disabled(&operator, "clinpgx") {
            return Err(format!("{names:?} restored a server exclusion: {operator}"));
        }
        let restricted = call(&server, &ddinter, &header, false)?;
        if !disabled(&restricted, "ddinter") {
            return Err(format!("{names:?} widened declared use: {restricted}"));
        }
    }
    server.stop()
}

fn post(server: &Server, method: &str, params: Value, extra: &str) -> Check<(u16, Value)> {
    let frame = json!({"jsonrpc":"2.0","id":1,"method":method,"params":params});
    let (status, reply, _) = server.raw(
        "POST",
        "/mcp",
        &server.address.to_string(),
        &format!("{HTTP}{extra}"),
        &frame.to_string(),
    )?;
    Ok((status, reply.ok_or("missing reply")?))
}

fn refusals() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listed = invoke(&env, &["source", "list"], "", 0)?;
    if listed.pointer("/result/header") != Some(&json!(HEADER)) {
        return Err(format!("list names the wrong header: {listed}"));
    }
    let names = listed
        .pointer("/result/entries")
        .and_then(Value::as_array)
        .ok_or("source list entries")?
        .iter()
        .map(|row| row["source"].as_str().map(str::to_owned))
        .collect::<Option<Vec<_>>>()
        .ok_or("source name")?;
    if !["pubmed", "oncokb", "meddra"]
        .iter()
        .all(|name| names.iter().any(|listed| listed == name))
        || names.iter().any(|name| name == "noncommercial")
    {
        return Err(format!("wrong source list: {listed}"));
    }
    let expected = json!({"error":format!(
        "Unknown source name in {HEADER} header; allowed values: {}",
        names.join(", ")
    )});
    let mut server =
        Server::configured_globals("127.0.0.1:0", None, None, None, None, &["--hosted".into()])?;
    let tool = json!({"name":"biomcp","arguments":{"args":CLINPGX}});
    for extra in [
        list("nonesuch"),
        list("noncommercial"),
        list("CLINPGX"),
        list("clinpgx,"),
        list(",clinpgx"),
        list("-clinpgx"),
        format!("{}{}", list("clinpgx"), list("civic")),
    ] {
        for (method, params) in [("tools/call", tool.clone()), ("tools/list", json!({}))] {
            let (status, reply) = post(&server, method, params, &extra)?;
            if status != 400 || reply != expected {
                return Err(format!("{extra:?} on {method} gave {status}: {reply}"));
            }
        }
    }
    let every = call(
        &server,
        &["help", "sources"],
        &list(&names.join(",")),
        false,
    )?;
    if every.pointer("/result/isError") == Some(&json!(true)) {
        return Err(format!("a listed name was refused: {every}"));
    }
    let hosted = call(&server, &["source", "list"], "", false)?;
    if hosted.to_string().contains(HEADER) {
        return Err(format!("hosted MCP served a local command: {hosted}"));
    }
    server.stop()?;
    let mut ordinary = Server::configured_globals("127.0.0.1:0", None, None, None, None, &[])?;
    let (status, _) = post(&ordinary, "tools/list", json!({}), &list("nonesuch"))?;
    ordinary.stop()?;
    if status == 200 {
        Ok(())
    } else {
        Err(format!("ordinary HTTP read the header: {status}"))
    }
}

fn offers_reactions(reply: &Value) -> bool {
    reply
        .pointer("/result/structuredContent/next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|step| step["args"].as_array().is_some_and(|args| args.contains(&json!("reactions"))))
}

fn reaction_filter(server: &Server, extra: &str) -> Check<bool> {
    let (status, reply) = post(server, "tools/list", json!({}), extra)?;
    let tools = reply
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .filter(|_| status == 200)
        .ok_or_else(|| format!("catalog {status}: {reply}"))?;
    let search = tools
        .iter()
        .find(|tool| tool["name"] == "search_adverse_event")
        .ok_or("search adverse event")?;
    Ok(search.pointer("/inputSchema/properties/reaction").is_some())
}

fn discovery() -> Check {
    let replay = fixtures().join("replies/adverse-event/faers-synthetic");
    let narrowed = format!("{ACADEMIC}{}", list("meddra"));
    let mut typed = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"mcp_tool_mode":"typed"})),
        Some(replay.clone()),
        None,
        None,
        &["--hosted".into()],
    )?;
    if !reaction_filter(&typed, ACADEMIC)? || reaction_filter(&typed, &narrowed)? {
        return Err("tool list disagrees with the source list".into());
    }
    typed.stop()?;
    let mut server = Server::configured_globals(
        "127.0.0.1:0",
        None,
        Some(replay),
        None,
        None,
        &["--hosted".into()],
    )?;
    let whole = call(&server, CARD, ACADEMIC, false)?;
    let card = call(&server, CARD, &narrowed, false)?;
    if !offers_reactions(&whole) || offers_reactions(&card) {
        return Err(format!("next disagrees with the list: {whole} then {card}"));
    }
    let help = call(&server, &["help", "sources"], &narrowed, false)?.to_string();
    if !help.contains("meddra: This request's source list leaves out") {
        return Err(format!("help disagrees with the list: {help}"));
    }
    if card.to_string().contains("--disable-sources") {
        return Err(format!("the header became a flag: {card}"));
    }
    server.stop()
}
