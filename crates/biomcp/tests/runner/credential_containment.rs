//! The actual CLI refuses reflected outbound credentials before persistence or output.
#![expect(
    clippy::indexing_slicing,
    reason = "Response-only mutations check the captured fixture shape"
)]
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
const KEY: &str = "synthetic-containment-key";
const SEARCH: &[&str] = &[
    "search",
    "trial",
    "--source",
    "nci",
    "--keyword",
    "synthetic containment",
];

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("credentials/reflected-wire-never-persists", || {
            wire().map_err(Into::into)
        }),
        Trial::test("credentials/contaminated-replay-and-cache-refused", || {
            stored().map_err(Into::into)
        }),
    ]
}
fn reply(request: &str) -> Result<String, String> {
    if request.contains("keyword=body") {
        Ok(json!({"total":0,"data":[],"ignored":KEY}).to_string())
    } else if request.contains("keyword=escaped") {
        Ok(json!({"total":0,"data":[],"ignored":KEY})
            .to_string()
            .replace("synthetic", "\\u0073ynthetic"))
    } else if request.contains("keyword=duplicate") {
        Ok(r#"{"total":0,"data":[],"ignored":"\u0073ynthetic-containment-key","ignored":"safe"}"#.into())
    } else if request.contains("keyword=deep") {
        Ok(format!(r#"{{"total":0,"data":[],"ignored":{}"\u0073ynthetic-containment-key"{}}}"#, "[".repeat(130), "]".repeat(130)))
    } else if request.contains("keyword=error") {
        Ok(KEY.into())
    } else {
        Ok(json!({"total":0,"data":[],"ignored":"synthetic-unrelated-config-key"}).to_string())
    }
}
fn setup() -> Result<(Environment, Stub), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::with_status(
        "clinicaltrialsapi.cancer.gov",
        |request| {
            if request.contains("keyword=media") {
                "application/json; synthetic-containment-key"
            } else {
                "application/json"
            }
        },
        reply,
        |_, bytes| bytes,
        |request| {
            if request.contains("keyword=error") {
                400
            } else {
                200
            }
        },
    )?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,"keys":{"nci_api_key":KEY,"semantic_scholar_api_key":"synthetic-unrelated-config-key"}})).map_err(|e|e.to_string())?;
    Ok((env, stub))
}
fn invoke(env: &Environment, args: Vec<String>, bad: bool) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    for bytes in [&output.stdout, &output.stderr] {
        clean(bytes)?;
    }
    if output.status.code() != Some(if bad { 4 } else { 0 }) || !output.stderr.is_empty() {
        let parsed: Value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
        return Err(format!(
            "containment command exit {:?}, expected {}; status {:?}, source reason {:?}",
            output.status.code(),
            if bad { 4 } else { 0 },
            parsed.get("status"),
            parsed.pointer("/sources/0/reason")
        ));
    }
    let out: Value =
        serde_json::from_slice(&output.stdout).map_err(|_| "invalid containment output")?;
    if bad && out.pointer("/sources/0/reason") != Some(&json!("bad_reply")) {
        return Err("contamination was not a sanitized bad_reply".into());
    }
    if !bad && out.pointer("/result/items") != Some(&json!([])) {
        return Err("ordinary keyed reply changed".into());
    }
    Ok(out)
}
fn args(query: &str, flag: &str, path: &Path) -> Vec<String> {
    let mut args = SEARCH
        .iter()
        .take(SEARCH.len() - 1)
        .map(|s| (*s).into())
        .collect::<Vec<String>>();
    args.extend([
        query.into(),
        flag.into(),
        path.to_string_lossy().into_owned(),
    ]);
    args
}
fn clean(bytes: &[u8]) -> Check {
    if bytes.windows(KEY.len()).any(|s| s == KEY.as_bytes()) {
        return Err("reflected credential escaped its wire boundary".into());
    }
    Ok(())
}
fn files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    if root.exists() {
        for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                paths.extend(files(&entry.path())?);
            } else {
                paths.push(entry.path());
            }
        }
    }
    Ok(paths)
}
fn wire() -> Check {
    let (env, stub) = setup()?;
    for query in ["duplicate", "deep", "body", "escaped", "error", "media"] {
        let dir = env.root().join(format!("record-{query}"));
        invoke(&env, args(query, "--record", &dir), true)?;
        for path in files(&dir)?
            .into_iter()
            .chain(files(&env.root().join("cache"))?)
        {
            clean(&fs::read(path).map_err(|e| e.to_string())?)?;
        }
        if !files(&dir)?.is_empty() {
            return Err("reflected response entered a recording".into());
        }
    }
    let dir = env.root().join("record-safe");
    invoke(&env, args("safe", "--record", &dir), false)?;
    invoke(&env, args("safe", "--replay", &dir), false)?;
    let requests = stub.finish()?;
    if requests.len() != 7
        || requests.iter().any(|(_, request)| {
            !request
                .lines()
                .any(|line| line.eq_ignore_ascii_case(&format!("x-api-key: {KEY}")))
        })
    {
        return Err("containment proof did not authenticate its exact synthetic requests".into());
    }
    Ok(())
}
fn stored() -> Check {
    let (env, stub) = setup()?;
    let dir = env.root().join("original");
    invoke(&env, args("safe", "--record", &dir), false)?;
    stub.finish()?;
    let entry = files(&dir)?
        .into_iter()
        .next()
        .ok_or("missing safe recording")?;
    let original = fs::read(&entry).map_err(|e| e.to_string())?;
    let mut record: Value = serde_json::from_slice(&original).map_err(|e| e.to_string())?;
    record["exchanges"][0]["body"]["value"]["ignored"] = json!(KEY);
    let changed = serde_json::to_vec(&record).map_err(|e| e.to_string())?;
    fs::write(&entry, &changed).map_err(|e| e.to_string())?;
    invoke(&env, args("safe", "--replay", &dir), true)?;
    if fs::read(&entry).map_err(|e| e.to_string())? != changed {
        return Err("contaminated recording was rewritten".into());
    }
    fs::write(&entry, original).map_err(|e| e.to_string())?;
    let cache = env.root().join("case-cache");
    let mut replay = args("safe", "--replay", &dir);
    replay.extend(["--case-cache".into(), cache.to_string_lossy().into()]);
    invoke(&env, replay, false)?;
    let connection =
        rusqlite::Connection::open(cache.join("replay.sqlite")).map_err(|e| e.to_string())?;
    let body = json!({"total":0,"data":[],"ignored":KEY}).to_string();
    for fresh in [i64::MAX, 0] {
        connection
            .execute(
                "UPDATE reply SET body=?1, fresh_until=?2",
                rusqlite::params![body.as_bytes(), fresh],
            )
            .map_err(|e| e.to_string())?;
        invoke(
            &env,
            args("safe", "--case-cache", &cache)
                .into_iter()
                .chain(["--offline".into()])
                .collect(),
            true,
        )?;
        let saved: Vec<u8> = connection
            .query_row("SELECT body FROM reply", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        if saved != body.as_bytes() {
            return Err("contaminated cache evidence was rewritten".into());
        }
    }
    Ok(())
}
