//! Find one executable typed tool without reading the full catalog first.
use super::{Environment, reply, request};
use crate::runner::{http::Server, launch::fixtures, mcp::exchange_with_globals};
use serde_json::{Value, json};

fn compact_list() -> Value {
    let mut frame = request(json!("compact"), "tools/list", json!({}));
    frame["params"]["_meta"]["biomcp/compactCatalog"] = json!(true);
    frame
}

fn discovery(id: &str, arguments: Value) -> Value {
    request(
        json!(id),
        "tools/call",
        json!({"name":"search_tools","arguments":arguments}),
    )
}

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"mcp_tool_mode":"typed"}))
        .map_err(|e| e.to_string())?;
    let frames = [
        compact_list(),
        discovery("search", json!({"query":"trials near a city"})),
        discovery("definition", json!({"name":"trial"})),
        request(json!("full"), "tools/list", json!({})),
        discovery("miss", json!({"query":"qqzzxx"})),
        request(
            json!("trial"),
            "tools/call",
            json!({
                "name":"trial","arguments":{"action":"search","search":{"condition":"melanoma"}}
            }),
        ),
    ];
    let replay = fixtures().join("replies/trial/condition");
    let replies = exchange_with_globals(&env, &frames, &env.globals(Some(&replay), false))?;
    let get = |id: &str| reply(&replies, &json!(id));
    let compact = &get("compact")?["result"]["tools"];
    if compact.as_array().map(Vec::len) != Some(1)
        || compact[0]["name"] != "search_tools"
        || compact.to_string().len() > 1500
    {
        return Err(format!(
            "expected a small discovery catalog; got {} bytes",
            compact.to_string().len()
        ));
    }
    let found = &get("search")?["result"]["structuredContent"]["tools"][0];
    if found["name"] != "trial"
        || !found["description"]
            .as_str()
            .is_some_and(|text| !text.is_empty() && text.len() <= 240)
        || found.as_object().map(serde_json::Map::len) != Some(2)
    {
        return Err(format!("expected a short trial match: {found}"));
    }
    let full = &get("full")?["result"]["tools"];
    let expected = full
        .as_array()
        .into_iter()
        .flatten()
        .find(|tool| tool["name"] == "trial");
    let definition = &get("definition")?["result"]["structuredContent"]["tools"][0];
    if Some(definition) != expected || !definition["outputSchema"].is_object() {
        return Err("trial definition differs from the full typed catalog".into());
    }
    if get("miss")?["result"]["structuredContent"]["tools"] != json!([]) {
        return Err("no-match discovery must return an empty tools array".into());
    }
    if get("trial")?["result"]["structuredContent"]["status"] != "complete" {
        return Err("discovered trial tool did not complete its recorded search".into());
    }
    default_list(&env)
}

fn default_list(env: &Environment) -> Result<(), String> {
    let flags = ["--offline".into(), "--tool-mode".into(), "single".into()];
    let frames = [
        compact_list(),
        request(json!("full"), "tools/list", json!({})),
    ];
    let replies = exchange_with_globals(env, &frames, &flags)?;
    let compact = &reply(&replies, &json!("compact"))?["result"]["tools"];
    let full = &reply(&replies, &json!("full"))?["result"]["tools"];
    if compact != full || full.as_array().map(Vec::len) != Some(3) {
        return Err("the compact option changed the default three-tool list".into());
    }
    Ok(())
}

pub(super) fn hosted() -> Result<(), String> {
    let flags = ["--offline".into(), "--hosted".into()];
    let config = json!({"mcp_tool_mode":"typed"});
    let server = Server::configured_globals("127.0.0.1:0", Some(config), None, None, None, &flags)?;
    let frame = discovery("definition", json!({"name":"saved"}));
    let (status, response) = server.request("POST", "/mcp", &frame.to_string())?;
    if status != 200 || response["result"]["structuredContent"]["tools"] != json!([]) {
        return Err(format!(
            "hosted discovery must omit unavailable saved tools: {status} {response}"
        ));
    }
    Ok(())
}
