//! Feature cpic-pgx: synthetic native retrieval through the actual CLI and MCP.
mod rows;
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let mut out = vec![];
    for name in [
        "pairs",
        "card",
        "view",
        "table",
        "guidelines",
        "diplotype",
        "empty",
    ] {
        out.push(Trial::test(
            format!("cpic-pgx/{name}-network-cache-replay"),
            move || scenario(name).map_err(Into::into),
        ));
    }
    for name in [
        "wrong-gene",
        "wrong-drug",
        "range",
        "missing-range",
        "broken",
        "429",
        "500",
    ] {
        out.push(Trial::test(format!("cpic-pgx/reject-{name}"), move || {
            failure(name).map_err(Into::into)
        }));
    }
    out
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let cli = invoke(env, args.clone(), exit)?;
    super::schema::Registry::embedded()?.validate(&cli, &args, exit < 2)?;
    let start = args
        .iter()
        .position(|s| matches!(s.as_str(), "search" | "get"))
        .ok_or("missing command")?;
    let (globals, command) = args.split_at(start);
    let mcp = super::mcp::call_with_globals(env, command, exit >= 2, globals)?;
    if cli != mcp {
        return Err(format!("CLI/MCP differs: {cli} {mcp}"));
    }
    Ok(cli)
}
fn scenario(name: &'static str) -> Check {
    let (args, body) = rows::scenario(name);
    let returned = body.as_array().ok_or("native array")?.len();
    let total = if matches!(name, "pairs" | "view" | "table" | "guidelines") {
        2
    } else {
        returned
    };
    let range = if returned == 0 {
        "*/0".into()
    } else {
        format!("0-{}/{total}", returned - 1)
    };
    let original = body.clone();
    let stub = Stub::counted(
        move |request| {
            check_request(name, request)?;
            Ok(body.to_string())
        },
        move |_| format!("Content-Range: {range}\r\n"),
        move |_| if total > returned { 206 } else { 200 },
    )?;
    let env = environment(&stub)?;
    let record = env.root().join("record");
    let mut recording = vec!["--record".into(), record.to_string_lossy().into()];
    recording.extend(args.clone());
    let cold = invoke(&env, recording, 0)?;
    let warm = both(&env, args.clone(), 0)?;
    if cold != warm {
        return Err("warm cache lost native response".into());
    }
    let replay_env = Environment::new().map_err(|e| e.to_string())?;
    let mut replay = vec!["--replay".into(), record.to_string_lossy().into()];
    replay.extend(args.clone());
    if both(&replay_env, replay, 0)? != cold {
        return Err("recording lost native response".into());
    }
    verify_body(name, &cold, &original, total)?;
    markdown(&env, &args)?;
    for switch in ["cpic", "noncommercial"] {
        let mut blocked = vec![
            "--replay".into(),
            record.to_string_lossy().into(),
            "--disable-sources".into(),
            switch.into(),
        ];
        blocked.extend(args.clone());
        let value = both(&env, blocked, 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err("disabled source restored replay".into());
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("warm cache or disabled call opened HTTP".into());
    }
    Ok(())
}
fn check_request(name: &str, r: &str) -> Check {
    if !r.to_ascii_lowercase().contains("prefer: count=exact") {
        return Err("exact count header missing".into());
    }
    let route = match name {
        "pairs" => "pair_view",
        "card" => "gene",
        "view" => "recommendation_view",
        "table" => "recommendation",
        "guidelines" => "guideline_summary_view",
        _ => "diplotype",
    };
    if !r.starts_with(&format!("GET /v1/{route}?")) {
        return Err(format!("wrong source route: {r}"));
    }
    if name == "table" && (!r.contains("drugid=eq.RxNorm%3A10324") || !r.contains("order=id.asc")) {
        return Err("native drug filter/order missing".into());
    }
    if name == "view"
        && (!r.contains("lookupkey-%3E%3EDPYD=not.is.null")
            || !r.contains("order=recommendationid.asc"))
    {
        return Err(format!("native view selector/order missing: {r}"));
    }
    Ok(())
}
fn verify_next(name: &str, value: &Value) -> Check {
    let steps = value.get("next").and_then(Value::as_array).ok_or("next")?;
    if steps
        .iter()
        .any(|s| s.get("kind") == Some(&json!("disambiguate")))
    {
        return Err("invented pair selection".into());
    }
    if name == "pairs" && steps.first().and_then(|s| s.get("kind")) != Some(&json!("page")) {
        return Err("page must precede card/recommendations".into());
    }
    if name == "card" && steps.len() != 2 {
        return Err("gene card must offer exactly implemented sections".into());
    }
    if steps.iter().filter_map(|s| s.get("args")).any(|a| {
        a.as_array()
            .is_some_and(|v| v.contains(&json!("help")) || v.contains(&json!("diplotype")))
    }) {
        return Err("help or diplotype invented next".into());
    }
    if matches!(name, "view" | "table")
        && steps.first().and_then(|s| s.get("kind")) != Some(&json!("page"))
    {
        return Err("recommendation page missing".into());
    }
    Ok(())
}
fn failure(name: &'static str) -> Check {
    let (mut args, mut body) = rows::scenario(if name == "wrong-drug" {
        "table"
    } else {
        "pairs"
    });
    if name == "wrong-gene" {
        body[0]["genesymbol"] = json!("TPMT");
    }
    if name == "wrong-drug" {
        body[0]["drugid"] = json!("RxNorm:999");
    }
    let stub = Stub::counted(
        move |_| {
            Ok(if name == "broken" {
                "{".into()
            } else {
                body.to_string()
            })
        },
        move |_| match name {
            "missing-range" => String::new(),
            "range" => "Content-Range: 4-4/5\r\n".into(),
            _ => "Content-Range: 0-0/2\r\n".into(),
        },
        move |_| match name {
            "429" => 429,
            "500" => 500,
            _ => 206,
        },
    )?;
    let env = environment(&stub)?;
    let record = env.root().join("record");
    args.splice(0..0, ["--record".into(), record.to_string_lossy().into()]);
    let value = invoke(&env, args.clone(), 4)?;
    if value.get("result").is_some_and(|v| !v.is_null())
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/terms").is_none()
    {
        return Err(format!("source failure hidden as success: {value}"));
    }
    let mut replay = args.clone();
    replay[0] = "--replay".into();
    let replay_env = Environment::new().map_err(|e| e.to_string())?;
    let repeated = both(&replay_env, replay, 4)?;
    if repeated.get("status") != value.get("status")
        || repeated.get("sources") != value.get("sources")
    {
        return Err("recorded source failure changed".into());
    }
    let mut offline = args;
    offline.drain(0..2);
    offline.insert(0, "--offline".into());
    let unavailable = both(&env, offline, 4)?;
    if unavailable.pointer("/sources/0/reason") != Some(&json!("offline")) {
        return Err("rejected reply entered the checked cache".into());
    }
    let count = stub.finish()?.len();
    if count == 0 {
        return Err("failure never reached synthetic source".into());
    }
    Ok(())
}

fn verify_body(name: &str, cold: &Value, original: &Value, total: usize) -> Check {
    let output_rows = if name == "card" {
        json!([cold.pointer("/result/identity").ok_or("identity")?])
    } else if matches!(name, "diplotype" | "empty") {
        cold.pointer("/result/diplotypes")
            .ok_or("diplotypes")?
            .clone()
    } else {
        cold.pointer("/result/items").ok_or("items")?.clone()
    };
    if output_rows != *original {
        return Err(format!("native context changed: {output_rows}"));
    }
    if let Some(page) = cold.pointer("/result/page")
        && (page.get("total") != Some(&json!(total))
            || page.get("total_kind") != Some(&json!("source")))
    {
        return Err("native count lost".into());
    }
    if name == "table" && cold.pointer("/result/representation") != Some(&json!("table")) {
        return Err("table confused with view".into());
    }
    verify_next(name, cold)?;
    Ok(())
}
fn markdown(env: &Environment, args: &[String]) -> Check {
    let mut md = args.to_vec();
    md.extend(["--format".into(), "md".into()]);
    let text = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &md)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(text.stdout).map_err(|e| e.to_string())?;
    if !text.contains("cpic") || !text.contains("biomcp") || text.trim_start().starts_with('{') {
        return Err("Markdown omitted source or follow-up commands".into());
    }
    Ok(())
}
