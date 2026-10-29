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
mod concurrent;
mod native;
mod optional_metadata;
type Check = Result<(), String>;
const SEARCH: &str = "https://www.wikipathways.org/json/findPathwaysByText.json";
const GET: &str = "https://www.wikipathways.org/json/getPathwayInfo.json";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("wikipathways/optional-name-keeps-native-identity", || {
            optional_metadata::check().map_err(Into::into)
        }),
        transcript_trial(
            "wikipathways/exact-source-species-before-text-normalization",
            || exact_species().map_err(Into::into),
        ),
        transcript_trial("wikipathways/stale-and-expired-stored-reads", || {
            read_policy().map_err(Into::into)
        }),
        transcript_trial("wikipathways/stored-identity-pages-and-replacement", || {
            run().map_err(Into::into)
        }),
        transcript_trial(
            "wikipathways/independent-union-legs-and-round-robin",
            || union().map_err(Into::into),
        ),
        transcript_trial(
            "wikipathways/concurrent-readers-during-pair-replacement",
            || concurrent::run().map_err(Into::into),
        ),
        transcript_trial("wikipathways/native-disclosure-measurements", || {
            native::run().map_err(Into::into)
        })
        .with_ignored_flag(
            !super::launch::fixtures()
                .join("cases/wikipathways-native")
                .is_dir(),
        ),
    ]
}
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
fn union() -> Check {
    use super::launch::fixtures;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, catalog(true, false), catalog(false, false), 200)?;
    call(
        "wikipathways/union/1",
        &env,
        &replay,
        &["sync", "wikipathways"],
        0,
    )?;
    let source = fixtures().join("replies/pathway/native/reactome");
    fs::create_dir_all(replay.join("reactome")).map_err(|e| e.to_string())?;
    for path in fs::read_dir(source).map_err(|e| e.to_string())? {
        let path = path.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|s| s == "json") {
            fs::copy(
                &path,
                replay
                    .join("reactome")
                    .join(path.file_name().ok_or("filename")?),
            )
            .map_err(|e| e.to_string())?;
        }
    }
    let args = [
        "search",
        "pathway",
        "MAPK signaling",
        "--source",
        "reactome",
        "--source",
        "wikipathways",
    ];
    // The synthetic catalog has no title match for this phrase; the leg still has an outcome.
    let mut missing = args.to_vec();
    missing.extend(["--offset", "5"]);
    let restarted = call("wikipathways/union/2", &env, &replay, &missing, 3)?;
    if restarted.pointer("/next/0/kind") != Some(&json!("restart")) {
        return Err(format!("missing mixed pool: {restarted}"));
    }
    let page = call("wikipathways/union/3", &env, &replay, &args, 0)?;
    if page.pointer("/sources/0/source") != Some(&json!("reactome"))
        || page.pointer("/sources/1/source") != Some(&json!("wikipathways"))
    {
        return Err(format!("independent legs: {page}"));
    }
    matching_union(&env, &replay, &args)
}
fn matching_union(env: &Environment, replay: &Path, args: &[&str]) -> Check {
    let mut body = catalog(true, false);
    for row in body
        .get_mut("pathwayInfo")
        .and_then(Value::as_array_mut)
        .ok_or("rows")?
    {
        *row.get_mut("name").ok_or("name")? = json!("MAPK signaling");
    }
    seed(env, body, catalog(false, false), 200)?;
    call(
        "wikipathways/matching_union/1",
        env,
        replay,
        &["sync", "wikipathways"],
        0,
    )?;
    let first = call("wikipathways/matching_union/2", env, replay, args, 0)?;
    let sources = first
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("items")?
        .iter()
        .map(|r| r.pointer("/id/source").and_then(Value::as_str))
        .collect::<Vec<_>>();
    if sources
        != vec![
            Some("reactome"),
            Some("wikipathways"),
            Some("reactome"),
            Some("wikipathways"),
            Some("reactome"),
        ]
    {
        return Err(format!("round robin order: {first}"));
    }
    short_union(env, replay, args)?;
    let mut failed = args.to_vec();
    failed.extend(["--disable-sources", "reactome"]);
    let partial = call("wikipathways/matching_union/3", env, replay, &failed, 0)?;
    if partial.get("status") != Some(&json!("partial"))
        || partial.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || partial.pointer("/result/items/0/id/source") != Some(&json!("wikipathways"))
    {
        return Err(format!("failed leg beside successful leg: {partial}"));
    }
    Ok(())
}
fn short_union(env: &Environment, replay: &Path, args: &[&str]) -> Check {
    use super::launch::fixtures;
    for path in fs::read_dir(fixtures().join("replies/pathway/mixed/reactome"))
        .map_err(|e| e.to_string())?
    {
        let path = path.map_err(|e| e.to_string())?.path();
        fs::copy(
            &path,
            replay
                .join("reactome")
                .join(path.file_name().ok_or("filename")?),
        )
        .map_err(|e| e.to_string())?;
    }
    let mut mixed = args.to_vec();
    mixed.push("--fresh");
    let short = call("wikipathways/short_union/1", env, replay, &mixed, 0)?;
    if short.pointer("/result/page/returned") != Some(&json!(4))
        || short.pointer("/result/page/next_offset") != Some(&json!(4))
    {
        return Err(format!("native admission turn: {short}"));
    }
    let next = short
        .get("next")
        .and_then(Value::as_array)
        .and_then(|a| a.iter().find(|s| s.get("kind") == Some(&json!("page"))))
        .and_then(|s| s.get("args"))
        .and_then(Value::as_array)
        .ok_or("short continuation")?
        .iter()
        .map(|v| v.as_str().ok_or("arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let resumed = call("wikipathways/short_union/2", env, replay, &next, 0)?;
    if resumed.pointer("/result/items/0/id/source") != Some(&json!("reactome"))
        || resumed.pointer("/result/items/1/id/id") != Some(&json!("WP3"))
    {
        return Err(format!("short page changed source turn: {resumed}"));
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
fn read_policy() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, catalog(true, false), catalog(false, false), 200)?;
    call(
        "wikipathways/read_policy/1",
        &env,
        &replay,
        &["sync", "wikipathways"],
        0,
    )?;
    let first = call(
        "wikipathways/read_policy/2",
        &env,
        &replay,
        &["search", "pathway", "Atlas", "--source", "wikipathways"],
        0,
    )?;
    let args = first
        .pointer("/next/5/args")
        .and_then(Value::as_array)
        .ok_or("page")?
        .iter()
        .map(|v| v.as_str().ok_or("arg"))
        .collect::<Result<Vec<_>, _>>()?;
    fs::remove_dir_all(replay.join("wikipathways")).map_err(|e| e.to_string())?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE trial_snapshot SET expires=0", [])
        .map_err(|e| e.to_string())?;
    let expired = call("wikipathways/read_policy/3", &env, &replay, &args, 3)?;
    if expired.pointer("/next/0/kind") != Some(&json!("restart")) {
        return Err(format!("expired catalog pool: {expired}"));
    }
    db.execute_batch(
        "BEGIN; CREATE TEMP TABLE original_content AS SELECT * FROM bundle_content; UPDATE bundle SET \
        fresh_until=0 WHERE name='wikipathways'; INSERT INTO bundle_content SELECT * FROM \
        original_content; COMMIT;",
    )
    .map_err(|e| e.to_string())?;
    let stale = call(
        "wikipathways/read_policy/4",
        &env,
        &replay,
        &[
            "search",
            "pathway",
            "Atlas",
            "--source",
            "wikipathways",
            "--fresh",
        ],
        0,
    )?;
    if stale.get("status") != Some(&json!("partial"))
        || stale.pointer("/sources/0/state") != Some(&json!("stale"))
        || !stale
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|a| a.iter().any(|s| s.get("kind") == Some(&json!("sync"))))
    {
        return Err(format!("stale stored source: {stale}"));
    }
    Ok(())
}

fn exact_species() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut search = catalog(true, false);
    let mut get = catalog(false, false);
    for body in [&mut search, &mut get] {
        *body
            .pointer_mut("/pathwayInfo/0/species")
            .ok_or("species")? = json!(" Homo sapiens");
        *body
            .pointer_mut("/pathwayInfo/1/species")
            .ok_or("species")? = json!("Homo sapiens ");
    }
    let replay = seed(&env, search, get, 200)?;
    let receipt = call(
        "wikipathways/exact_species/1",
        &env,
        &replay,
        &["sync", "wikipathways"],
        0,
    )?;
    if receipt.pointer("/result/search/human_rows") != Some(&json!(10))
        || receipt.pointer("/result/get/human_rows") != Some(&json!(10))
    {
        return Err(format!(
            "padded source species counted as exact human: {receipt}"
        ));
    }
    let found = call(
        "wikipathways/exact_species/2",
        &env,
        &replay,
        &["search", "pathway", "Atlas", "--source", "wikipathways"],
        0,
    )?;
    if found.pointer("/result/page/total") != Some(&json!(10))
        || found.pointer("/result/items/0/id/id") != Some(&json!("WP3"))
    {
        return Err(format!("padded source species admitted: {found}"));
    }
    for id in ["WP1", "WP2"] {
        call(
            "wikipathways/exact_species/3",
            &env,
            &replay,
            &["get", "pathway", id],
            1,
        )?;
    }
    Ok(())
}
