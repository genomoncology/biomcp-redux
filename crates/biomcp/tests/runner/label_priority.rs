//! Feature drug-label: native indication discovery precedes incidental text in both tool modes.
use super::super::{
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};

type Check = Result<(), String>;

pub(super) fn check() -> Check {
    let mut failures = Vec::new();
    for mode in ["single", "typed"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        env.config(&json!({"mcp_tool_mode":mode}))
            .map_err(|e| e.to_string())?;
        for (query, offset, total, names) in [
            (
                "pancreatic",
                0,
                3,
                vec!["Indication therapy", "Native unnamed therapy"],
            ),
            (
                "pancreatic",
                2,
                3,
                vec!["00000000-0000-4000-8000-000000000003"],
            ),
            ("pancreatic", 3, 3, vec![]),
            ("melanoma", 0, 1, vec!["Melanoma reference formulation"]),
        ] {
            let reply = search(&env, mode, query, offset)?;
            let actual = reply
                .pointer("/result/items")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|row| row.get("name").and_then(Value::as_str))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            if actual != names
                || reply.pointer("/page/total") != Some(&json!(total))
                || reply.pointer("/result/total") != Some(&json!(total))
                || reply.pointer("/page/returned") != Some(&json!(names.len()))
                || reply.get("status") != Some(&json!("complete"))
            {
                failures.push(format!("{mode} {query} offset {offset}: expected names {names:?} and total {total}; got {reply}"));
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

fn search(env: &Environment, mode: &str, query: &str, offset: u32) -> Result<Value, String> {
    let params = if mode == "single" {
        json!({"name":"biomcp","arguments":{"args":["search","drug",query,"--source","openfda-label","--limit","2","--offset",offset.to_string()]}})
    } else {
        json!({"name":"search_drug","arguments":{"query":query,"source":"openfda-label","limit":2,"offset":offset}})
    };
    let replay = fixtures().join("replies/drug/label-priority");
    let replies = exchange_with_globals(
        env,
        &[request(json!(1), "tools/call", params)],
        &env.globals(Some(&replay), false),
    )?;
    let reply = &replies[0];
    if reply.pointer("/result/isError") != Some(&json!(false)) {
        return Err(format!("search failed: {reply}"));
    }
    reply
        .pointer("/result/structuredContent")
        .cloned()
        .ok_or("search envelope missing".into())
}
