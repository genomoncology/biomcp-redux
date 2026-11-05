//! Compare raw single-tool wire replies with the verified unchanged product baseline.
use super::super::super::{
    launch::{self, Environment},
    mcp::request,
    process,
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, process::Stdio};

pub(super) fn check() -> Result<(), String> {
    let baseline = launch::typed_baseline()
        .ok_or("BIOMCP_TYPED_BASELINE is required for single-mode wire parity")?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let frames = [
        request(json!(1), "server/discover", json!({})),
        request(json!(2), "tools/list", json!({})),
        request(
            json!(3),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":["get","variant","BRAF V600E"]}}),
        ),
        request(
            json!(4),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":["search","trial","--condition","melanoma"]}}),
        ),
    ];
    let expected = exchange(
        &env,
        &baseline,
        &["serve".into(), "--offline".into()],
        &frames,
    )?;
    for mode in [
        vec!["serve".into(), "--offline".into()],
        vec![
            "serve".into(),
            "--offline".into(),
            "--tool-mode".into(),
            "single".into(),
        ],
    ] {
        let actual = exchange(
            &env,
            std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &mode,
            &frames,
        )?;
        compare_frames(&env, &mode, &frames, &expected, &actual)?;
    }
    invalid_id_witness()
}
pub(super) fn exchange(
    env: &Environment,
    binary: &std::path::Path,
    args: &[String],
    frames: &[Value],
) -> Result<BTreeMap<String, String>, String> {
    let mut transcript_args = args.to_vec();
    transcript_args.extend(frames.iter().map(Value::to_string));
    let mut command = env.command_with_arguments(binary, args, &transcript_args);
    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let (status, stdout, stderr) = process::exchange(child, frames)?;
    command
        .record(status, &stdout, "mcp")
        .map_err(|e| e.to_string())?;
    if !status.success() || !stderr.is_empty() {
        return Err(format!(
            "baseline process failed: {}",
            String::from_utf8_lossy(&stderr)
        ));
    }
    String::from_utf8(stdout)
        .map_err(|e| e.to_string())?
        .lines()
        .map(|line| {
            let value: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            Ok((
                value.get("id").ok_or("missing reply id")?.to_string(),
                line.to_owned(),
            ))
        })
        .collect()
}

// Named differences authorized by m25441; every other byte remains frozen.
pub(super) fn named_hint(
    env: &Environment,
    server_args: &[String],
    args: &[String],
    old: &str,
    new: &str,
) -> Result<bool, String> {
    let Some(expected_next) = named_next(args) else {
        return Ok(false);
    };
    let old_value: Value = serde_json::from_str(old).map_err(|e| e.to_string())?;
    let envelope = old_value
        .pointer("/result/structuredContent")
        .ok_or("frozen envelope")?;
    let entity = args.get(1).ok_or("named get entity")?;
    let old_next = json!([{"args":["help","get",entity],"kind":"configure"}]);
    if old_value.pointer("/result/isError") != Some(&json!(true))
        || envelope.get("status") != Some(&json!("usage"))
        || envelope.get("next") != Some(&old_next)
    {
        return Ok(false);
    }
    let text = old_value
        .pointer("/result/content/0/text")
        .and_then(Value::as_str)
        .ok_or("frozen text")?;
    if serde_json::from_str::<Value>(text).map_err(|e| e.to_string())? != *envelope {
        return Ok(false);
    }
    let old_literal = old_next.to_string();
    let old_text = escaped_fragment(&old_next)?;
    if old.matches(&old_literal).count() != 1 || old.matches(&old_text).count() != 1 {
        return Ok(false);
    }
    let expected = old
        .replace(&old_literal, &expected_next.to_string())
        .replace(&old_text, &escaped_fragment(&expected_next)?);
    if expected != new {
        return Ok(false);
    }
    let call = expected_next.pointer("/0/args").ok_or("named next args")?;
    execute_named_next(env, server_args, call)?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "TYPED_NAMED_HINT args={args:?} exact_next_executed={call} ruling=m25441"
    )
    .map_err(|e| e.to_string())?;
    Ok(true)
}

fn execute_named_next(
    env: &Environment,
    server_args: &[String],
    call: &Value,
) -> Result<(), String> {
    let mut recovery_args = server_args.to_vec();
    // Validation-only cases have no source capture; keep the recovery probe offline.
    if !server_args.iter().any(|arg| {
        arg == "--replay"
            || arg.starts_with("--replay=")
            || arg == "--offline"
            || arg == "--offline=true"
    }) {
        recovery_args.push("--offline".into());
    }
    let replies = exchange(
        env,
        std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
        &recovery_args,
        &[request(
            json!(1),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":call}}),
        )],
    )?;
    let reply: Value = serde_json::from_str(replies.get("1").ok_or("named next reply")?)
        .map_err(|e| e.to_string())?;
    if reply
        .pointer("/result/structuredContent/status")
        .and_then(Value::as_str)
        .is_none_or(|status| status == "usage")
    {
        return Err(format!(
            "approved named next call failed to execute: {call}: {reply}"
        ));
    }
    Ok(())
}

fn named_next(args: &[String]) -> Option<Value> {
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["get", "variant", "BRAF V600E"] => Some(
            json!([{"args":["search","variant","--query=BRAF V600E","--source=myvariant"],"kind":"disambiguate"}]),
        ),
        ["get", "trial", "lung cancer"] => {
            Some(json!([{"args":["search","trial","--","lung cancer"],"kind":"disambiguate"}]))
        }
        _ => None,
    }
}

fn escaped_fragment(value: &Value) -> Result<String, String> {
    let encoded = serde_json::to_string(&value.to_string()).map_err(|e| e.to_string())?;
    encoded
        .get(1..encoded.len().saturating_sub(1))
        .map(str::to_owned)
        .ok_or_else(|| "escaped JSON fragment".into())
}

fn compare_frames(
    env: &Environment,
    server_args: &[String],
    frames: &[Value],
    old: &BTreeMap<String, String>,
    new: &BTreeMap<String, String>,
) -> Result<(), String> {
    if old.keys().ne(new.keys()) {
        return Err("single mode changed reply IDs".into());
    }
    for frame in frames {
        let id = frame.get("id").ok_or("frame id")?.to_string();
        let before = old.get(&id).ok_or("frozen reply")?;
        let after = new.get(&id).ok_or("current reply")?;
        if before == after {
            continue;
        }
        let args: Vec<String> = frame
            .pointer("/params/arguments/args")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        if !named_hint(env, server_args, &args, before, after)? {
            return Err(format!(
                "unapproved single RAW frame {id}: frozen={before}; current={after}"
            ));
        }
    }
    Ok(())
}

pub(super) fn invalid_id_witness() -> Result<(), String> {
    use std::io::Write;
    let baseline = launch::typed_baseline()
        .ok_or("BIOMCP_TYPED_BASELINE is required for single-mode wire parity")?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let frames = [request(
        json!(1),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":["get","trial","lung cancer"]}}),
    )];
    let args = ["serve".into(), "--offline".into()];
    let old = exchange(&env, &baseline, &args, &frames)?;
    let current = exchange(
        &env,
        std::path::Path::new(env!("CARGO_BIN_EXE_biomcp")),
        &args,
        &frames,
    )?;
    for (label, replies) in [("frozen-ed9c", &old), ("current-default-single", &current)] {
        for raw in replies.values() {
            writeln!(
                std::io::stdout(),
                "TYPED_INVALID_ID_RAW {label} mode=serve-offline isolated-config=default {raw}"
            )
            .map_err(|e| e.to_string())?;
        }
    }
    compare_frames(&env, &args, &frames, &old, &current)
}
