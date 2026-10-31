//! Feature mcp-hosted: licensed replies belong to the caller's effective key.
use super::{http::Server, launch::Environment, network_stub::Stub, request_keys::call};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
const FIRST: &str = "synthetic-cache-caller-a";
const SECOND: &str = "synthetic-cache-caller-b";
const STATE: &str = "/result/structuredContent/result/candidates/0/sections/oncokb/state";

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "mcp-hosted/cache/licensed-callers",
        || licensed().map_err(Into::into),
    )]
}

fn request(server: &Server, key: Option<&str>, offline: bool) -> Result<Value, String> {
    let args = super::oncokb::args();
    let mut args: Vec<_> = args.iter().map(String::as_str).collect();
    if offline {
        args.push("--offline");
    }
    let header = key
        .map(|key| format!("x-biomcp-key-oncokb-api-key: {key}\r\n"))
        .unwrap_or_default();
    let reply = call(server, &args, &header, false)?;
    if [FIRST, SECOND]
        .iter()
        .any(|key| reply.to_string().contains(key))
    {
        return Err("licensed credential escaped in reply".into());
    }
    Ok(reply)
}

fn licensed() -> Result<(), String> {
    for hosted in [true, false] {
        let assets = Environment::new().map_err(|e| e.to_string())?;
        let stub = Stub::for_source("www.oncokb.org", "application/json", |request| {
            let mut reply: Value =
                serde_json::from_str(&super::oncokb::reply(request)?).map_err(|e| e.to_string())?;
            reply["variantSummary"] = json!(if request.contains(SECOND) {
                "Synthetic caller B result"
            } else {
                "Synthetic caller A result"
            });
            Ok(reply.to_string())
        })?;
        stub.retain(&assets);
        let ca = assets.root().join("ca.pem");
        std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
        let globals = if hosted {
            vec!["--hosted".into()]
        } else {
            vec![]
        };
        let mut server = Server::configured_globals(
            "127.0.0.1:0",
            Some(
                json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,
                "keys":{"oncokb_api_key":FIRST}}),
            ),
            None,
            None,
            None,
            &globals,
        )?;
        let first = request(&server, Some(FIRST), false)?;
        if first.pointer(STATE) != Some(&json!("data")) {
            return Err("caller A did not fetch the licensed reply".into());
        }
        if request(&server, Some(FIRST), true)?.pointer(STATE) != Some(&json!("data")) {
            return Err("caller A did not retain its own licensed cached reply".into());
        }
        let other = request(&server, Some(SECOND), true)?;
        if hosted && other.pointer(STATE) == Some(&json!("data")) {
            return Err("caller B read caller A's licensed cache while offline".into());
        }
        let second = request(&server, Some(SECOND), false)?;
        if second.pointer(STATE) != Some(&json!("data"))
            || !second.to_string().contains(if hosted {
                "Synthetic caller B result"
            } else {
                "Synthetic caller A result"
            })
        {
            return Err("licensed reply did not follow the effective caller key".into());
        }
        let keyless = request(&server, None, false)?;
        if hosted
            && keyless.pointer("/result/structuredContent/error/source") != Some(&json!("oncokb"))
        {
            return Err("keyless caller received licensed cached data".into());
        }
        server.stop()?;
        let requests = stub.finish()?;
        if requests.len() != if hosted { 2 } else { 1 } {
            return Err("licensed cache fetched the wrong number of replies".into());
        }
    }
    Ok(())
}
