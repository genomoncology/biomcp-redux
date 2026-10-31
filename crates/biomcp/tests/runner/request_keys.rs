//! Feature mcp-hosted: each hosted call owns its source credentials.
use super::{http::Server, launch::Environment};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check<T = ()> = Result<T, String>;
const CALLER: &str = "synthetic-request-caller";
const SERVER: &str = "synthetic-request-server";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("mcp-hosted/keys/discovery", || {
            discovery().map_err(Into::into)
        }),
        transcript_trial("mcp-hosted/keys/licensed-request-snapshot", || {
            licensed().map_err(Into::into)
        }),
        transcript_trial("mcp-hosted/keys/public-wire-fallback", || {
            public().map_err(Into::into)
        }),
    ]
}
fn discovery() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listed = super::key_registration::invoke(&env, &["key", "list"], "", 0)?;
    let entries = listed
        .pointer("/result/entries")
        .and_then(Value::as_array)
        .ok_or("missing entries")?;
    for entry in entries {
        let config = entry["config"].as_str().ok_or("missing config name")?;
        let header = format!(
            "x-biomcp-key-{}",
            config
                .strip_prefix("keys.")
                .ok_or("config prefix")?
                .replace('_', "-")
        );
        let tier = if matches!(entry["source"].as_str(), Some("oncokb" | "alphagenome")) {
            "no-fallback"
        } else {
            "fallback"
        };
        if entry.get("header") != Some(&json!(header)) || entry.get("tier") != Some(&json!(tier)) {
            return Err("key discovery omitted its header or declared tier".into());
        }
    }
    Ok(())
}
fn call(server: &Server, args: &[&str], extra: &str, extra_argument: bool) -> Check<Value> {
    let mut arguments = json!({"args":args});
    if extra_argument {
        arguments
            .as_object_mut()
            .ok_or("arguments object")?
            .insert("ncbi_api_key".into(), json!(CALLER));
    }
    let frame = json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"biomcp","arguments":arguments}});
    let headers = format!(
        "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\n{extra}"
    );
    let (status, reply, _) = server.raw(
        "POST",
        "/mcp",
        &server.address.to_string(),
        &headers,
        &frame.to_string(),
    )?;
    let reply = reply.ok_or("missing reply")?;
    if status != 200
        || [CALLER, SERVER]
            .iter()
            .any(|key| reply.to_string().contains(key))
    {
        return Err("request rejected or credential escaped in reply".into());
    }
    if let Some(context) = super::transcript::current() {
        let args: Vec<String> = args.iter().map(|arg| (*arg).into()).collect();
        let key = context.allocate(&args);
        context.record(
            &key,
            json!({"args":args,"surface":"http","http_status":status,
            "supplied_header":!extra.is_empty(),"extra_argument":extra_argument,
            "reply":reply}),
        )?;
    }
    Ok(reply)
}
fn licensed() -> Check {
    let replay = super::launch::fixtures().join("replies/gene/oncokb-synthetic/data");
    for hosted in [true, false] {
        let globals = if hosted {
            vec!["--hosted".into()]
        } else {
            vec![]
        };
        let mut server = Server::configured_globals(
            "127.0.0.1:0",
            Some(json!({"keys":{"oncokb_api_key":SERVER}})),
            Some(replay.clone()),
            None,
            None,
            &globals,
        )?;
        let args = ["get", "gene", "TP53", "oncokb"];
        for header in [false, true, false] {
            let extra = if header {
                format!("X-BioMCP-Key-Oncokb-Api-Key: {CALLER}\r\n")
            } else {
                String::new()
            };
            let reply = call(&server, &args, &extra, false)?;
            if hosted && !header {
                if reply.pointer("/result/structuredContent/error/message")
                    != Some(&json!("add the source key in your account settings"))
                    || reply.pointer("/result/structuredContent/error/source")
                        != Some(&json!("oncokb"))
                {
                    return Err(
                        "licensed source used a server key or omitted account guidance".into(),
                    );
                }
            } else if reply
                .pointer("/result/structuredContent/result/items/0/sections/oncokb/state")
                != Some(&json!("data"))
            {
                return Err("request credential failed to admit recorded licensed reply".into());
            }
        }
        if hosted {
            concurrent(&server, &args)?;
        }
        let reply = call(&server, &args, "", true)?;
        if reply.pointer("/error/code") != Some(&json!(-32602)) {
            return Err("tool argument accepted a source key".into());
        }
        header_boundaries(&server, hosted)?;
        server.stop()?;
    }
    Ok(())
}
fn header_boundaries(server: &Server, hosted: bool) -> Check {
    let frame = json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"biomcp","arguments":{"args":["help"]}}});
    for extra in [
        format!(
            "x-biomcp-key-oncokb-api-key: {CALLER}\r\nX-BIOMCP-KEY-ONCOKB-API-KEY: {SERVER}\r\n"
        ),
        "x-biomcp-key-oncokb-api-key: \r\n".into(),
    ] {
        let headers = format!(
            "Content-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2025-11-25\r\n{extra}"
        );
        let (status, reply, _) = server.raw(
            "POST",
            "/mcp",
            &server.address.to_string(),
            &headers,
            &frame.to_string(),
        )?;
        if status != if hosted { 400 } else { 200 }
            || reply.as_ref().is_some_and(|value| {
                [CALLER, SERVER]
                    .iter()
                    .any(|key| value.to_string().contains(key))
            })
        {
            return Err("ambiguous credential headers were accepted or reflected".into());
        }
    }
    Ok(())
}
fn concurrent(server: &Server, args: &[&str]) -> Check {
    let extra = format!("x-biomcp-key-oncokb-api-key: {CALLER}\r\n");
    let (keyed, unkeyed) = std::thread::scope(|scope| {
        let keyed = scope.spawn(|| call(server, args, &extra, false));
        let unkeyed = scope.spawn(|| call(server, args, "", false));
        Ok::<_, String>((
            keyed.join().map_err(|_| "keyed caller stopped")??,
            unkeyed.join().map_err(|_| "unkeyed caller stopped")??,
        ))
    })?;
    if keyed.pointer("/result/structuredContent/result/items/0/sections/oncokb/state")
        != Some(&json!("data"))
        || unkeyed.pointer("/result/structuredContent/error/source") != Some(&json!("oncokb"))
    {
        return Err("concurrent callers shared a credential snapshot".into());
    }
    Ok(())
}
fn public() -> Check {
    let assets = Environment::new().map_err(|e| e.to_string())?;
    let stub = super::network_stub::Stub::for_source(
        "api.semanticscholar.org",
        "application/json",
        |_| Ok(r#"{"total":0,"data":[]}"#.into()),
    )?;
    stub.retain(&assets);
    let ca = assets.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    for hosted in [true, false] {
        let globals = if hosted {
            vec!["--hosted".into()]
        } else {
            vec![]
        };
        let mut server = Server::configured_globals(
            "127.0.0.1:0",
            Some(
                json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,"keys":{"semantic_scholar_api_key":SERVER}}),
            ),
            None,
            None,
            None,
            &globals,
        )?;
        for (index, supplied) in [true, false].into_iter().enumerate() {
            let query = format!("synthetic request {hosted} {index}");
            let extra = if supplied {
                format!("x-biomcp-key-semantic-scholar-api-key: {CALLER}\r\n")
            } else {
                String::new()
            };
            call(
                &server,
                &["search", "article", &query, "--source", "semantic-scholar"],
                &extra,
                false,
            )?;
        }
        server.stop()?;
    }
    let requests = stub.finish()?;
    if requests.len() != 4 {
        return Err("public source wire count".into());
    }
    for ((_, request), expected) in requests.iter().zip([CALLER, SERVER, SERVER, SERVER]) {
        let value = request.lines().find_map(|line| {
            line.split_once(':')
                .filter(|(name, _)| name.eq_ignore_ascii_case("x-api-key"))
                .map(|(_, value)| value.trim())
        });
        if value != Some(expected) {
            return Err("public source selected wrong request credential".into());
        }
    }
    Ok(())
}
