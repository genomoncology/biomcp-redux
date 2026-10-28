//! Feature diagnostic-retirements: old stored data remains visible without refresh.
use crate::runner::transcript::trial as transcript_trial;
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
    mcp,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write};
pub(super) fn trials() -> Vec<Trial> {
    ["gtr", "who-ivd", "fda-cdx"]
        .into_iter()
        .map(|source| {
            transcript_trial(
                format!("diagnostic-retirements/{source}-stale-bundle"),
                move || stale(source).map_err(Into::into),
            )
        })
        .collect()
}
fn replay_fixture(env: &Environment, source: &str) -> Result<std::path::PathBuf, String> {
    let fixture = match source {
        "gtr" => "gtr-synthetic",
        "who-ivd" => "who-synthetic",
        _ => "fda-cdx-synthetic",
    };
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join(source)).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(
        fixtures()
            .join("replies/diagnostic")
            .join(fixture)
            .join(source),
    )
    .map_err(|e| e.to_string())?
    {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join(source).join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn stale(source: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",listener.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    let replay = replay_fixture(&env, source)?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(["sync".into(), source.into()]);
    invoke(&env, args, 0)?;
    fs::remove_dir_all(replay.join(source)).map_err(|e| e.to_string())?;
    let cache = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    cache
        .execute_batch(
            "BEGIN; CREATE TEMP TABLE saved_content AS SELECT * FROM bundle_content; UPDATE bundle SET \
        fresh_until=0; INSERT INTO bundle_content SELECT * FROM saved_content; COMMIT;",
        )
        .map_err(|e| e.to_string())?;
    let tail = match source {
        "gtr" => vec!["get", "diagnostic", "GTR000006692.3", "--source", source],
        "who-ivd" => vec!["get", "diagnostic", "0466-046-00", "--source", source],
        _ => vec![
            "search",
            "diagnostic",
            "--source",
            source,
            "--device-id",
            "P200010/S026",
        ],
    };
    for fresh in [false, true] {
        let globals = env.globals(Some(&replay), false);
        let mut command: Vec<String> = tail.iter().map(|v| (*v).into()).collect();
        let mut args = globals.clone();
        args.extend(command.clone());
        if fresh {
            args.push("--fresh".into());
            command.push("--fresh".into());
        }
        let value = invoke(&env, args.clone(), 0)?;
        if value.get("status") != Some(&json!("partial"))
            || value.pointer("/sources/0/state") != Some(&json!("stale"))
            || value.get("result").is_none()
            || !value
                .get("next")
                .and_then(Value::as_array)
                .is_some_and(|steps| {
                    steps
                        .iter()
                        .any(|v| v.get("args") == Some(&json!(["sync", source])))
                })
        {
            return Err(format!("stale bundle lost data or named sync: {value}"));
        }
        bounded(source, &value)?;
        if mcp::call_with_globals(&env, &command, false, &globals)? != value {
            return Err("stale CLI/MCP mismatch".into());
        }
    }
    if listener.accept().is_ok() {
        return Err("stale read opened source connection".into());
    }
    Ok(())
}

fn bounded(source: &str, value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    let baseline: usize = match source {
        "gtr" => 1559,
        "who-ivd" => 2576,
        _ => 1997,
    };
    writeln!(std::io::stdout(), "{source} stale: {bytes}").map_err(|e| e.to_string())?;
    if bytes > (baseline * 115).div_ceil(100) {
        return Err(format!(
            "{source} stale output exceeds its measured ceiling"
        ));
    }
    Ok(())
}
