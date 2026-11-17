//! Hosted typed tools offer only source words that reach a source hosting keeps on.
use super::{cli, context, listed, seed};
use crate::runner::{http::Server, launch::Environment, mcp::request};
use serde_json::{Value, json};

// The hosted list must never offer a word that only reaches a source hosting has switched off.
pub(super) fn hosted_sources(env: &Environment, failures: &mut Vec<String>) -> Result<(), String> {
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"mcp_tool_mode":"typed"})),
        None,
        None,
        None,
        &["--offline".into(), "--hosted".into()],
    )?;
    let frame = request(json!(1), "tools/list", json!({}));
    let (_, catalog) = server.request("POST", "/mcp", &frame.to_string())?;
    let tools = catalog
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .ok_or("hosted tools")?;
    let tools = crate::runner::protocol::typed_catalog::commands(tools);
    for (verb, entity, id) in biomcp_types::command_paths() {
        let name = format!("{verb}_{entity}").replace('-', "_");
        let Some(tool) = tools.iter().find(|tool| tool["name"] == name.as_str()) else {
            continue;
        };
        let schema = cli(env, &["schema".into(), id.into()])?;
        for word in listed(
            "source",
            tool.pointer("/inputSchema/properties/source/enum"),
        ) {
            let mut values = seed(verb, entity, &schema)?;
            context(verb, entity, "source", &mut values)?;
            values.insert("source".into(), json!(word));
            let call = request(
                json!(2),
                "tools/call",
                crate::runner::protocol::typed_catalog::entity(&name, json!(values)),
            );
            let (_, reply) = server.request("POST", "/mcp", &call.to_string())?;
            if only_disabled(reply.pointer("/result/structuredContent/sources")) {
                failures.push(format!("hosted {name} offers refused source {word}"));
            }
        }
    }
    Ok(())
}

fn only_disabled(sources: Option<&Value>) -> bool {
    sources
        .and_then(Value::as_array)
        .filter(|sources| !sources.is_empty())
        .is_some_and(|sources| sources.iter().all(|s| s["reason"] == "disabled"))
}
