//! Hosted discovery and follow-ups describe this server's actual authority.
use super::super::{
    http::Server,
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

type Check = Result<(), String>;

pub(super) fn check() -> Check {
    for mode in ["single", "typed"] {
        continuation(mode)?;
        components(mode)?;
        surface(mode)?;
    }
    Ok(())
}

fn surface(mode: &str) -> Check {
    let config = Environment::new().map_err(|e| e.to_string())?;
    config
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
            config
                .root()
                .join("config/biomcp/config.json")
                .to_string_lossy()
                .into_owned(),
        ],
    )?;
    parser_recovery(&server, &config, mode)?;
    if mode == "typed" {
        let tools = catalog(&server)?;
        let gene = tools
            .iter()
            .find(|tool| tool.get("name") == Some(&json!("get_gene")))
            .ok_or("get_gene")?;
        let validator = jsonschema::validator_for(gene.get("inputSchema").ok_or("gene schema")?)
            .map_err(|e| e.to_string())?;
        if !validator.is_valid(&json!({"input":["BRAF"]}))
            || validator.is_valid(&json!({"input":["BRAF"],"section":["oncokb"]}))
        {
            return Err("hosted gene schema disagrees with supported input/OncoKB policy".into());
        }
    }
    offered_help(&server, mode)?;
    source_notices(&server, mode)
}

fn parser_recovery(server: &Server, env: &Environment, mode: &str) -> Check {
    let invalid = params(
        mode,
        "article_text",
        &["article", "text", "--unexpected"],
        json!({"unexpected":true}),
    );
    recovery(&call(server, invalid.clone())?)?;
    let stdio = exchange_with_globals(
        env,
        &[request(json!(3), "tools/call", invalid)],
        &["--hosted".into(), "--offline".into()],
    )?;
    recovery(
        stdio
            .first()
            .and_then(|reply| reply.pointer("/result/structuredContent"))
            .ok_or("stdio error reply")?,
    )
}

fn offered_help(server: &Server, mode: &str) -> Check {
    for path in [
        "",
        "get gene",
        "get pathway",
        "get adverse-event",
        "search adverse-event",
        "sources",
        "config",
        "relations",
    ] {
        let reply = help(server, mode, path)?;
        let commands = reply
            .pointer("/result/commands")
            .and_then(Value::as_array)
            .ok_or("help commands")?;
        for command in commands {
            let words = command
                .get("command")
                .and_then(Value::as_array)
                .ok_or("help command words")?;
            if matches!(
                words.first().and_then(Value::as_str),
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
                return Err(format!("{mode}: unavailable command offered: {words:?}"));
            }
            if command.to_string().contains("--fresh") || command.to_string().contains("--config") {
                return Err(format!("{mode}: host-only option in help {words:?}"));
            }
            if path == "get pathway"
                && command
                    .get("arguments")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .any(|arg| {
                        arg.get("choices")
                            .and_then(Value::as_array)
                            .is_some_and(|choices| choices.contains(&json!("events")))
                    })
            {
                return Err(format!("{mode}: disabled Reactome events section offered"));
            }
        }
        if matches!(path, "sources" | "config") {
            let text = reply.to_string();
            if !text.contains("restricted")
                || !text.contains("operator")
                || !text.contains("self-hosted")
            {
                return Err(format!("{mode}: source help lacks server policy"));
            }
        }
        if path == "relations" {
            let text = reply.to_string();
            if text.contains("pathway (get section pathways)")
                || !text.contains("gene (get section genes)")
            {
                return Err(format!(
                    "{mode}: relation map loses source-specific availability: {reply}"
                ));
            }
        }
    }
    Ok(())
}

fn source_notices(server: &Server, mode: &str) -> Check {
    for (tool, args, input, expected) in [
        (
            "search_drug",
            vec![
                "search",
                "drug",
                "imatinib",
                "--source",
                "openfda-label",
                "--limit",
                "3",
            ],
            json!({"query":"imatinib","source":"openfda-label","limit":3}),
            "next",
        ),
        (
            "search_drug",
            vec!["search", "drug", "artemether", "--source", "who-pq"],
            json!({"query":"artemether","source":"who-pq"}),
            "restricted",
        ),
        (
            "search_trial",
            vec!["search", "trial", "--condition", "melanoma"],
            json!({"condition":"melanoma"}),
            "operator",
        ),
        (
            "get_gene",
            vec!["get", "gene", "BRAF"],
            json!({"input":["BRAF"]}),
            "",
        ),
    ] {
        let reply = query(server, mode, tool, &args, input)?;
        if expected == "next"
            && reply
                .get("next")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(format!("{mode}: allowed follow-ups lost: {reply}"));
        }
        if !expected.is_empty()
            && expected != "next"
            && (!reply.to_string().contains(expected) || !reply.to_string().contains("self-hosted"))
        {
            return Err(format!(
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
                return Err(format!("{mode}: unavailable next: {step}"));
            }
        }
    }
    Ok(())
}

fn components(mode: &str) -> Check {
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
    if mode == "typed" {
        component_catalog(&server)?;
    }
    component_help(&server, mode)?;
    component_recovery(&server, mode)?;
    aggregate_component(&server, mode)?;
    component_card(&server, mode)
}

fn component_catalog(server: &Server) -> Check {
    let tools = catalog(server)?;
    let get = tools
        .iter()
        .find(|tool| tool.get("name") == Some(&json!("get_adverse_event")))
        .ok_or("get adverse event")?;
    let validator = jsonschema::validator_for(get.get("inputSchema").ok_or("get schema")?)
        .map_err(|e| e.to_string())?;
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
        .find(|tool| tool.get("name") == Some(&json!("search_adverse_event")))
        .ok_or("search adverse event")?;
    let validator = jsonschema::validator_for(search.get("inputSchema").ok_or("search schema")?)
        .map_err(|e| e.to_string())?;
    if !validator.is_valid(&json!({"text":"Synthetic medicine","source":"faers"}))
        || validator.is_valid(&json!({"text":"device","source":"maude"}))
        || search.pointer("/inputSchema/properties/reaction").is_some()
        || search
            .pointer("/inputSchema/properties/aggregate")
            .is_some()
    {
        return Err("search component capabilities disagree with admission".into());
    }
    Ok(())
}

fn component_help(server: &Server, mode: &str) -> Check {
    for path in ["get adverse-event", "search adverse-event"] {
        let help = help(server, mode, path)?;
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
                if matches!(
                    section.get("word").and_then(Value::as_str),
                    Some("reactions" | "devices")
                ) {
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
                let section = choices.is_some_and(|choices| {
                    choices
                        .iter()
                        .any(|word| matches!(word.as_str(), Some("reactions" | "devices")))
                });
                let source = path == "search adverse-event"
                    && choices.is_some_and(|choices| choices.contains(&json!("maude")));
                if matches!(
                    argument.get("name").and_then(Value::as_str),
                    Some("--reaction" | "--aggregate")
                ) || section
                    || source
                {
                    return Err(format!(
                        "{mode}: component-disabled option in help: {argument}"
                    ));
                }
            }
        }
    }
    Ok(())
}

fn component_recovery(server: &Server, mode: &str) -> Check {
    for (tool, args, input) in [
        (
            "search_adverse_event",
            vec!["search", "adverse-event", "x", "--source", "bogus"],
            json!({"text":"x","source":"bogus"}),
        ),
        (
            "get_adverse_event",
            vec!["get", "adverse-event", "100", "--source", "faers", "bogus"],
            json!({"id":"100","source":"faers","section":"bogus"}),
        ),
    ] {
        let reply = query(server, mode, tool, &args, input)?;
        recovery(&reply)?;
        if reply
            .pointer("/error/valid_values")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .any(|word| matches!(word.as_str(), Some("maude" | "reactions" | "devices")))
        {
            return Err(format!(
                "{mode}: recovery offers a component-disabled choice: {reply}"
            ));
        }
    }
    Ok(())
}

fn aggregate_component(server: &Server, mode: &str) -> Check {
    for aggregate in ["bogus", "reactions"] {
        let reply = query(
            server,
            mode,
            "search_adverse_event",
            &[
                "search",
                "adverse-event",
                "Synthetic medicine",
                "--aggregate",
                aggregate,
            ],
            json!({"text":"Synthetic medicine","aggregate":aggregate}),
        )?;
        if aggregate == "bogus" {
            recovery(&reply)?;
            if reply.pointer("/error/valid_values").is_some() {
                return Err(format!("disabled aggregation offered in recovery: {reply}"));
            }
        } else if reply.pointer("/sources/0/reason") != Some(&json!("disabled"))
            || !reply.to_string().contains("operator")
        {
            return Err(format!(
                "aggregation component refusal/cause changed: {reply}"
            ));
        }
    }
    Ok(())
}

fn component_card(server: &Server, mode: &str) -> Check {
    let card = query(
        server,
        mode,
        "get_adverse_event",
        &["get", "adverse-event", "100"],
        json!({"id":"100"}),
    )?;
    if card.get("status") != Some(&json!("complete")) {
        return Err(format!("card failed: {card}"));
    }
    if card
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|step| {
            step.get("args")
                .and_then(Value::as_array)
                .is_some_and(|args| args.contains(&json!("reactions")))
        })
    {
        return Err("disabled reaction section offered by next".into());
    }
    for (source, section) in [("faers", "reactions"), ("maude", "devices")] {
        let reply = query(
            server,
            mode,
            "get_adverse_event",
            &["get", "adverse-event", "100", section, "--source", source],
            json!({"id":"100","source":source,"section":section}),
        )?;
        if reply.pointer("/sources/0/reason") != Some(&json!("disabled"))
            || !reply.to_string().contains("operator")
        {
            return Err(format!("component refusal/cause changed: {reply}"));
        }
    }
    if mode == "single" {
        let reply = query(
            server,
            mode,
            "get_adverse_event",
            &["get", "adverse-event", "100", "--disable-sources", "faers"],
            json!({"id":"100"}),
        )?;
        let message = reply
            .pointer("/sources/0/notices")
            .and_then(Value::as_array)
            .and_then(|notices| notices.last())
            .and_then(|notice| notice.get("message"))
            .and_then(Value::as_str)
            .ok_or("request exclusion notice")?;
        if !message.contains("request")
            || message.contains("operator")
            || message.contains("ceiling")
        {
            return Err(format!(
                "unrelated MedDRA exclusion blamed for caller's card exclusion: {reply}"
            ));
        }
    }
    Ok(())
}

fn continuation(mode: &str) -> Check {
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
    let reply = query(
        &server,
        mode,
        "get_variant",
        &[
            "get",
            "variant",
            "CA123643",
            "civic",
            "--section-limit",
            "2",
            "--evidence-level",
            "A",
            "--evidence-status",
            "submitted",
        ],
        json!({"input":"CA123643","section":["civic"],"section_limit":2,"evidence_level":["A"],"evidence_status":"submitted"}),
    )?;
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
                "{mode}: CIViC continuation changed at {pointer}: {page}"
            ));
        }
    }
    Ok(())
}

fn recovery(reply: &Value) -> Check {
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

fn catalog(server: &Server) -> Result<Vec<Value>, String> {
    let (_, reply) = server.request(
        "POST",
        "/mcp",
        &request(json!(2), "tools/list", json!({})).to_string(),
    )?;
    reply
        .pointer("/result/tools")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| format!("missing catalog: {reply}"))
}

fn help(server: &Server, mode: &str, path: &str) -> Result<Value, String> {
    let words = std::iter::once("help")
        .chain(path.split_whitespace())
        .collect::<Vec<_>>();
    query(server, mode, "biomcp_help", &words, json!({"command":path}))
}

fn params(mode: &str, tool: &str, words: &[&str], input: Value) -> Value {
    if mode == "typed" {
        json!({"name":tool,"arguments":input})
    } else {
        json!({"name":"biomcp","arguments":{"args":words}})
    }
}

fn query(
    server: &Server,
    mode: &str,
    tool: &str,
    words: &[&str],
    input: Value,
) -> Result<Value, String> {
    call(server, params(mode, tool, words, input))
}

fn call(server: &Server, params: Value) -> Result<Value, String> {
    let frame = request(json!(1), "tools/call", params);
    let (_, reply) = server.request("POST", "/mcp", &frame.to_string())?;
    reply
        .pointer("/result/structuredContent")
        .cloned()
        .ok_or_else(|| format!("missing tool reply to {frame}: {reply}"))
}
