//! Exact threshold behavior through the command and its emitted continuation.
use super::*;
fn start<'a>(threshold: &'a str) -> Vec<&'a str> {
    vec!["search", "gwas", "--pubmed", "17529967", "--p-value", threshold]
}
fn follow(env: &Environment, replay: &Path, value: &Value, exit: i32) -> Result<Value, String> {
    let args = value.pointer("/next/0/args").and_then(Value::as_array).ok_or("continuation")?
        .iter().map(|arg| arg.as_str().ok_or("arg")).collect::<Result<Vec<_>,_>>()?;
    call(env, replay, &args, exit)
}
fn page(env: &Environment, number: u32, total: u32, exponent: i64) -> Result<PathBuf, String> {
    let mut p = pairs();
    let n = number.to_string();
    p.retain(|(key,_)| *key != "page"); p.push(("page", &n));
    let u = url(&p);
    let mut value = body(&u, number, total);
    for row in value.pointer_mut("/_embedded/associations").and_then(Value::as_array_mut).ok_or("rows")? {
        *row.get_mut("pvalue_exponent").ok_or("exponent")? = json!(exponent);
        *row.get_mut("p_value").ok_or("pvalue")? = json!(0);
    }
    seed(env, &u, value, 200, 0)
}
pub(super) fn walk() -> Result<(), String> {
    for (threshold, exponent, count) in [("4e-254", -254, 6), ("4e-400", -400, 6), ("3e-254", -254, 0)] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = page(&env, 0, 6, exponent)?;
        page(&env, 1, 6, exponent)?;
        let first = call(&env, &replay, &start(threshold), 0)?;
        if count == 0 { final_count(&first, 0)?; continue; }
        if first.pointer("/result/items").and_then(Value::as_array).map(Vec::len) != Some(3)
            || first.pointer("/result/qualifying_total") != Some(&Value::Null)
            || first.pointer("/next/0/kind") != Some(&json!("page")) {
            return Err(format!("inclusive first page must keep unknown qualifying total: {first}"));
        }
        let second = follow(&env, &replay, &first, 0)?;
        final_count(&second, count)?;
        if second.pointer("/result/items/0/id") != Some(&json!(6113)) { return Err("second page identity".into()); }
    }
    crossing()
}
fn final_count(value: &Value, count: u64) -> Result<(), String> {
    if value.pointer("/result/qualifying_total") != Some(&json!(count))
        || value.pointer("/result/continuation").is_some()
        || value.get("next").and_then(Value::as_array).is_some_and(|steps| steps.iter().any(|step| step.get("kind") == Some(&json!("page")))) {
        return Err(format!("wrong completed threshold total: {value}"));
    }
    Ok(())
}
fn crossing() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = page(&env, 0, 6, -400)?;
    page(&env, 1, 6, -399)?;
    let first = call(&env, &replay, &start("4e-400"), 0)?;
    let second = follow(&env, &replay, &first, 0)?;
    final_count(&second, 3)?;
    if second.pointer("/result/items") != Some(&json!([])) || second.pointer("/page/total") != Some(&json!(6)) {
        return Err("source total or empty threshold boundary lost".into());
    }
    Ok(())
}
pub(super) fn failures() -> Result<(), String> {
    for fault in ["429", "500", "timeout", "broken", "duplicate", "order", "page"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = page(&env, 0, 6, -400)?;
        let mut p = pairs(); p.retain(|(key,_)| *key != "page"); p.push(("page", "1"));
        let u = url(&p);
        let mut response = body(&u, 1, 6);
        if fault == "duplicate" { *response.pointer_mut("/_embedded/associations/0/association_id").ok_or("id")? = json!(6110); }
        if fault == "order" {
            for row in response.pointer_mut("/_embedded/associations").and_then(Value::as_array_mut).ok_or("rows")? {
                *row.get_mut("pvalue_exponent").ok_or("exponent")? = json!(-401);
            }
        }
        if fault == "page" { *response.pointer_mut("/page/number").ok_or("page")? = json!(0); }
        seed_text(&env, &u, if fault == "broken" { "{".into() } else { response.to_string() }, fault.parse().unwrap_or(200), if fault == "timeout" { 16000 } else { 0 })?;
        let first = call(&env, &replay, &start("1e-200"), 0)?;
        let second = follow(&env, &replay, &first, 4)?;
        if second.get("result").is_some() || second.get("page").is_some()
            || second.pointer("/sources/0/use") != Some(&json!("restricted"))
            || second.pointer("/sources/0/terms").is_none()
            || second.get("status") != Some(&json!("failed")) {
            return Err(format!("later {fault} became a threshold answer: {second}"));
        }
        if ["duplicate", "order", "page"].contains(&fault)
            && second.pointer("/sources/0/reason") != Some(&json!("bad_reply")) { return Err(format!("wrong {fault} reason: {second}")); }
    }
    Ok(())
}
