//! Research wrappers map the existing CLI result without provider calls.
use super::{launch::Environment, mcp::{exchange, request}, trial_get::command};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("mcp/research-search-fetch-parity", || parity().map_err(Into::into))]
}
fn call(env: &Environment, name: &str, arguments: Value, globals: &[String]) -> Result<Value, String> {
    let replies = super::mcp::exchange_with_globals(env, &[request(json!(1), "tools/call", json!({"name":name,"arguments":arguments}))], globals)?;
    let result = replies.first().and_then(|v| v.get("result")).ok_or("missing tool result")?;
    let value = result.get("structuredContent").ok_or("missing structured content")?;
    let text = result.pointer("/content/0/text").and_then(Value::as_str).ok_or("missing text")?;
    if serde_json::from_str::<Value>(text).map_err(|e| e.to_string())? != *value {
        return Err("research text differs from structured content".into());
    }
    Ok(value.clone())
}
fn parity() -> Result<(), String> {
    for (query, args, capture) in [
        ("AKT1 E17K", vec!["search", "article", "AKT1 E17K"], "article/search/synthetic-card-step"),
        ("trial --condition melanoma", vec!["search", "trial", "--condition", "melanoma"], "trial/condition"),
        ("variant --gene EGFR --protein L858R --build GRCh37", vec!["search", "variant", "--gene", "EGFR", "--protein", "L858R", "--build", "GRCh37"], "variant/search/L858R"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let globals = env.globals(Some(&super::launch::fixtures().join("replies").join(capture)), false);
        let args = args.into_iter().map(str::to_owned).collect::<Vec<_>>();
        let cli = command(&env, &args, &globals, 0)?;
        let wrapped = call(&env, "search", json!({"query":query}), &globals)?;
        if wrapped.get("metadata") != Some(&cli) {
            return Err(format!("search metadata lost CLI fields: {wrapped}"));
        }
        let rows = cli.pointer("/result/items").or_else(|| cli.pointer("/result/candidates")).and_then(Value::as_array).ok_or("CLI rows missing")?;
        let hits = wrapped.get("results").and_then(Value::as_array).ok_or("search results missing")?;
        if rows.len() != hits.len() || hits.is_empty() { return Err("research dropped source rows".into()); }
        let hit = hits.first().ok_or("first hit missing")?;
        let id = hit.get("id").and_then(Value::as_str).ok_or("hit id missing")?;
        if !id.starts_with(args.get(1).ok_or("entity missing")?) || hit.get("url").and_then(Value::as_str).is_none_or(str::is_empty) { return Err("uncitable result".into()); }
        // Search and get captures are retained separately; exact ids come from the wrapper.
        let get_capture = match args.get(1).map(String::as_str) {
            Some("article") => "article/search/synthetic-card-step",
            Some("trial") => "trial/get/NCT04631731",
            _ => "variant/search/L858R",
        };
        if super::launch::fixtures().join("replies").join(get_capture).exists() {
            let fetch_globals = env.globals(Some(&super::launch::fixtures().join("replies").join(get_capture)), false);
            let mut get = vec!["get".to_owned()];
            get.extend(id.split_whitespace().map(str::to_owned));
            let cli_get = command(&env, &get, &fetch_globals, 0)?;
            let fetched = call(&env, "fetch", json!({"id":id}), &fetch_globals)?;
            if fetched.get("metadata") != Some(&cli_get) || fetched.get("id") != Some(&json!(id)) { return Err("fetch differs from CLI".into()); }
            let mut md = get;
            md.extend(["--format".to_owned(), "md".to_owned()]);
            md.extend(fetch_globals);
            let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &md).output().map_err(|e| e.to_string())?;
            let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            if fetched.get("text") != Some(&json!(text)) { return Err("fetch Markdown differs".into()); }
        }
    }
    invalid()
}
fn invalid() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for (id, args) in [("", vec!["get"]), ("article nonsense", vec!["get", "article", "nonsense"]), ("gene", vec!["get", "gene"])] {
        let cli = command(&env, &args.into_iter().map(str::to_owned).collect::<Vec<_>>(), &[], 2)?;
        if call(&env, "fetch", json!({"id":id}), &[])? != cli { return Err("fetch usage differs from CLI".into()); }
    }
    let replies = exchange(&env, &[request(json!(1), "tools/call", json!({"name":"search", "arguments":{"query":3}}))])?;
    if replies.first().and_then(|r| r.pointer("/error/code")) != Some(&json!(-32602)) { return Err("invalid input accepted".into()); }
    Ok(())
}
