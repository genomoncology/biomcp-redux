use super::{CSV, Check, Environment, Value, invoke, json, mcp, seed_csv};
use std::{io::Write, path::Path};
pub(super) fn check() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let header = CSV.lines().next().ok_or("missing header")?;
    let mut body = format!("{header}\n");
    for n in 0..10 {
        body.push_str(&format!(
            "SYN-{n},Trastuzumab synthetic product,Synthetic type,Synthetic area,Synthetic applicant,Synthetic form,,\n"
        ));
    }
    let replay = seed_csv(&env, body)?;
    let base = ["search", "drug", "trastuzumab", "--source", "who-pq"];
    let default = call(&env, &replay, &base, 0)?;
    candidates(&default, 10)?;
    let mut args = base.to_vec();
    args.extend(["--limit", "5"]);
    let first = call(&env, &replay, &args, 0)?;
    candidates(&first, 5)?;
    let page = first
        .get("next")
        .and_then(Value::as_array)
        .ok_or("next missing")?
        .iter()
        .find(|step| step.get("kind") == Some(&json!("page")))
        .and_then(|step| step.get("args"))
        .ok_or("five WHO candidates removed continuation")?;
    let words: Vec<String> = serde_json::from_value(page.clone()).map_err(|e| e.to_string())?;
    let tail: Vec<&str> = words.iter().map(String::as_str).collect();
    let second = call(&env, &replay, &tail, 0)?;
    candidates(&second, 5)?;
    if second.pointer("/page/offset") != Some(&json!(5))
        || second
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step.get("kind") == Some(&json!("page")))
            })
    {
        return Err("terminal WHO page changed offset or retained continuation".into());
    }
    let disabled = call(
        &env,
        &replay,
        &[
            "search",
            "drug",
            "trastuzumab",
            "--source",
            "who-pq",
            "--disable-sources",
            "who-pq",
        ],
        4,
    )?;
    if disabled.get("result").is_some()
        || disabled.get("next").is_some()
        || disabled.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err("disabled source exposed candidates or next actions".into());
    }
    Ok(())
}
fn candidates(value: &Value, count: usize) -> Check {
    let rows = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("items")?;
    let steps = value.get("next").and_then(Value::as_array).ok_or("next")?;
    let gets: Vec<_> = steps
        .iter()
        .filter(|step| step.get("kind") == Some(&json!("disambiguate")))
        .collect();
    if rows.len() != count || gets.len() != count {
        return Err(format!(
            "WHO candidate rows={}/actions={} expected={count}: {value}",
            rows.len(),
            gets.len()
        ));
    }
    for (row, step) in rows.iter().zip(gets) {
        let reference = row
            .get("reference")
            .and_then(Value::as_str)
            .ok_or("reference")?;
        if step.get("args") != Some(&json!(["get", "drug", reference, "--source", "who-pq"])) {
            return Err("WHO candidate action changed reference or source".into());
        }
    }
    Ok(())
}
fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let globals = env.globals(Some(replay), false);
    let command: Vec<String> = args.iter().map(|s| (*s).into()).collect();
    let mut words = globals.clone();
    words.extend(command.clone());
    let value = invoke(env, words, exit)?;
    if mcp::call_with_globals(env, &command, exit == 4, &globals)? != value {
        return Err("WHO navigation CLI/MCP differ".into());
    }
    let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    // Full compact output measured by j13524; each fixture ceiling adds fifteen percent.
    let baseline: usize = if exit == 4 { 355 } else { 3842 };
    if bytes > (baseline * 115).div_ceil(100) {
        return Err(format!(
            "WHO navigation exceeds measured output ceiling: {bytes}"
        ));
    }
    writeln!(
        std::io::stdout(),
        "WHO_NAV {} bytes={}",
        args.join(" "),
        serde_json::to_vec(&value).map_err(|e| e.to_string())?.len()
    )
    .map_err(|e| e.to_string())?;
    Ok(value)
}
