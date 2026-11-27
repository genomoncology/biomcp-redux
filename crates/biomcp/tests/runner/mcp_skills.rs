//! The published research skill as seen by stdio and HTTP MCP clients.
use crate::runner::{
    http::Server,
    launch::{Environment, checkout},
    mcp::{exchange_with_globals, request},
    transcript::trial,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs};

const ROOT: &str = "skill://biomcp-research/";
const GUIDE: &str = "skill://biomcp-research/SKILL.md";
const UNKNOWN: &str = "skill://unknown/SKILL.md";
const EXTENSION: &str = "/result/capabilities/extensions/io.modelcontextprotocol~1skills";

type Check = Result<(), String>;

pub(super) fn trials() -> [Trial; 4] {
    [
        trial("mcp-skills/stdio-manifest", || stdio().map_err(Into::into)),
        trial("mcp-skills/http-manifest", || http().map_err(Into::into)),
        trial("mcp-skills/hosted-exclusions", || {
            hosted().map_err(Into::into)
        }),
        trial("mcp-skills/legacy-resources-and-opt-out", || {
            legacy_and_opt_out().map_err(Into::into)
        }),
    ]
}

fn discover() -> Value {
    request(json!("discover"), "server/discover", json!({}))
}

fn list() -> Value {
    request(json!("list"), "skills/list", json!({}))
}

fn asset_request(method: &str, uri: &str, id: &str) -> Value {
    request(json!(id), method, json!({"uri":uri}))
}

fn manifest_requests() -> [Value; 6] {
    [
        discover(),
        list(),
        asset_request("skills/get", GUIDE, "get"),
        asset_request("skills/get", UNKNOWN, "missing-skill"),
        asset_request("resources/read", UNKNOWN, "missing-resource"),
        request(json!("resources"), "resources/list", json!({})),
    ]
}

fn stdio() -> Check {
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let exchange =
        |frames: &[Value]| exchange_with_globals(&environment, frames, &["--offline".into()]);
    let replies = exchange(&manifest_requests())?;
    let files = manifest(&replies)?;
    let reads = files
        .iter()
        .map(|file| uri(file).map(|uri| asset_request("resources/read", uri, uri)))
        .collect::<Result<Vec<_>, String>>()?;
    let contents = exchange(&reads)?;
    for file in files {
        let name = uri(file)?;
        check_file(file, reply(&contents, name)?, &replies)?;
    }
    Ok(())
}

fn http() -> Check {
    let server =
        Server::configured_globals("127.0.0.1:0", None, None, None, None, &["--offline".into()])?;
    let replies = manifest_requests()
        .iter()
        .map(|frame| post(&server, frame))
        .collect::<Result<Vec<_>, _>>()?;
    for file in manifest(&replies)? {
        let read = post(
            &server,
            &asset_request("resources/read", uri(file)?, "read"),
        )?;
        check_file(file, &read, &replies)?;
    }
    Ok(())
}

fn hosted() -> Check {
    let config = json!({"disable_sources":["semantic-scholar"]});
    let server = Server::configured_globals(
        "127.0.0.1:0",
        Some(config),
        None,
        None,
        None,
        &["--offline".into(), "--hosted".into()],
    )?;
    let listed = post(&server, &list())?;
    let result = &listed["result"];
    check_cache(result, "private")?;
    let files = result
        .pointer("/skills/0/resources")
        .and_then(Value::as_array)
        .ok_or("missing hosted manifest")?;
    let omitted = format!("{ROOT}references/04-article-follow-up.md");
    let kept = format!("{ROOT}references/17-exact-variant-literature.md");
    let uris = uris(files)?;
    if uris.contains(omitted.as_str()) || !uris.contains(kept.as_str()) {
        return Err(
            "hosted skill must omit the Semantic Scholar follow-up and keep the PubMed example"
                .into(),
        );
    }
    let resources = post(
        &server,
        &request(json!("resources"), "resources/list", json!({})),
    )?;
    match_resources(&resources, uris)?;
    let missing = post(
        &server,
        &asset_request("resources/read", &omitted, "excluded"),
    )?;
    if missing.pointer("/error/code") != Some(&json!(-32602)) {
        return Err("hosted resources/read served an omitted reference".into());
    }
    Ok(())
}

fn legacy_and_opt_out() -> Check {
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let globals = ["--offline".into()];
    for revision in ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"] {
        let frames = [
            json!({"jsonrpc":"2.0","id":"init","method":"initialize","params":{
                "protocolVersion":revision,"capabilities":{},"clientInfo":{"name":"test","version":"1"}}}),
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            json!({"jsonrpc":"2.0","id":"resources","method":"resources/list"}),
            json!({"jsonrpc":"2.0","id":"read","method":"resources/read","params":{"uri":GUIDE}}),
        ];
        let replies = exchange_with_globals(&environment, &frames, &globals)?;
        let init = reply(&replies, "init")?;
        let resources = reply(&replies, "resources")?;
        let read = reply(&replies, "read")?;
        if init["result"]["protocolVersion"] != revision
            || init.pointer(EXTENSION).is_some()
            || !init["result"]["capabilities"]["resources"].is_object()
            || !resources["result"]["resources"]
                .as_array()
                .is_some_and(|files| files.iter().any(|file| file["uri"] == GUIDE))
            || read.pointer("/result/contents/0/uri") != Some(&json!(GUIDE))
        {
            return Err(format!(
                "{revision} did not advertise and serve legacy skill resources"
            ));
        }
    }

    environment
        .config(&json!({"mcp_skills":false}))
        .map_err(|error| error.to_string())?;
    let replies = exchange_with_globals(&environment, &[discover(), list()], &globals)?;
    let disabled = reply(&replies, "discover")?;
    if disabled["result"]["capabilities"] != json!({"tools":{}})
        || instructions(disabled)?.contains(GUIDE)
        || reply(&replies, "list")?.pointer("/error/code") != Some(&json!(-32601))
    {
        return Err(
            "mcp_skills=false must withdraw skill discovery and the instructions pointer".into(),
        );
    }
    Ok(())
}

fn manifest(replies: &[Value]) -> Result<&[Value], String> {
    let discover = reply(replies, "discover")?;
    if discover.pointer(EXTENSION) != Some(&json!({"directoryRead":false}))
        || !discover["result"]["capabilities"]["resources"].is_object()
        || !instructions(discover)?.contains(GUIDE)
    {
        return Err("discovery must declare the extension, resources and a skill pointer".into());
    }
    let result = &reply(replies, "list")?["result"];
    let get = &reply(replies, "get")?["result"];
    check_cache(result, "public")?;
    check_cache(get, "public")?;
    let skills = result["skills"].as_array().ok_or("missing skills list")?;
    let [skill] = skills.as_slice() else {
        return Err("expected one research skill".into());
    };
    if skill["uri"] != GUIDE || get["skill"] != *skill {
        return Err("skills/get and skills/list disagree on the research skill".into());
    }
    for id in ["missing-skill", "missing-resource"] {
        if reply(replies, id)?.pointer("/error/code") != Some(&json!(-32602)) {
            return Err(format!("{id} must return -32602"));
        }
    }
    let files = skill["resources"]
        .as_array()
        .ok_or("missing file manifest")?;
    let listed = uris(files)?;
    let expected = expected_uris()?;
    if listed != expected.iter().map(String::as_str).collect() {
        return Err(format!("incomplete research skill manifest: {listed:?}"));
    }
    match_resources(reply(replies, "resources")?, listed)?;
    Ok(files)
}

fn expected_uris() -> Result<BTreeSet<String>, String> {
    let mut uris = BTreeSet::from([GUIDE.to_owned()]);
    for entry in fs::read_dir(use_cases()).map_err(|error| error.to_string())? {
        let name = entry.map_err(|error| error.to_string())?.file_name();
        let name = name.to_str().ok_or("invalid use-case file name")?;
        if name.ends_with(".md") && !name.starts_with('_') {
            uris.insert(format!("{ROOT}references/{name}"));
        }
    }
    Ok(uris)
}

fn check_file(file: &Value, read: &Value, replies: &[Value]) -> Check {
    let name = uri(file)?;
    let content = read
        .pointer("/result/contents/0")
        .ok_or_else(|| format!("missing contents for {name}"))?;
    let text = content["text"].as_str().ok_or("missing resource text")?;
    let expected = if name == GUIDE {
        let frontmatter = text
            .strip_prefix("---\n")
            .and_then(|text| text.split_once("\n---\n"))
            .ok_or("missing guide frontmatter")?
            .0;
        let fields: Value =
            serde_saphyr::from_str(frontmatter).map_err(|error| error.to_string())?;
        if reply(replies, "get")?.pointer("/result/skill/frontmatter") != Some(&fields) {
            return Err("guide frontmatter and manifest disagree".into());
        }
        include_str!("../../src/skill/biomcp-research/SKILL.md").to_owned()
    } else {
        let name = name
            .strip_prefix(&format!("{ROOT}references/"))
            .ok_or("expected a reference URI")?;
        fs::read_to_string(use_cases().join(name)).map_err(|error| error.to_string())?
    };
    if content["uri"] != name
        || content["mimeType"] != "text/markdown"
        || text != expected
        || file["size"] != text.len()
        || file["digest"] != format!("sha256:{:x}", Sha256::digest(text))
    {
        return Err(format!(
            "resource bytes, size or digest disagree for {name}"
        ));
    }
    Ok(())
}

fn use_cases() -> std::path::PathBuf {
    checkout().join("../../skills/use-cases")
}

fn match_resources(reply: &Value, expected: BTreeSet<&str>) -> Check {
    let resources = reply
        .pointer("/result/resources")
        .and_then(Value::as_array)
        .ok_or("missing resources/list")?;
    if uris(resources)? != expected {
        return Err("resources/list and the skill manifest disagree".into());
    }
    Ok(())
}

fn uris(files: &[Value]) -> Result<BTreeSet<&str>, String> {
    let unique = files.iter().map(uri).collect::<Result<BTreeSet<_>, _>>()?;
    if unique.len() != files.len() {
        return Err("duplicate resource URI".into());
    }
    Ok(unique)
}

fn uri(file: &Value) -> Result<&str, String> {
    file["uri"].as_str().ok_or_else(|| "missing URI".into())
}

fn reply<'a>(replies: &'a [Value], id: &str) -> Result<&'a Value, String> {
    replies
        .iter()
        .find(|reply| reply["id"] == id)
        .ok_or_else(|| format!("missing reply {id}"))
}

fn instructions(reply: &Value) -> Result<&str, String> {
    reply
        .pointer("/result/instructions")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing instructions".into())
}

fn check_cache(result: &Value, scope: &str) -> Check {
    if result["resultType"] != "complete"
        || result["cacheScope"] != scope
        || result["ttlMs"].as_u64().is_none_or(|ttl| ttl == 0)
    {
        return Err(format!(
            "expected a complete result with a positive TTL and {scope} cache scope"
        ));
    }
    Ok(())
}

fn post(server: &Server, frame: &Value) -> Result<Value, String> {
    let (status, reply) = server.request("POST", "/mcp", &frame.to_string())?;
    if status != 200 {
        return Err(format!(
            "{} returned HTTP {status}: {reply}",
            frame["method"]
        ));
    }
    Ok(reply)
}
