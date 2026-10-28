//! Native organization continuations retain a complete observation without another request.
use super::super::{launch::Environment, nci::invoke, response_cases::copy_capture};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("organization/native-local-pages", || pages().map_err(Into::into)),
        Trial::test("organization/bounded-failures", || failures().map_err(Into::into)),
    ]
}
fn environment() -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"keys":{"nci_api_key":"synthetic-nci-stored-key"}})).map_err(|e|e.to_string())?;
    Ok(env)
}
fn query() -> Vec<String> {
    ["search","organization","--source","nci","--name","Dana-Farber","--limit","2"].map(str::to_owned).to_vec()
}
fn pages() -> Check {
    let env = environment()?;
    let replay = super::super::launch::fixtures().join("replies/organization/native");
    let mut args = query();
    args.extend(env.globals(Some(&replay),false));
    let mut answer = invoke(&env,&args,0)?;
    let mut names = Vec::new();
    let mut counts = Vec::new();
    for _ in 0..6 {
        if answer.pointer("/page/total") != Some(&json!(11)) { return Err("organization total changed while paging".into()); }
        let rows = answer.pointer("/result/items").and_then(Value::as_array).ok_or("missing organization rows")?;
        for row in rows {
            names.push(row.get("name").cloned().ok_or("missing name")?);
            counts.push(row.get("count").cloned().ok_or("missing native trial count")?);
            if row.get("id").is_some() { return Err("organization acquired an invented identity".into()); }
        }
        let next = answer.get("next").and_then(Value::as_array).into_iter().flatten().find(|s| s.get("kind")==Some(&json!("page")));
        let Some(next) = next else { break; };
        let mut args: Vec<String> = serde_json::from_value(next.get("args").cloned().ok_or("page args")?).map_err(|e|e.to_string())?;
        if !args.iter().any(|s| s=="--pool") { return Err("page lost snapshot identity".into()); }
        args.extend(env.globals(None,true));
        answer = invoke(&env,&args,0)?;
    }
    if names.len()!=11 || counts.get(3)!=Some(&json!(3376)) || names.get(3)!=Some(&json!("Dana-Farber Cancer Institute")) {
        return Err("local pages skipped, repeated or changed source rows".into());
    }
    Ok(())
}
fn failures() -> Check {
    for kind in ["overflow","bucket","bad-value","cut-collection"] {
        let env = environment()?;
        let replay = env.root().join("reply");
        copy_capture("organization/native", &replay, |_,record| mutate(record,kind)).map_err(|e|e.to_string())?;
        let mut args = query();
        args.extend(env.globals(Some(&replay),false));
        let answer = invoke(&env,&args,if kind=="overflow" {2} else if kind=="bad-value" {0} else {4})?;
        match kind {
            "overflow" => {
                if answer.pointer("/error/code") != Some(&json!("usage.too_broad")) || !answer.pointer("/error/message").and_then(Value::as_str).is_some_and(|s|s.contains("--city") && s.contains("--family")) {
                    return Err("overflow lost typed narrowing advice".into());
                }
            }
            "bad-value" => {
                if answer.pointer("/page/total") != Some(&json!(11)) || answer.pointer("/result/items/0/count")!=Some(&json!(7)) || !answer.get("problems").and_then(Value::as_array).is_some_and(|p|p.iter().any(|p|p.get("field")==Some(&json!("org_coordinates")))) {
                    return Err("bad optional value removed organizations or hid its problem".into());
                }
            }
            _ => {
                if answer.get("page").is_some() || answer.pointer("/sources/0/state")!=Some(&json!("failed")) {
                    return Err("source failure became zero organizations".into());
                }
            }
        }
    }
    Ok(())
}
fn mutate(record: &mut Value, kind: &str) {
    if let Some(exchange) = record.pointer_mut("/exchanges/0") {
        match kind {
            "bucket" => {
                if let Some(body) = exchange.get_mut("body") { *body = json!({"kind":"json","value":{"error":{"type":"too_many_buckets_exception","reason":"synthetic bucket limit"}}}); }
            }
            "overflow" => {
                if let Some(body) = exchange.get_mut("body") { *body = json!({"kind":"text","value":"x".repeat(1024*1024+1)}); }
            }
            "bad-value" => {
                if let Some(value) = exchange.pointer_mut("/body/value/data/0/org_coordinates") { *value=json!({"lat":999,"lon":0}); }
            }
            _ => { if let Some(value) = exchange.pointer_mut("/body/value/total") { *value=json!(999); } }
        }
    }
}
