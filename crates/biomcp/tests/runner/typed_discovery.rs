//! Find one executable typed tool without reading the full catalog first.
use super::{Environment, reply, request};
use crate::runner::{http::Server, launch::fixtures, mcp::exchange_with_globals};
use serde_json::{Value, json};

fn compact_list() -> Result<Value, String> {
    let mut frame = request(json!("compact"), "tools/list", json!({}));
    frame
        .pointer_mut("/params/_meta")
        .and_then(Value::as_object_mut)
        .ok_or("missing request metadata")?
        .insert("biomcp/compactCatalog".into(), json!(true));
    Ok(frame)
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
        compact_list()?,
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
    let compact = get("compact")?
        .pointer("/result/tools")
        .ok_or("compact tools")?;
    if compact.as_array().map(Vec::len) != Some(2)
        || compact.pointer("/0/name").and_then(Value::as_str) != Some("search_tools")
        || compact.to_string().len() > 1500
    {
        return Err(format!(
            "expected a small discovery catalog; got {} bytes",
            compact.to_string().len()
        ));
    }
    let found = get("search")?
        .pointer("/result/structuredContent/tools/0")
        .ok_or("search match")?;
    if found.get("name").and_then(Value::as_str) != Some("trial")
        || !found
            .get("description")
            .and_then(Value::as_str)
            .is_some_and(|text| !text.is_empty() && text.len() <= 240)
        || found.as_object().map(serde_json::Map::len) != Some(2)
    {
        return Err(format!("expected a short trial match: {found}"));
    }
    let full = get("full")?.pointer("/result/tools").ok_or("full tools")?;
    let expected = full
        .as_array()
        .into_iter()
        .flatten()
        .find(|tool| tool.get("name").and_then(Value::as_str) == Some("trial"));
    let definition = get("definition")?
        .pointer("/result/structuredContent/tools/0")
        .ok_or("requested definition")?;
    if Some(definition) != expected || !definition.get("outputSchema").is_some_and(Value::is_object)
    {
        return Err("trial definition differs from the full typed catalog".into());
    }
    if get("miss")?.pointer("/result/structuredContent/tools") != Some(&json!([])) {
        return Err("no-match discovery must return an empty tools array".into());
    }
    if get("trial")?
        .pointer("/result/structuredContent/status")
        .and_then(Value::as_str)
        != Some("complete")
    {
        return Err("discovered trial tool did not complete its recorded search".into());
    }
    default_list(&env)
}

fn default_list(env: &Environment) -> Result<(), String> {
    let flags = ["--offline".into(), "--tool-mode".into(), "single".into()];
    let frames = [
        compact_list()?,
        request(json!("full"), "tools/list", json!({})),
    ];
    let replies = exchange_with_globals(env, &frames, &flags)?;
    let compact = reply(&replies, &json!("compact"))?.pointer("/result/tools");
    let full = reply(&replies, &json!("full"))?.pointer("/result/tools");
    if compact != full || full.and_then(Value::as_array).map(Vec::len) != Some(3) {
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
    if status != 200 || response.pointer("/result/structuredContent/tools") != Some(&json!([])) {
        return Err(format!(
            "hosted discovery must omit unavailable saved tools: {status} {response}"
        ));
    }
    Ok(())
}

pub(super) fn short_source_routes() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    for (case, name, field, pointer, expected) in [
        (
            "mechanism",
            "drug",
            "mechanism_of_action",
            "/page/returned",
            json!(3),
        ),
        (
            "curation",
            "disease",
            "source_title_prefix",
            "/page/returned",
            json!(1),
        ),
        (
            "cdx",
            "diagnostic",
            "drug_application",
            "/page/total",
            json!(2),
        ),
        (
            "pmid",
            "article",
            "28944238",
            "/result/pmcid",
            json!("PMC5606870"),
        ),
    ] {
        let path = fixtures().join(format!("cases/mcp/typed/short-source-{case}.yaml"));
        let case = crate::runner::case_data::read_case(&path)?;
        let mut config = crate::runner::launch::case_config(&case);
        config
            .as_object_mut()
            .ok_or("expected a case configuration object")?
            .insert("mcp_catalog".into(), json!("compact"));
        env.config(&config).map_err(|error| error.to_string())?;
        let replay = fixtures()
            .join("replies")
            .join(case.replay.ok_or("missing replay")?);
        let globals = env.globals(Some(&replay), false);
        if let Some(sync) = case.steps.first().and_then(|step| step.run.as_deref())
            && sync.first().is_some_and(|word| word == "sync")
        {
            crate::runner::trial_get::command(&env, sync, &globals, 0)?;
        }
        let definition = discovery("definition", json!({"name":name}));
        let replies = exchange_with_globals(&env, &[definition], &globals)?;
        let description = reply(&replies, &json!("definition"))?
            .pointer("/result/structuredContent/tools/0/description")
            .and_then(Value::as_str)
            .ok_or("missing compact tool description")?;
        let arguments = copied_example(description, field)
            .ok_or_else(|| format!("{name} description has no {field} example"))?;
        let call = request(
            json!("example"),
            "tools/call",
            json!({
                "name":"call_tool","arguments":{"name":name,"arguments":arguments}
            }),
        );
        let replies = exchange_with_globals(&env, &[call], &globals)?;
        let result = reply(&replies, &json!("example"))?
            .get("result")
            .ok_or("missing result")?;
        let value = crate::runner::mcp::one_value(result)?;
        if result.get("isError") != Some(&json!(false)) || value.pointer(pointer) != Some(&expected)
        {
            return Err(format!("{name} copied example returned {value}"));
        }
    }
    Ok(())
}

fn copied_example(description: &str, field: &str) -> Option<Value> {
    description
        .match_indices("{\"action\":\"")
        .filter_map(|(offset, _)| {
            serde_json::Deserializer::from_str(description.get(offset..)?)
                .into_iter::<Value>()
                .next()?
                .ok()
        })
        .find(|example| example.to_string().contains(field))
}
