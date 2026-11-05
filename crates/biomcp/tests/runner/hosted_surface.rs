//! Hosted discovery and follow-ups describe this server's actual authority.
use super::super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp::request,
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let mut errors = Vec::new();
    for mode in ["single", "typed"] {
        let config_owner = Environment::new().map_err(|e| e.to_string())?;
        config_owner.config(&json!({"mcp_tool_mode":mode,"hosted_max_use":"restricted","disable_sources":["ctgov","meddra","reactome"]})).map_err(|e| e.to_string())?;
        let server = Server::configured_globals(
            "127.0.0.1:0",
            None,
            Some(fixtures().join("replies/drug/openfda-label-page")),
            None,
            None,
            &[
                "--hosted".into(),
                "--config".into(),
                config_owner
                    .root()
                    .join("config/biomcp/config.json")
                    .to_string_lossy()
                    .into_owned(),
            ],
        )?;
        let catalog = request(json!(2), "tools/list", json!({}));
        let (_, catalog) = server.request("POST", "/mcp", &catalog.to_string())?;
        if mode == "typed" {
            let tools = catalog
                .pointer("/result/tools")
                .and_then(Value::as_array)
                .ok_or("catalog")?;
            let gene = tools
                .iter()
                .find(|tool| tool["name"] == "get_gene")
                .ok_or("get_gene")?;
            let schema = &gene["inputSchema"];
            let validator = jsonschema::validator_for(schema).map_err(|e| e.to_string())?;
            if !validator.is_valid(&json!({"input":["BRAF"]})) {
                return Err("hosted gene schema rejects a supported input".into());
            }
            if validator.is_valid(&json!({"input":["BRAF"],"section":["oncokb"]})) {
                errors.push("typed: disabled OncoKB section offered".into());
            }
        }
        for path in ["", "get gene", "get pathway", "sources"] {
            let params = if mode == "typed" {
                json!({"name":"biomcp_help","arguments":{"command":path}})
            } else {
                let mut args = vec!["help"];
                args.extend(path.split_whitespace());
                json!({"name":"biomcp","arguments":{"args":args}})
            };
            let reply = call(&server, params)?;
            let commands = reply
                .pointer("/result/commands")
                .and_then(Value::as_array)
                .ok_or("help commands")?;
            if path == "get pathway"
                && commands.iter().any(|command| {
                    command
                        .get("arguments")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                        .any(|arg| {
                            arg.get("choices")
                                .and_then(Value::as_array)
                                .is_some_and(|choices| choices.contains(&json!("events")))
                        })
                })
            {
                errors.push(format!("{mode}: disabled Reactome events section offered"));
            }
            for command in commands {
                let path = command.get("command").cloned().unwrap_or(json!([]));
                if matches!(
                    path.get(0).and_then(Value::as_str),
                    Some(
                        "health"
                            | "sync"
                            | "store"
                            | "version"
                            | "serve"
                            | "key"
                            | "mcp-config"
                            | "skill"
                    )
                ) {
                    errors.push(format!("{mode}: unavailable command offered: {path}"));
                }
                if command.to_string().contains("--fresh")
                    || command.to_string().contains("--config")
                {
                    errors.push(format!("{mode}: host-only option in help {path}"));
                }
            }
            if path == "sources" {
                let text = reply.to_string();
                if !text.contains("restricted")
                    || !text.contains("operator")
                    || !text.contains("self-hosted")
                {
                    errors.push(format!("{mode}: source help lacks server policy"));
                }
            }
        }
        for (verb, entity, args, input, expected) in [
            (
                "search",
                "adverse-event",
                vec!["Synthetic medicine", "--reaction", "rash"],
                json!({"text":"Synthetic medicine","reaction":"rash"}),
                "operator",
            ),
            (
                "search",
                "drug",
                vec!["imatinib", "--source", "openfda-label", "--limit", "3"],
                json!({"query":"imatinib","source":"openfda-label","limit":3}),
                "next",
            ),
            (
                "search",
                "drug",
                vec!["artemether", "--source", "who-pq"],
                json!({"query":"artemether","source":"who-pq"}),
                "restricted",
            ),
            (
                "search",
                "trial",
                vec!["--condition", "melanoma"],
                json!({"condition":"melanoma"}),
                "operator",
            ),
            ("get", "gene", vec!["BRAF"], json!({"input":["BRAF"]}), ""),
        ] {
            let params = if mode == "typed" {
                json!({"name":format!("{verb}_{entity}"),"arguments":input})
            } else {
                let mut words = vec![verb, entity];
                words.extend(args);
                json!({"name":"biomcp","arguments":{"args":words}})
            };
            let reply = call(&server, params)?;
            if expected == "next"
                && reply
                    .get("next")
                    .and_then(Value::as_array)
                    .is_none_or(Vec::is_empty)
            {
                errors.push(format!("{mode}: allowed follow-ups lost: {reply}"));
            }
            if !expected.is_empty()
                && expected != "next"
                && (!reply.to_string().contains(expected)
                    || !reply.to_string().contains("self-hosted"))
            {
                errors.push(format!(
                    "{mode}: disabled source lacks {expected} explanation: {reply}"
                ));
            }
            for step in reply
                .get("next")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let args = step
                    .get("args")
                    .and_then(Value::as_array)
                    .ok_or("next args")?;
                if args.iter().any(|word| {
                    matches!(
                        word.as_str(),
                        Some("--fresh" | "--config" | "sync" | "health")
                    )
                }) {
                    errors.push(format!("{mode}: unavailable next: {step}"));
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn call(server: &Server, params: Value) -> Result<Value, String> {
    let frame = request(json!(1), "tools/call", params);
    let (_, reply) = server.request("POST", "/mcp", &frame.to_string())?;
    reply
        .pointer("/result/structuredContent")
        .cloned()
        .ok_or_else(|| format!("missing tool reply: {reply}"))
}
