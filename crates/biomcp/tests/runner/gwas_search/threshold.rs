//! Exact threshold behavior through the command and its emitted continuation.
use super::{Environment, Path, PathBuf, Value, body, call, fs, json, pairs, seed, seed_text, url};
fn start(threshold: &str) -> Vec<&str> {
    vec![
        "search",
        "gwas",
        "--pubmed",
        "17529967",
        "--p-value",
        threshold,
    ]
}
fn follow(env: &Environment, replay: &Path, value: &Value, exit: i32) -> Result<Value, String> {
    let args = value
        .pointer("/next/0/args")
        .and_then(Value::as_array)
        .ok_or("continuation")?
        .iter()
        .map(|arg| arg.as_str().ok_or("arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let cli = call("gwas_search/threshold/follow/1", env, replay, &args, exit)?;
    if exit == 0 {
        let owned = args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        let mcp = super::super::mcp::call_with_globals(
            env,
            &owned,
            false,
            &env.globals(Some(replay), false),
        )?;
        if cli != mcp {
            return Err("threshold continuation CLI/MCP mismatch".into());
        }
    }
    Ok(cli)
}
fn page(env: &Environment, number: u32, total: u32, exponent: i64) -> Result<PathBuf, String> {
    let mut p = pairs();
    let n = number.to_string();
    p.retain(|(key, _)| *key != "page");
    p.push(("page", &n));
    let u = url(&p);
    let mut value = body(&u, number, total);
    for row in value
        .pointer_mut("/_embedded/associations")
        .and_then(Value::as_array_mut)
        .ok_or("rows")?
    {
        *row.get_mut("pvalue_exponent").ok_or("exponent")? = json!(exponent);
        *row.get_mut("p_value").ok_or("pvalue")? = json!(0);
    }
    let offset = (number * 3).to_string();
    let args = [
        "search", "gwas", "--pubmed", "17529967", "--offset", &offset,
    ];
    seed(env, &args, value, 200, 0)
}
pub(super) fn walk() -> Result<(), String> {
    for (threshold, exponent, count, fresh) in [
        ("4e-254", -254, 6, false),
        ("4e-254", -254, 6, true),
        ("4e-400", -400, 6, false),
        ("3e-254", -254, 0, false),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = page(&env, 0, 6, exponent)?;
        page(&env, 1, 6, exponent)?;
        let mut args = start(threshold);
        if fresh {
            args.push("--fresh");
        }
        let first = call("gwas_search/threshold/walk/1", &env, &replay, &args, 0)?;
        if count == 0 {
            final_count(&first, 0)?;
            continue;
        }
        if first
            .pointer("/result/items")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(3)
            || first.pointer("/result/qualifying_total") != Some(&Value::Null)
            || first.pointer("/next/0/kind") != Some(&json!("page"))
        {
            return Err(format!(
                "inclusive first page must keep unknown qualifying total: {first}"
            ));
        }
        let second = follow(&env, &replay, &first, 0)?;
        final_count(&second, count)?;
        if second.pointer("/result/items/0/id") != Some(&json!(6113)) {
            return Err("second page identity".into());
        }
    }
    crossing()
}
fn final_count(value: &Value, count: u64) -> Result<(), String> {
    if value.pointer("/result/qualifying_total") != Some(&json!(count))
        || value.pointer("/result/continuation").is_some()
        || value
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps
                    .iter()
                    .any(|step| step.get("kind") == Some(&json!("page")))
            })
    {
        return Err(format!("wrong completed threshold total: {value}"));
    }
    Ok(())
}
fn crossing() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = page(&env, 0, 6, -400)?;
    page(&env, 1, 6, -399)?;
    let first = call(
        "gwas_search/threshold/crossing/1",
        &env,
        &replay,
        &start("4e-400"),
        0,
    )?;
    let second = follow(&env, &replay, &first, 0)?;
    final_count(&second, 3)?;
    if second.pointer("/result/items").is_some() || second.pointer("/page/total") != Some(&json!(6))
    {
        return Err("source total or empty threshold boundary lost".into());
    }
    Ok(())
}
pub(super) fn failures() -> Result<(), String> {
    for fault in [
        "429",
        "500",
        "timeout",
        "broken",
        "duplicate",
        "order",
        "page",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = page(&env, 0, 6, -400)?;
        let mut p = pairs();
        p.retain(|(key, _)| *key != "page");
        p.push(("page", "1"));
        let u = url(&p);
        let mut response = body(&u, 1, 6);
        if fault == "duplicate" {
            *response
                .pointer_mut("/_embedded/associations/0/association_id")
                .ok_or("id")? = json!(6110);
        }
        if fault == "order" {
            for row in response
                .pointer_mut("/_embedded/associations")
                .and_then(Value::as_array_mut)
                .ok_or("rows")?
            {
                *row.get_mut("pvalue_exponent").ok_or("exponent")? = json!(-401);
            }
        }
        if fault == "page" {
            *response.pointer_mut("/page/number").ok_or("page")? = json!(0);
        }
        seed_text(
            &env,
            &["search", "gwas", "--pubmed", "17529967", "--offset", "3"],
            if fault == "broken" {
                "{".into()
            } else {
                response.to_string()
            },
            fault.parse().unwrap_or(200),
            if fault == "timeout" { 16000 } else { 0 },
        )?;
        let first = call(
            "gwas_search/threshold/failures/1",
            &env,
            &replay,
            &start("1e-200"),
            0,
        )?;
        let second = follow(&env, &replay, &first, 4)?;
        if second.get("result").is_some()
            || second.get("page").is_some()
            || second.pointer("/sources/0/use") != Some(&json!("restricted"))
            || second.pointer("/sources/0/terms").is_none()
            || second.get("status") != Some(&json!("failed"))
        {
            return Err(format!("later {fault} became a threshold answer: {second}"));
        }
        if ["duplicate", "order", "page"].contains(&fault)
            && second.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
        {
            return Err(format!("wrong {fault} reason: {second}"));
        }
    }
    Ok(())
}
pub(super) fn native(directory: &Path) -> Result<(), String> {
    use std::io::Write;
    let registry = super::super::schema::Registry::embedded()?;
    let mut paths = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|entry| entry.map(|entry| entry.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    if paths.len() != 10 {
        return Err("expected ten threshold native cases".into());
    }
    let outputs = directory
        .parent()
        .ok_or("fixture root")?
        .join("gwas-native-outputs");
    fs::create_dir(&outputs).map_err(|e| e.to_string())?;
    let budgets: std::collections::BTreeMap<String, [usize; 2]> =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/gwas-threshold-budgets.json"
        )))
        .map_err(|e| e.to_string())?;
    for path in paths {
        let value = super::super::run_case(&path, &registry)?;
        let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
        let comparable = super::super::source_terms::size(&value, "")?;
        let name = path
            .file_stem()
            .and_then(|v| v.to_str())
            .ok_or("case name")?;
        let [full, comparable_limit] = budgets.get(name).ok_or("case budget")?;
        if bytes.len() + 1 > (full * 115).div_ceil(100)
            || comparable > (comparable_limit * 115).div_ceil(100)
        {
            return Err(format!(
                "{name} exceeded its measured fifteen-percent output ceiling"
            ));
        }
        fs::write(outputs.join(format!("{name}.json")), &bytes).map_err(|e| e.to_string())?;
        writeln!(
            std::io::stdout(),
            "GWAS threshold {name}: CLI={}, JSON={}, comparable={}, estimated_tokens={}",
            bytes.len() + 1,
            bytes.len(),
            comparable,
            comparable.div_ceil(4)
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
