//! Hosted discovery and follow-ups describe this server's actual authority.
use super::super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let mut errors = Vec::new();
    for mode in ["single", "typed"] {
        continuation(mode)?;
        components(mode)?;
        let config_owner = Environment::new().map_err(|e| e.to_string())?;
        config_owner
            .config(&json!({
                "mcp_tool_mode":mode, "hosted_max_use":"restricted",
                "disable_sources":["ctgov","meddra","gmdn","reactome"]
            }))
            .map_err(|e| e.to_string())?;
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
        let invalid = if mode == "typed" {
            json!({"name":"article_text","arguments":{"unexpected":true}})
        } else {
            json!({"name":"biomcp","arguments":{"args":["article","text","--unexpected"]}})
        };
        recovery(&call(&server, invalid.clone())?)?;
        let stdio = exchange_with_globals(
            &config_owner,
            &[request(json!(3), "tools/call", invalid)],
            &["--hosted".into(), "--offline".into()],
        )?;
        recovery(
            stdio
                .first()
                .and_then(|reply| reply.pointer("/result/structuredContent"))
                .ok_or("stdio error reply")?,
        )?;
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
        for path in [
            "",
            "get gene",
            "get pathway",
            "get adverse-event",
            "search adverse-event",
            "sources",
        ] {
            let reply = help(&server, mode, path)?;
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

fn recovery(reply: &Value) -> Result<(), String> {
    if reply.pointer("/error/code") != Some(&json!("usage.arguments")) {
        return Err(format!("parser refusal changed: {reply}"));
    }
    for value in reply
        .pointer("/error/valid_values")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if matches!(
            value.as_str(),
            Some(
                "save"
                    | "replace"
                    | "--save"
                    | "--replace"
                    | "--fresh"
                    | "--config"
                    | "--source_pacing"
            )
        ) {
            return Err(format!("hosted recovery offers refused option: {reply}"));
        }
    }
    Ok(())
}

fn continuation(mode: &str) -> Result<(), String> {
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(json!({"mcp_tool_mode":mode,"hosted_max_use":"restricted"})),
        Some(fixtures().join("replies/variant/civic-evidence-filters")),
        None,
        None,
        &[
            "--hosted".into(),
            "--disable-sources".into(),
            "myvariant".into(),
        ],
    )?;
    let params = if mode == "typed" {
        json!({"name":"get_variant","arguments":{
            "input":"CA123643","section":["civic"],"section_limit":2,
            "evidence_level":["A"],"evidence_status":"submitted"
        }})
    } else {
        json!({"name":"biomcp","arguments":{"args":[
            "get","variant","CA123643","civic","--section-limit","2",
            "--evidence-level","A","--evidence-status","submitted"
        ]}})
    };
    let reply = call(&server, params)?;
    let page = reply
        .pointer("/result/candidates/0/sections/civic/data/profiles/0/evidence_page")
        .ok_or_else(|| format!("{mode}: CIViC page missing: {reply}"))?;
    for (pointer, expected) in [
        ("/total", json!(3)),
        ("/returned", json!(2)),
        ("/next/input", json!("CA123643")),
        ("/next/civic_profile", json!(12)),
        ("/next/civic_kind", json!("evidence")),
        ("/next/evidence_level", json!(["A"])),
        ("/next/civic_cursor", json!("0:E2")),
        ("/next/section", json!(["civic"])),
        ("/next/evidence_status", json!("submitted")),
    ] {
        if page.pointer(pointer) != Some(&expected) {
            return Err(format!(
                "{mode}: typed CIViC continuation changed at {pointer}: {page}"
            ));
        }
    }
    Ok(())
}

fn components(mode: &str) -> Result<(), String> {
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(
            json!({"mcp_tool_mode":mode,"hosted_max_use":"restricted","disable_sources":["meddra","gmdn"]}),
        ),
        Some(fixtures().join("replies/adverse-event/faers-synthetic")),
        None,
        None,
        &["--hosted".into()],
    )?;
    let (_, catalog) = server.request(
        "POST",
        "/mcp",
        &request(json!(2), "tools/list", json!({})).to_string(),
    )?;
    if mode == "typed" {
        let tools = catalog
            .pointer("/result/tools")
            .and_then(Value::as_array)
            .ok_or("component tools")?;
        let get = tools
            .iter()
            .find(|tool| tool["name"] == "get_adverse_event")
            .ok_or("get adverse event")?;
        let validator =
            jsonschema::validator_for(&get["inputSchema"]).map_err(|e| e.to_string())?;
        if !validator.is_valid(&json!({"id":"100","source":"faers","section":"drugs"}))
            || !validator.is_valid(&json!({"id":"123","source":"maude","section":"narrative"}))
        {
            return Err("component-free report sections disappeared".into());
        }
        for (source, section) in [("faers", "reactions"), ("maude", "devices")] {
            if validator.is_valid(&json!({"id":"100","source":source,"section":section})) {
                return Err(format!(
                    "{section} offered with its required component disabled"
                ));
            }
        }
        let search = tools
            .iter()
            .find(|tool| tool["name"] == "search_adverse_event")
            .ok_or("search adverse event")?;
        let validator =
            jsonschema::validator_for(&search["inputSchema"]).map_err(|e| e.to_string())?;
        if !validator.is_valid(&json!({"text":"Synthetic medicine","source":"faers"}))
            || validator.is_valid(&json!({"text":"device","source":"maude"}))
            || search.pointer("/inputSchema/properties/reaction").is_some()
        {
            return Err("search component capabilities disagree with admission".into());
        }
    }
    for path in ["get adverse-event", "search adverse-event"] {
        let help = help(&server, mode, path)?;
        for command in help
            .pointer("/result/commands")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            for section in command
                .get("sections")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if matches!(section["word"].as_str(), Some("reactions" | "devices")) {
                    return Err(format!("{mode}: component-disabled section in help"));
                }
            }
            for argument in command
                .get("arguments")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let choices = argument.get("choices").and_then(Value::as_array);
                let blocked_section = choices.is_some_and(|choices| {
                    choices
                        .iter()
                        .any(|word| matches!(word.as_str(), Some("reactions" | "devices")))
                });
                let blocked_source = path == "search adverse-event"
                    && choices.is_some_and(|choices| choices.contains(&json!("maude")));
                if argument["name"] == "--reaction" || blocked_section || blocked_source {
                    return Err(format!(
                        "{mode}: component-disabled option in help: {argument}"
                    ));
                }
            }
        }
    }
    let card = if mode == "typed" {
        json!({"name":"get_adverse_event","arguments":{"id":"100"}})
    } else {
        json!({"name":"biomcp","arguments":{"args":["get","adverse-event","100"]}})
    };
    let card = call(&server, card)?;
    if card["status"] != "complete" {
        return Err(format!("card failed: {card}"));
    }
    if card
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|step| {
            step["args"]
                .as_array()
                .is_some_and(|args| args.contains(&json!("reactions")))
        })
    {
        return Err("disabled reaction section offered by next".into());
    }
    for (source, section) in [("faers", "reactions"), ("maude", "devices")] {
        let params = if mode == "typed" {
            json!({"name":"get_adverse_event","arguments":{"id":"100","source":source,"section":section}})
        } else {
            json!({"name":"biomcp","arguments":{"args":["get","adverse-event","100",section,"--source",source]}})
        };
        let refused = call(&server, params)?;
        if refused.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("component refusal changed: {refused}"));
        }
    }
    Ok(())
}

fn help(server: &Server, mode: &str, path: &str) -> Result<Value, String> {
    let params = if mode == "typed" {
        json!({"name":"biomcp_help","arguments":{"command":path}})
    } else {
        let mut args = vec!["help"];
        args.extend(path.split_whitespace());
        json!({"name":"biomcp","arguments":{"args":args}})
    };
    call(server, params)
}
