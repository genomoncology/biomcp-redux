//! Research wrappers map the existing CLI result without provider calls.
use super::{
    launch::Environment,
    mcp::{exchange, request},
    trial_get::command,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("mcp/research-search-fetch-parity", || {
        parity().map_err(Into::into)
    })]
}
pub(super) fn call(
    env: &Environment,
    name: &str,
    arguments: Value,
    globals: &[String],
) -> Result<Value, String> {
    let replies = super::mcp::exchange_with_globals(
        env,
        &[request(json!(0), "tools/list", json!({})), request(
            json!(1),
            "tools/call",
            json!({"name":name,"arguments":arguments}),
        )],
        globals,
    )?;
    let result = replies
        .iter().find(|v| v.get("id") == Some(&json!(1)))
        .and_then(|v| v.get("result"))
        .ok_or("missing tool result")?;
    let value = result
        .get("structuredContent")
        .ok_or("missing structured content")?;
    let text = result
        .pointer("/content/0/text")
        .and_then(Value::as_str)
        .ok_or("missing text")?;
    if serde_json::from_str::<Value>(text).map_err(|e| e.to_string())? != *value {
        return Err("research text differs from structured content".into());
    }
    if result.get("isError") == Some(&json!(false)) {
        let tool = replies.iter().find(|v| v.get("id") == Some(&json!(0)))
            .and_then(|v| v.pointer("/result/tools")).and_then(Value::as_array)
            .and_then(|tools| tools.iter().find(|v| v.get("name") == Some(&json!(name))))
            .ok_or("tool definition missing")?;
        let schema = tool.get("outputSchema").ok_or("output schema missing")?;
        jsonschema::validator_for(schema).map_err(|e|e.to_string())?.validate(value).map_err(|e|e.to_string())?;
    }
    use std::io::Write;
    writeln!(std::io::stdout(), "research {name} compact_bytes={}", value.to_string().len()).map_err(|e|e.to_string())?;
    Ok(value.clone())
}
fn parity() -> Result<(), String> {
    for (query, args, capture) in [
        (
            "AKT1 E17K",
            vec!["search", "article", "AKT1 E17K"],
            "article/search/synthetic-card-step",
        ),
        (
            "trial --condition melanoma",
            vec!["search", "trial", "--condition", "melanoma"],
            "trial/condition",
        ),
        (
            "variant --query rs113488022 --source myvariant --build GRCh37",
            vec![
                "search",
                "variant",
                "--query",
                "rs113488022",
                "--source",
                "myvariant",
                "--build",
                "GRCh37",
            ],
            "variant/search/raw",
        ),
    ] {
        scenario(query, args, capture)?;
    }
    super::vaers::research_aggregate()?;
    invalid()
}
fn scenario(query: &str, args: Vec<&str>, capture: &str) -> Result<(), String> {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let recording = prepare(&env, capture)?;
        let globals = env.globals(Some(&recording), false);
        let args = args.into_iter().map(str::to_owned).collect::<Vec<_>>();
        let cli = command(&env, &args, &globals, 0)?;
        let wrapped = call(&env, "search", json!({"query":query}), &globals)?;
        if wrapped.pointer("/metadata/envelope") != Some(&cli) {
            return Err(format!("search metadata lost CLI fields: {wrapped}"));
        }
        let rows = cli
            .pointer("/result/items")
            .or_else(|| cli.pointer("/result/candidates"))
            .and_then(Value::as_array)
            .ok_or("CLI rows missing")?;
        let hits = wrapped
            .get("results")
            .and_then(Value::as_array)
            .ok_or("search results missing")?;
        if rows.len() != hits.len() || hits.is_empty() {
            return Err("research dropped source rows".into());
        }
        let hit = if capture == "variant/search/raw" {
            hits.get(1)
        } else {
            hits.first()
        }
        .ok_or("hit missing")?;
        let id = hit
            .get("id")
            .and_then(Value::as_str)
            .ok_or("hit id missing")?;
        if !id.starts_with(args.get(1).ok_or("entity missing")?)
            || hit
                .get("url")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
        {
            return Err("uncitable result".into());
        }
        fetch_parity(&env, &args, id, &recording)?;
    Ok(())
}
fn fetch_parity(env: &Environment, args: &[String], id: &str, recording: &std::path::Path) -> Result<(), String> {
        // Search and get captures are retained separately; exact ids come from the wrapper.
        let get_capture = match args.get(1).map(String::as_str) {
            Some("article") => "article/search/synthetic-card-step",
            Some("trial") => "trial/get/NCT02296125",
            _ => "variant/synthetic-components",
        };
        {
            super::response_cases::copy_capture(get_capture, &recording, |_, _| {})
                .map_err(|e| e.to_string())?;
            let fetch_globals = env.globals(Some(&recording), false);
            let mut get = vec!["get".to_owned()];
            get.extend(shlex::split(id).ok_or("invalid argument-list id")?);
            let cli_get = command(&env, &get, &fetch_globals, 0)?;
            let fetched = call(&env, "fetch", json!({"id":id}), &fetch_globals)?;
            if fetched.get("metadata") != Some(&cli_get) || fetched.get("id") != Some(&json!(id)) {
                return Err("fetch differs from CLI".into());
            }
            let mut md = get;
            md.extend(["--format".to_owned(), "md".to_owned()]);
            md.extend(fetch_globals);
            let output = env
                .command(env!("CARGO_BIN_EXE_biomcp"), &md)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            if fetched.get("text") != Some(&json!(text)) {
                return Err("fetch Markdown differs".into());
            }
        }
    Ok(())
}
fn prepare(env: &Environment, capture: &str) -> Result<std::path::PathBuf, String> {
        let recording = env.root().join("research-replay");
        super::response_cases::copy_capture(capture, &recording, |_, _| {})
            .map_err(|e| e.to_string())?;
        if capture == "trial/condition" {
            // Controlled projection of a retained native study into the search fixture.
            let raw = super::trial_get::original("NCT02296125").map_err(|e| e.to_string())?;
            super::response_cases::copy_capture(capture, &recording, |_, record| {
                if let Some(first) =
                    record.pointer_mut("/exchanges/0/body/value/studies/0/protocolSection")
                {
                    *first = raw.clone();
                }
            })
            .map_err(|e| e.to_string())?;
            super::response_cases::copy_capture("trial/get/NCT02296125", &recording, |_, _| {})
                .map_err(|e| e.to_string())?;
        }
    Ok(recording)
}
fn invalid() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for (id, args) in [
        ("", vec!["get"]),
        ("article nonsense", vec!["get", "article", "nonsense"]),
        ("gene", vec!["get", "gene"]),
    ] {
        let cli = command(
            &env,
            &args.into_iter().map(str::to_owned).collect::<Vec<_>>(),
            &[],
            2,
        )?;
        if call(&env, "fetch", json!({"id":id}), &[])? != cli {
            return Err("fetch usage differs from CLI".into());
        }
    }
    let replies = exchange(
        &env,
        &[request(
            json!(1),
            "tools/call",
            json!({"name":"search", "arguments":{"query":3}}),
        )],
    )?;
    if replies.first().and_then(|r| r.pointer("/error/code")) != Some(&json!(-32602)) {
        return Err("invalid input accepted".into());
    }
    Ok(())
}
