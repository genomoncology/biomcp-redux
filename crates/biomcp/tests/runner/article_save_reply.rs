//! Prepare retained native replies for the cut-content save fixture.
use super::launch::fixtures;
use serde_json::Value;
use std::fs;

pub(super) fn cut_reply(request: &str) -> Result<String, String> {
    if request.contains(".xlsx?") {
        return Ok("PK\x03".into());
    }
    let target = request.split_whitespace().nth(1).ok_or("request target")?;
    for file in fs::read_dir(fixtures().join("replies/article/supplements/live/pmc-oa"))
        .map_err(|e| e.to_string())?
    {
        let path = file.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|v| v != "json") {
            continue;
        }
        let record: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if record
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| url.ends_with(target))
        {
            let body = record.pointer("/exchanges/0/body/value").ok_or("body")?;
            return Ok(body
                .as_str()
                .map_or_else(|| body.to_string(), str::to_owned));
        }
    }
    Err(format!("unrecorded local request {target}"))
}

pub(super) fn unknown_acquisition() -> Result<(), String> {
    let env = super::launch::Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("legacy-article");
    let directory = replay.join("pubmed");
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    for file in fs::read_dir(fixtures().join("replies/article/get/synthetic-derived/pubmed"))
        .map_err(|e| e.to_string())?
    {
        let file = file.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        for exchange in value.get_mut("exchanges").and_then(Value::as_array_mut).ok_or("exchanges")? {
            exchange
                .as_object_mut()
                .ok_or("exchange")?
                .remove("fetched");
        }
        fs::write(directory.join(file.file_name()), value.to_string())
            .map_err(|e| e.to_string())?;
    }
    let mut args = env.globals(Some(&replay), false);
    args.extend(["get", "article", "33123537"].map(str::to_owned));
    let value = super::configuration::invoke(&env, args, 0)?;
    let words = ["get", "article", "33123537"].map(str::to_owned);
    let mcp =
        super::mcp::call_with_globals(&env, &words, false, &env.globals(Some(&replay), false))?;
    if value != mcp {
        return Err("legacy acquisition differs between CLI and MCP".into());
    }
    let answer = value
        .pointer("/result/items/0/answer")
        .ok_or("article answer")?;
    if answer.get("resolution").and_then(Value::as_str) != Some("resolved")
        || answer.get("fetched").is_some()
    {
        return Err(format!("legacy article acquisition was invented: {value}"));
    }
    Ok(())
}
