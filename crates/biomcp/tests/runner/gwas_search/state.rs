//! Technical controls retain source semantics rather than inventing clinical negatives.
use super::{Environment, Value, body, call, fs, gwas_budgets, json, pairs, seed, url};
pub(super) fn stale() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["search", "gwas", "--pubmed", "17529967"];
    let replay = seed(&env, &args, body(&url(&pairs()), 0, 6), 200, 0)?;
    call("gwas_search/state/stale/1", &env, &replay, &args, 0)?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE reply SET fresh_until=0", [])
        .map_err(|e| e.to_string())?;
    drop(db);
    let mut offline = env.globals(None, true);
    offline.extend(args.map(str::to_owned));
    let value = gwas_budgets::invoke("gwas_search/state/stale/2", &env, offline, 0)?;
    if value.get("status") != Some(&json!("partial"))
        || value.get("result").is_none()
        || value.pointer("/sources/0/state") != Some(&json!("stale"))
        || super::super::action(&value, "fresh")?.is_empty()
    {
        return Err(format!("stale search became complete or empty: {value}"));
    }
    bounded(&value)
}
pub(super) fn expired() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = [
        "search",
        "gwas",
        "--pubmed",
        "17529967",
        "--p-value",
        "1e-200",
    ];
    let replay = seed(&env, &args, body(&url(&pairs()), 0, 6), 200, 0)?;
    let first = call("gwas_search/state/expired/1", &env, &replay, &args, 0)?;
    let page = super::super::action(&first, "page")?;
    fs::remove_dir_all(replay.join("gwas")).map_err(|e| e.to_string())?;
    for source in ["gwas", "restricted"] {
        let mut command = env.globals(Some(&replay), false);
        command.extend(page.clone());
        command.extend(["--disable-sources".into(), source.into()]);
        let value = gwas_budgets::invoke("gwas_search/state/expired/2", &env, command, 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            || value.get("result").is_some()
        {
            return Err(format!("disabled source restored threshold pool: {value}"));
        }
        bounded(&value)?;
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE pool SET expires=0", [])
        .map_err(|e| e.to_string())?;
    drop(db);
    let mut command = env.globals(Some(&replay), false);
    command.extend(page);
    let value = gwas_budgets::invoke("gwas_search/state/expired/3", &env, command, 3)?;
    let restart = super::super::action(&value, "restart")?;
    if value.pointer("/error/code") != Some(&json!("pool.expired"))
        || !args.iter().all(|arg| restart.contains(&(*arg).to_owned()))
        || restart.contains(&"--pool".to_owned())
    {
        return Err(format!("expired pool lost selectors or refetched: {value}"));
    }
    bounded(&value)
}
pub(super) fn literal() -> Result<(), String> {
    let stub = super::super::network_stub::Stub::for_source(
        "www.ebi.ac.uk",
        "application/json",
        |request| {
            let path = request.split_whitespace().nth(1).ok_or("request target")?;
            Ok(body(&format!("https://www.ebi.ac.uk{path}"), 0, 0).to_string())
        },
    )?;
    let env = super::super::variant::source_environment(&stub)?;
    let value = gwas_budgets::invoke(
        "gwas_search/state/literal/1",
        &env,
        ["search", "gwas", "--trait-id", "EFO_0000305"]
            .map(str::to_owned)
            .into(),
        0,
    )?;
    if value.pointer("/page/total") != Some(&json!(0)) {
        return Err("native zero was not preserved".into());
    }
    let requests = stub.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err("expected one exact trait request".into());
    };
    if !request.contains("efo_id=EFO_0000305") || request.contains("MONDO") {
        return Err("native trait ID was repaired".into());
    }
    bounded(&value)
}
fn bounded(value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    if bytes > crate::runner::output_limit("gwas-state-bytes")?
        || bytes.div_ceil(4) > crate::runner::output_limit("gwas-state-tokens")?
    {
        return Err("synthetic state exceeds ceiling".into());
    }
    Ok(())
}
