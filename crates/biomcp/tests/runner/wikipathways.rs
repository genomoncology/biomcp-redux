//! Feature pathway-wikipathways: explicit atomic catalog replacement and stored-only reads.
use super::launch::Environment;
use crate::runner::pathway_budgets as budgets;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};

type Check = Result<(), String>;
const SEARCH: &str = "https://www.wikipathways.org/json/findPathwaysByText.json";
const GET: &str = "https://www.wikipathways.org/json/getPathwayInfo.json";

pub(super) fn catalog(search: bool, changed: bool) -> Value {
    json!(
        {"pathwayInfo":(0..13).map(|n| { let mut row = json!({"id":format!("WP{}",n+1),
        "url":format!("https://www.wikipathways.org/instance/WP{}",n+1),
        "name":if changed {format!("New Atlas {n}")} else {format!("Literal Atlas {n}")},
        "species":if n==12 {"Mus musculus"} else {"Homo sapiens"},"revision":"2025-05-22","authors":"Source Author",
        "description":"Native source description...",
        "citedIn":""}); if search && let Some(map)=row.as_object_mut() { map.insert("datanodes".into(),
        json!("TP53, unchecked label")); map.insert("annotations".into(),json!("native annotation")); } row }).collect::<Vec<_>>()}
    )
}
fn recording(root: &Path, url: &str) -> PathBuf {
    use sha2::{Digest, Sha256};
    let key = Sha256::digest(format!("GET\0{url}").as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    root.join("wikipathways").join(format!("{key}.json"))
}
pub(super) fn seed(
    env: &Environment,
    search: Value,
    get: Value,
    status: u16,
) -> Result<PathBuf, String> {
    let root = env.root().join("replay");
    let dir = root.join("wikipathways");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    for (url, body, code) in [(SEARCH, search, 200), (GET, get, status)] {
        let exchange = json!(
            {"status":code,"content_type":"application/json","delay_ms":0,"retry_after_ms":null,"body":{"kind":"text","value":body.to_string()}}
        );
        let exchanges = vec![exchange; if code == 500 { 3 } else { 1 }];
        let wire = json!({"source":"wikipathways","method":"GET","url":url,"exchanges":exchanges});
        fs::write(recording(&root, url), wire.to_string()).map_err(|e| e.to_string())?;
    }
    Ok(root)
}
fn call(
    phase: &str,
    env: &Environment,
    replay: &Path,
    args: &[&str],
    exit: i32,
) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    budgets::invoke(phase, env, command, exit)
}
fn run() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, catalog(true, false), catalog(false, false), 200)?;
    let receipt = call(
        "wikipathways/run/1",
        &env,
        &replay,
        &["sync", "wikipathways"],
        0,
    )?;
    if receipt.pointer("/result/search/total_rows") != Some(&json!(13))
        || receipt.pointer("/result/get/human_rows") != Some(&json!(12))
    {
        return Err(format!("receipt: {receipt}"));
    }
    fs::remove_dir_all(replay.join("wikipathways")).map_err(|e| e.to_string())?;
    let first = call(
        "wikipathways/run/2",
        &env,
        &replay,
        &["search", "pathway", "atlas", "--source", "wikipathways"],
        0,
    )?;
    if first.pointer("/result/page/total") != Some(&json!(12))
        || first.pointer("/result/items/0/id/id") != Some(&json!("WP1"))
    {
        return Err(format!("title page: {first}"));
    }
    let pool = first
        .pointer("/next/5/args")
        .and_then(Value::as_array)
        .ok_or("page action")?
        .iter()
        .map(|v| v.as_str().ok_or("arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let second = call("wikipathways/run/3", &env, &replay, &pool, 0)?;
    if second.pointer("/result/items/0/id/id") != Some(&json!("WP6")) {
        return Err("page order".into());
    }
    let card = call(
        "wikipathways/run/4",
        &env,
        &replay,
        &["get", "pathway", "WP1", "summary"],
        0,
    )?;
    if card.pointer("/result/description_truncated") != Some(&json!(true))
        || card.pointer("/result/pathway/description")
            != Some(&json!("Native source description..."))
    {
        return Err(format!("source summary: {card}"));
    }
    call(
        "wikipathways/run/5",
        &env,
        &replay,
        &["get", "pathway", "WP13"],
        1,
    )?;
    call(
        "wikipathways/run/6",
        &env,
        &replay,
        &["get", "pathway", "WP99999"],
        1,
    )?;
    call(
        "wikipathways/run/7",
        &env,
        &replay,
        &["get", "pathway", "WP1", "events"],
        2,
    )?;
    replacement(&env, &replay, &pool)
}
fn replacement(env: &Environment, replay: &Path, pool: &[&str]) -> Check {
    for fault in [json!({"unexpected":true}), json!({"pathwayInfo":[]})] {
        let label = fault.to_string();
        seed(env, catalog(true, true), fault, 200)?;
        let failed = call(
            "wikipathways/replacement/1",
            env,
            replay,
            &["sync", "wikipathways"],
            4,
        )
        .map_err(|e| format!("catalog fault {label}: {e}"))?;
        if failed.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
            || failed.get("local_error").is_some()
        {
            return Err(format!("validation classification: {failed}"));
        }
        let retained = call(
            "wikipathways/replacement/2",
            env,
            replay,
            &[
                "search",
                "pathway",
                "Literal Atlas",
                "--source",
                "wikipathways",
            ],
            0,
        )?;
        if retained.pointer("/result/page/total") != Some(&json!(12)) {
            return Err("failed pair replaced old snapshot".into());
        }
    }
    partial_download(env, replay)?;
    seed(env, catalog(true, true), catalog(false, true), 500)?;
    call(
        "wikipathways/replacement/3",
        env,
        replay,
        &["sync", "wikipathways"],
        4,
    )?;
    seed(env, catalog(true, true), catalog(false, true), 200)?;
    call(
        "wikipathways/replacement/4",
        env,
        replay,
        &["sync", "wikipathways"],
        0,
    )?;
    let retained = call("wikipathways/replacement/5", env, replay, pool, 0)?;
    if retained.pointer("/result/items/0/name") != Some(&json!("Literal Atlas 5")) {
        return Err("pool changed with catalog replacement".into());
    }
    let disabled = call(
        "wikipathways/replacement/6",
        env,
        replay,
        &["get", "pathway", "WP1", "--disable-sources", "wikipathways"],
        4,
    )?;
    if disabled.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err("disabled read used stored rows".into());
    }
    Ok(())
}

fn partial_download(env: &Environment, replay: &Path) -> Check {
    seed(env, catalog(true, true), catalog(false, true), 200)?;
    let path = recording(replay, GET);
    let mut value: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    *value.pointer_mut("/exchanges/0/body/value").ok_or("body")? = json!("{\"pathwayInfo\":[");
    fs::write(path, value.to_string()).map_err(|e| e.to_string())?;
    let failed = call(
        "wikipathways/partial_download/1",
        env,
        replay,
        &["sync", "wikipathways"],
        4,
    )?;
    if failed.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
        || failed.get("local_error").is_some()
    {
        return Err(format!("partial download: {failed}"));
    }
    let retained = call(
        "wikipathways/partial_download/2",
        env,
        replay,
        &[
            "search",
            "pathway",
            "Literal Atlas",
            "--source",
            "wikipathways",
        ],
        0,
    )?;
    if retained.pointer("/result/page/total") != Some(&json!(12)) {
        return Err("partial download replaced accepted catalogs".into());
    }
    Ok(())
}

tool_only! {}
