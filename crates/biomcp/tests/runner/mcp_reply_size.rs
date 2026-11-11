//! Ten recorded calls measure what the default MCP reply form sends.
use super::super::{
    case_data, launch,
    launch::Environment,
    mcp::{exchange_with_globals, request},
    response_cases,
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::io::Write;

/// Recorded cases whose first step is one ordinary read across entities.
const CASES: [&str; 10] = [
    "gene/braf.yaml",
    "gene/band.yaml",
    "gene/p16.yaml",
    "article/filters/include-retracted.yaml",
    "article/citations/citing.yaml",
    "article/supplements/file-list.yaml",
    "cell-line/examples.yaml",
    "pathway/examples.yaml",
    "protein/native-domains.yaml",
    "variant/population/source-counts.yaml",
];

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial("mcp-parity/reply-sizes", || {
        check().map_err(Into::into)
    })]
}

fn check() -> Result<(), String> {
    for mode in ["single", "typed"] {
        let mut totals = (0, 0);
        for name in CASES {
            let (wire, model) = measure(name, mode)?;
            totals = (totals.0 + wire, totals.1 + model);
        }
        writeln!(
            std::io::stdout(),
            "MCP_REPLY_SIZE mode={mode} case=total wire_bytes={} json_copies_bytes={}",
            totals.0,
            totals.1
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Returns the result's bytes and the bytes of every JSON copy and summary it carries.
fn measure(name: &str, mode: &str) -> Result<(usize, usize), String> {
    let case = case_data::read_case(&launch::fixtures().join("cases").join(name))?;
    let args = case
        .steps
        .first()
        .and_then(|step| step.run.clone())
        .ok_or_else(|| format!("{name}: first step runs nothing"))?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut config = launch::case_config(&case);
    config
        .as_object_mut()
        .ok_or("case config object")?
        .insert("mcp_tool_mode".into(), json!(mode));
    env.config(&config).map_err(|e| e.to_string())?;
    let replay = response_cases::replay(&env, case.replay.as_deref(), case.cache_acquired)?;
    let config_path = env.root().join("config/biomcp/config.json");
    let mut globals = vec!["--config".into(), config_path.to_string_lossy().into()];
    globals.extend(env.globals(replay.as_deref(), false));
    let call = request(
        json!(1),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":args}}),
    );
    let replies = exchange_with_globals(&env, &[call], &globals)?;
    let result = replies
        .first()
        .and_then(|reply| reply.get("result"))
        .ok_or_else(|| format!("{name}: no tool result"))?;
    let text: usize = result
        .get("content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|block| block.get("text").and_then(Value::as_str))
        .map(str::len)
        .sum();
    let structured = result
        .get("structuredContent")
        .map_or(0, |value| value.to_string().len());
    let wire = result.to_string().len();
    let path = server_path(result, &config_path.to_string_lossy());
    writeln!(
        std::io::stdout(),
        "MCP_REPLY_SIZE mode={mode} case={name} wire_bytes={wire} text_bytes={text} \
         structured_bytes={structured} server_path_in_next={path}"
    )
    .map_err(|e| e.to_string())?;
    Ok((wire, text + structured))
}

/// Whether either copy names the server's configuration file; only `next` steps carry paths.
fn server_path(result: &Value, path: &str) -> bool {
    let mut copies = vec![result.get("structuredContent").cloned()];
    copies.extend(
        result
            .get("content")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|block| block.get("text").and_then(Value::as_str))
            .map(|text| Some(Value::String(text.to_owned()))),
    );
    copies
        .into_iter()
        .flatten()
        .any(|copy| next_names(&copy, path))
}

fn next_names(value: &Value, path: &str) -> bool {
    match value {
        Value::String(text) => text.contains(path),
        Value::Array(items) => items.iter().any(|item| next_names(item, path)),
        Value::Object(fields) => fields.values().any(|value| next_names(value, path)),
        _ => false,
    }
}
