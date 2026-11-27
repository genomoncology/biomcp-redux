//! Cache hints on the catalog replies clients receive over each transport.
use crate::runner::{
    http::Server,
    launch::Environment,
    mcp::{exchange_with_globals, request},
    transcript::trial,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> [Trial; 2] {
    [
        trial("mcp-parity/stdio-catalog-cache-fields", || {
            stdio().map_err(Into::into)
        }),
        trial("mcp-http/catalog-cache-fields", || {
            http().map_err(Into::into)
        }),
    ]
}

fn catalog_requests() -> [Value; 2] {
    [
        request(json!(1), "server/discover", json!({})),
        request(json!(2), "tools/list", json!({})),
    ]
}

fn scopes() -> [(Vec<String>, &'static str); 2] {
    [
        (vec!["--offline".into()], "public"),
        (vec!["--offline".into(), "--hosted".into()], "private"),
    ]
}

fn stdio() -> Result<(), String> {
    for (globals, scope) in scopes() {
        let environment = Environment::new().map_err(|error| error.to_string())?;
        let frames = catalog_requests();
        let replies = exchange_with_globals(&environment, &frames, &globals)?;
        for frame in frames {
            let reply = replies
                .iter()
                .find(|reply| reply["id"] == frame["id"])
                .ok_or("missing stdio catalog reply")?;
            check_fields(&frame, reply, scope)?;
        }
    }
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let replies = exchange_with_globals(
        &environment,
        &[
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
                "protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}),
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        ],
        &["--offline".into()],
    )?;
    if replies.first().and_then(|reply| reply.pointer("/result/protocolVersion"))
        != Some(&json!("2025-11-25"))
    {
        return Err("expected a 2025-11-25 initialization result".into());
    }
    let list = replies
        .get(1)
        .and_then(|reply| reply.get("result"))
        .ok_or("missing legacy tools/list result")?;
    if !list["tools"].is_array() || list.get("ttlMs").is_some() || list.get("cacheScope").is_some() {
        return Err("expected a 2025-11-25 tool list without cache fields".into());
    }
    Ok(())
}

fn http() -> Result<(), String> {
    for (globals, scope) in scopes() {
        let server =
            Server::configured_globals("127.0.0.1:0", None, None, None, None, &globals)?;
        for frame in catalog_requests() {
            let (status, reply) = server.request("POST", "/mcp", &frame.to_string())?;
            if status != 200 {
                return Err(format!("HTTP {} returned {status}", frame["method"]));
            }
            check_fields(&frame, &reply, scope)?;
        }
    }
    Ok(())
}

fn check_fields(frame: &Value, reply: &Value, scope: &str) -> Result<(), String> {
    let result = &reply["result"];
    if result["ttlMs"].as_u64().is_none_or(|ttl| ttl == 0)
        || result["cacheScope"].as_str() != Some(scope)
    {
        return Err(format!(
            "{} expected a positive integer ttlMs and {scope} cacheScope, got {} and {}",
            frame["method"], result["ttlMs"], result["cacheScope"]
        ));
    }
    Ok(())
}
