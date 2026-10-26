use super::super::{configuration::invoke, launch::Environment};
use serde_json::{Value, json};

fn seed(env: &Environment) -> Result<std::path::PathBuf, String> {
    let mut capture: Value = serde_json::from_str(include_str!(
        "../../../../../tests/replies/drug/ema-synthetic/ema/1854254ba57f4597bd03970bf3664f62fac42eda059bf0022aaf32367297a317.json"
    ))
    .map_err(|e| e.to_string())?;
    let mut body: Value = serde_json::from_str(
        capture
            .pointer("/exchanges/0/body/value")
            .and_then(Value::as_str)
            .ok_or("missing body")?,
    )
    .map_err(|e| e.to_string())?;
    for row in body
        .get_mut("data")
        .and_then(Value::as_array_mut)
        .ok_or("missing rows")?
    {
        super::replace(row, "/name_of_medicine", json!("Synthetic medicine"))?;
    }
    super::replace(
        &mut capture,
        "/exchanges/0/body/value",
        json!(body.to_string()),
    )?;
    let replay = env.root().join("replay");
    let source = replay.join("ema");
    std::fs::create_dir_all(&source).map_err(|e| e.to_string())?;
    std::fs::write(
        source.join("1854254ba57f4597bd03970bf3664f62fac42eda059bf0022aaf32367297a317.json"),
        capture.to_string(),
    )
    .map_err(|e| e.to_string())?;
    let mut sync = env.globals(Some(&replay), false);
    sync.extend(["sync", "ema"].map(str::to_owned));
    invoke(env, sync, 0)?;
    Ok(replay)
}

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    let args: Vec<_> = [
        "search",
        "drug",
        "Synthetic",
        "--source",
        "ema",
        "--limit",
        "5",
    ]
    .map(str::to_owned)
    .into();
    let mut cli = env.globals(Some(&replay), false);
    cli.extend(args.clone());
    let value = invoke(&env, cli, 0)?;
    let globals: Vec<_> = env.globals(Some(&replay), false);
    if super::super::mcp::call_with_globals(&env, &args, false, &globals)? != value {
        return Err("EMA candidate navigation differs over MCP".into());
    }
    super::super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    if value.get("page") != Some(&json!({"offset":0,"returned":5,"total":6})) {
        return Err(format!("wrong EMA page: {value}"));
    }
    let items = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("missing candidates")?;
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing navigation")?;
    if next.len() != 6 {
        return Err(format!("wrong navigation count: {next:?}"));
    }
    for (row, action) in items.iter().zip(next) {
        let id = row.get("id").ok_or("missing candidate id")?;
        if action != &json!({"kind":"disambiguate","args":["get","drug",id,"--source","ema"]}) {
            return Err(format!("candidate get lost or changed: {action}"));
        }
    }
    if next.get(5)
        != Some(
            &json!({"kind":"page","args":["search","drug","Synthetic","--limit","5","--offset","5","--source","ema"]}),
        )
    {
        return Err(format!("EMA page continuation lost: {next:?}"));
    }
    let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    let comparable = super::super::source_terms::size(&value, "")?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "EMA navigation full={full}, comparable={comparable}"
    )
    .map_err(|e| e.to_string())?;
    if full > 2096 || comparable > 1898 || comparable.div_ceil(4) > 475 {
        return Err(format!(
            "EMA navigation exceeds measured budget: full={full}, comparable={comparable}"
        ));
    }
    Ok(())
}
