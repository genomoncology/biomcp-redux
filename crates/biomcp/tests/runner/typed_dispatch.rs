//! A client can discover and run a typed tool using only the tools it was shown.
use crate::runner::{http::Server, launch::fixtures, mcp::request};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let compact = server(&[])?;
    let full = server(&["--catalog".into(), "full".into()])?;
    let listed = result(&compact, "tools/list", json!({}))?;
    let full_list = result(&full, "tools/list", json!({}))?;
    let available = |name| tool(&listed, name);
    if listed.get("tools").and_then(Value::as_array).map(Vec::len) != Some(2) {
        return Err("plain tools/list did not return two compact tools".into());
    }
    let search = tool_name(available("search_tools")?)?;
    let dispatcher = available("call_tool")?;
    let dispatch = tool_name(dispatcher)?;
    let matches = call(&compact, search, json!({"query":"trials near a city"}))?;
    let name = tool_name(
        matches
            .pointer("/structuredContent/tools/0")
            .ok_or("no search match")?,
    )?;
    if name != "trial" {
        return Err(format!("expected trial, got {name}"));
    }
    let described = call(&compact, search, json!({"name":name}))?;
    let definition = described
        .pointer("/structuredContent/tools/0")
        .ok_or("missing definition")?;
    if definition != tool(&full_list, name)? {
        return Err("discovered definition differs from the full typed list".into());
    }
    let inputs = [
        json!({"action":"search","search":{"condition":"melanoma"}}),
        json!({"action":"get","get":{"id":"not-an-nct"}}),
    ];
    for (arguments, is_error) in inputs.into_iter().zip([false, true]) {
        let routed = call(
            &compact,
            dispatch,
            json!({"name":name,"arguments":arguments}),
        )?;
        let direct = call(&full, name, arguments)?;
        if routed != direct || routed.get("isError") != Some(&json!(is_error)) {
            return Err(format!(
                "dispatched trial reply differs from direct reply: {routed} vs {direct}"
            ));
        }
        let content = routed
            .get("structuredContent")
            .ok_or("missing structured reply")?;
        for advertised in [dispatcher, definition] {
            let schema = advertised
                .get("outputSchema")
                .ok_or("missing output schema")?;
            jsonschema::validator_for(schema)
                .map_err(|error| error.to_string())?
                .validate(content)
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn server(flags: &[String]) -> Result<Server, String> {
    let config = json!({"mcp_tool_mode":"typed","mcp_catalog":"compact"});
    let replay = fixtures().join("replies/trial/condition");
    Server::configured_globals("127.0.0.1:0", Some(config), Some(replay), None, None, flags)
}

fn tool<'a>(list: &'a Value, name: &str) -> Result<&'a Value, String> {
    list.get("tools")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .find(|tool| tool_name(tool).is_ok_and(|listed| listed == name))
        .ok_or_else(|| format!("{name} is missing from tools/list"))
}

fn tool_name(tool: &Value) -> Result<&str, String> {
    tool.get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing tool name".into())
}

fn call(server: &Server, name: &str, arguments: Value) -> Result<Value, String> {
    result(
        server,
        "tools/call",
        json!({"name":name,"arguments":arguments}),
    )
}

fn result(server: &Server, method: &str, params: Value) -> Result<Value, String> {
    let frame = request(json!(1), method, params);
    let (status, response) = server.request("POST", "/mcp", &frame.to_string())?;
    response
        .get("result")
        .cloned()
        .filter(|_| status == 200)
        .ok_or_else(|| format!("{method} failed: {status} {response}"))
}
