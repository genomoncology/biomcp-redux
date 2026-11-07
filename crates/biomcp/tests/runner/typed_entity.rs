//! Every entity tool and action reaches its registry command through stdio and hosted HTTP.
//! live_check: offline; each call returns its command's own offline reply, with no network.
use super::super::{
    http::Server,
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};
use std::io::Write;

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let config = json!({"mcp_tool_mode":"typed"});
    env.config(&config).map_err(|e| e.to_string())?;
    let offline = vec!["--offline".to_owned()];
    let hosted = vec!["--offline".to_owned(), "--hosted".to_owned()];
    let list = request(json!("list"), "tools/list", json!({}));
    let local = tools(&exchange_with_globals(
        &env,
        std::slice::from_ref(&list),
        &offline,
    )?)?;
    super::typed_catalog::check(&local)?;
    let server =
        Server::configured_globals("127.0.0.1:0", Some(config), None, None, None, &hosted)?;
    let (_, served) = server.request("POST", "/mcp", &list.to_string())?;
    let served = tools(std::slice::from_ref(&served))?;
    let names = served.iter().map(|tool| &tool["name"]).collect::<Vec<_>>();
    if names != super::typed_catalog::TOOLS {
        return Err(format!("hosted HTTP serves other tools: {names:?}"));
    }
    super::typed_catalog::coverage_schemas(&served)?;
    report(&local, &served)?;

    let calls = super::typed_catalog::MAP
        .iter()
        .map(|&(tool, action, command)| {
            let (verb, entity, id) = path(command)?;
            let schema = cli(&env, &["schema".into(), id.into()])?;
            let values = crate::runner::enum_case::seed(verb, entity, &schema)?;
            let argv = crate::runner::enum_case::argv(verb, entity, &schema, &values);
            Ok((tool, action, values, argv))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let frames = calls
        .iter()
        .enumerate()
        .map(|(index, (tool, action, values, _))| {
            request(
                json!(index),
                "tools/call",
                json!({"name":tool,"arguments":{"action":action,*action:values}}),
            )
        })
        .collect::<Vec<_>>();
    // The server runs at most 32 calls at once, so send them in batches.
    let mut stdio = Vec::new();
    for batch in frames.chunks(30) {
        stdio.extend(exchange_with_globals(&env, batch, &offline)?);
    }
    let mut failures = Vec::new();
    for (index, ((tool, action, _, argv), frame)) in calls.iter().zip(&frames).enumerate() {
        let label = format!("{tool}.{action}");
        let expected = cli(&env, argv)?;
        let raw = stdio.iter().find(|reply| reply["id"] == json!(index));
        let Some(reply) = raw.and_then(|reply| reply.pointer("/result/structuredContent")) else {
            failures.push(format!("{label}: no stdio tool result: {raw:?}"));
            continue;
        };
        if super::typed_followups::data(reply) != super::typed_followups::data(&expected) {
            failures.push(format!("{label} stdio differs from CLI {argv:?}: {reply}"));
        }
        let (status, http) = server.request("POST", "/mcp", &frame.to_string())?;
        let http = http
            .pointer("/result/structuredContent")
            .filter(|_| status == 200)
            .ok_or_else(|| format!("{label}: no hosted reply: {http}"))?;
        if let Err(error) = reached(http, &expected) {
            failures.push(format!("{label} hosted HTTP: {error}"));
        }
    }
    writeln!(
        std::io::stdout(),
        "TYPED_ENTITY_CALLS actions={} failures={}",
        calls.len(),
        failures.len()
    )
    .map_err(|e| e.to_string())?;
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

/// A hosted reply reaches the same command: the same canonical query, or a hosted refusal
/// for an action hosting does not serve.
/// Hosted HTTP reaches the same command, or meets the existing hosted refusal.
fn reached(http: &Value, expected: &Value) -> Result<(), String> {
    if http.pointer("/error/code") == Some(&json!("usage.mcp_unavailable")) {
        return Ok(());
    }
    match (http.get("query"), expected.get("query")) {
        (Some(actual), Some(query)) if actual == query => Ok(()),
        (None, None) if http.get("schema") == expected.get("schema") => Ok(()),
        _ => Err(format!(
            "reply differs from its command: {http} versus {expected}"
        )),
    }
}

fn path(command: &str) -> Result<(&'static str, &'static str, &'static str), String> {
    biomcp_types::command_paths()
        .into_iter()
        .find(|(verb, entity, _)| format!("{verb}_{entity}").replace('-', "_") == command)
        .ok_or_else(|| format!("{command}: not a registry command"))
}

fn tools(replies: &[Value]) -> Result<Vec<Value>, String> {
    replies
        .first()
        .and_then(|reply| reply.pointer("/result/tools"))
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| format!("tools/list failed: {replies:?}"))
}

fn cli(env: &Environment, args: &[String]) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| format!("CLI {args:?}: {e}"))
}

/// The list size an agent loads, for the ticket's report.
fn report(local: &[Value], served: &[Value]) -> Result<(), String> {
    let bytes = |tools: &[Value]| serde_json::to_vec(tools).map_or(0, |bytes| bytes.len());
    let mut out = std::io::stdout().lock();
    for tool in local {
        writeln!(
            out,
            "TYPED_ENTITY_TOOL {} definition_bytes={} description_characters={}",
            tool["name"],
            bytes(std::slice::from_ref(tool)),
            tool["description"]
                .as_str()
                .map_or(0, |text| text.chars().count())
        )
        .map_err(|e| e.to_string())?;
    }
    writeln!(
        out,
        "TYPED_ENTITY_LIST local_bytes={} hosted_bytes={}",
        bytes(local),
        bytes(served)
    )
    .map_err(|e| e.to_string())
}
