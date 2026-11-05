//! Feature device-approval-search: discover both sources and open returned identities in both MCP modes.
use super::super::{
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

type Check<T = ()> = Result<T, String>;

pub(super) fn check() -> Check {
    for mode in ["single", "typed"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&json!({"mcp_tool_mode":mode,"mcp_typed_output_schema":true}))
            .map_err(|e| e.to_string())?;
        discovery(&env, mode)?;
        for (source, name, manufacturer, field) in [
            (
                "fda-pma",
                "FoundationOne CDx",
                "Foundation Medicine",
                "pma_number",
            ),
            (
                "fda-510k",
                "FreeStyle Libre 3",
                "Abbott Diabetes",
                "k_number",
            ),
        ] {
            for (filter, value) in [("name", name), ("manufacturer", manufacturer)] {
                let search = call(
                    &env,
                    mode,
                    "search",
                    source,
                    &[format!("--{filter}"), value.into()],
                    json!({"source":source, filter:value}),
                    "device-approvals",
                )?;
                let rows = search
                    .pointer("/result/items")
                    .and_then(Value::as_array)
                    .filter(|rows| !rows.is_empty())
                    .ok_or_else(|| format!("{mode} {source} {filter}: {search}"))?;
                if filter == "name" {
                    open(&env, mode, source, &rows[0], field)?;
                }
            }
            let empty = call(
                &env,
                mode,
                "search",
                source,
                &["--name".into(), "NoSuchDevice0668".into()],
                json!({"source":source,"name":"NoSuchDevice0668"}),
                "device-search-roundtrip",
            )?;
            if empty
                .pointer("/result/items")
                .is_some_and(|items| items != &json!([]))
                || empty.pointer("/page/total") != Some(&json!(0))
            {
                return Err(format!("no match did not stay empty: {empty}"));
            }
        }
    }
    Ok(())
}

fn discovery(env: &Environment, mode: &str) -> Check {
    let catalog = exchange_with_globals(env, &[request(json!(1), "tools/list", json!({}))], &[])?;
    if mode == "typed" {
        let tool = catalog[0]
            .pointer("/result/tools")
            .and_then(Value::as_array)
            .and_then(|tools| {
                tools
                    .iter()
                    .find(|t| t.get("name") == Some(&json!("search_diagnostic")))
            })
            .ok_or("typed diagnostic search tool")?;
        let schema = tool.get("inputSchema").ok_or("typed input schema")?;
        let validator = jsonschema::validator_for(schema).map_err(|e| e.to_string())?;
        for source in ["fda-pma", "fda-510k"] {
            validator
                .validate(&json!({"source":source,"name":"Example"}))
                .map_err(|e| e.to_string())?;
            if !schema.to_string().contains(&format!("\"{source}\"")) {
                return Err(format!("source missing from typed input schema: {source}"));
            }
        }
    }
    let params = if mode == "single" {
        json!({"name":"biomcp","arguments":{"args":["help","search","diagnostic"]}})
    } else {
        json!({"name":"biomcp_help","arguments":{"command":"search diagnostic"}})
    };
    let replies = exchange_with_globals(env, &[request(json!(2), "tools/call", params)], &[])?;
    let help = replies[0]
        .pointer("/result/structuredContent")
        .ok_or("help result")?
        .to_string();
    for source in ["fda-pma", "fda-510k"] {
        if !help.contains(source) {
            return Err(format!("search help omits {source}"));
        }
    }
    Ok(())
}

fn call(
    env: &Environment,
    mode: &str,
    verb: &str,
    source: &str,
    extra: &[String],
    input: Value,
    capture: &str,
) -> Check<Value> {
    let params = if mode == "single" {
        let mut args = vec![verb.into(), "diagnostic".into()];
        args.extend_from_slice(extra);
        args.extend(["--source".into(), source.into()]);
        json!({"name":"biomcp","arguments":{"args":args}})
    } else {
        json!({"name":format!("{verb}_diagnostic"),"arguments":input})
    };
    let replay = fixtures().join("replies/diagnostic").join(capture);
    let replies = exchange_with_globals(
        env,
        &[request(json!(3), "tools/call", params)],
        &env.globals(Some(&replay), false),
    )?;
    let reply = &replies[0];
    let body = reply
        .pointer("/result/structuredContent")
        .ok_or_else(|| format!("missing structured result: {reply}"))?;
    if reply.pointer("/result/isError") != Some(&json!(false))
        || body.pointer("/sources/0/source") != Some(&json!(source))
        || !matches!(
            body.get("status").and_then(Value::as_str),
            Some("complete" | "empty")
        )
    {
        return Err(format!("{mode} {verb} {source}: {reply}"));
    }
    Ok(body.clone())
}

fn open(env: &Environment, mode: &str, source: &str, row: &Value, field: &str) -> Check {
    let header = row.get("header").ok_or("candidate header")?;
    let id = header
        .get(field)
        .and_then(Value::as_str)
        .ok_or("candidate ID")?;
    let mut args = vec![id.into()];
    let mut input = json!({"id":id,"source":source});
    if let Some(supplement) = header
        .get("supplement_number")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
    {
        args.extend(["--supplement".into(), supplement.into()]);
        input["supplement"] = json!(supplement);
    }
    let get = call(
        env,
        mode,
        "get",
        source,
        &args,
        input,
        "device-search-roundtrip",
    )?;
    let pointer = if source == "fda-pma" {
        format!("/result/header/{field}")
    } else {
        format!("/result/{field}")
    };
    if get.pointer(&pointer) != Some(&json!(id)) {
        return Err(format!("returned ID did not open: {get}"));
    }
    Ok(())
}
