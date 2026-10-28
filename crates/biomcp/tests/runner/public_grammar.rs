//! Feature generated-help: public grammar and safe argument failures at the CLI boundary.
use crate::runner::transcript::trial as transcript_trial;
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("public-grammar/argument-errors", || {
            errors().map_err(Into::into)
        }),
        transcript_trial("public-grammar/described-help", || {
            help().map_err(Into::into)
        }),
    ]
}
fn invoke(args: &[&str], exit: i32) -> Result<Value, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "wrong exit: {:?}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    if String::from_utf8_lossy(&output.stdout).contains("private-sentinel")
        || String::from_utf8_lossy(&output.stderr).contains("private-sentinel")
    {
        return Err("argument value leaked".into());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn errors() -> Result<(), String> {
    for (args, option) in [
        (vec!["get", "gene"], "input"),
        (vec!["search", "trial", "--limit", "0"], "--limit"),
        (
            vec!["search", "trial", "--limit", "private-sentinel"],
            "--limit",
        ),
        (vec!["--format", "private-sentinel", "health"], "--format"),
        (vec!["serve", "--http", "0.0.0.0:0"], "--allow-host"),
    ] {
        let value = invoke(&args, 2)?;
        if value.pointer("/error/code") != Some(&json!("usage.arguments"))
            || value.pointer("/error/option") != Some(&json!(option))
        {
            return Err(format!("missing safe argument identity {option}: {value}"));
        }
    }
    Ok(())
}
fn help() -> Result<(), String> {
    let value = invoke(&["help"], 0)?;
    let commands = value
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .ok_or("missing help commands")?;
    command_metadata(commands)?;
    for command in commands {
        for argument in arguments(command)? {
            if argument["description"]
                .as_str()
                .is_none_or(|s| s.trim().is_empty())
            {
                return Err(format!("public argument lacks description: {argument}"));
            }
        }
    }
    for flag in ["--replay", "--record", "--case-cache", "--observe"] {
        if commands.iter().any(|c| {
            c["arguments"]
                .as_array()
                .is_some_and(|args| args.iter().any(|a| a["name"] == flag))
        }) {
            return Err(format!("public harness flag {flag}"));
        }
    }
    let references = commands
        .iter()
        .find(|c| c["command"] == json!(["article", "references"]))
        .ok_or("missing article references")?;
    if references["description"]
        .as_str()
        .is_none_or(|s| !s.contains("referenced") || s.contains("citing or"))
    {
        return Err("article references has no command-specific description".into());
    }
    let author = commands
        .iter()
        .find(|c| c["command"] == json!(["search", "author"]))
        .ok_or("missing search author")?;
    if author["description"]
        .as_str()
        .is_none_or(|s| s.contains("papers supports"))
    {
        return Err("search author describes papers options".into());
    }
    Ok(())
}

fn command_metadata(commands: &[Value]) -> Result<(), String> {
    let index = commands
        .iter()
        .find(|c| c.get("command").is_none())
        .ok_or("missing command index")?;
    let mut paths = std::collections::BTreeSet::from([Vec::new()]);
    for line in index["description"]
        .as_str()
        .ok_or("missing index description")?
        .lines()
    {
        let (name, _) = line.split_once(": ").ok_or("invalid command index")?;
        paths.insert(vec![name.to_owned()]);
    }
    for command in commands {
        if let Some(name) = command.pointer("/command/0").and_then(Value::as_str) {
            paths.insert(vec![name.to_owned()]);
        }
    }
    for path in paths {
        if let Some(name) = path.first() {
            invoke(&["help", name], 0).map_err(|error| format!("help {name}: {error}"))?;
        }
        let command = commands
            .iter()
            .find(|c| {
                if path.is_empty() {
                    c.get("command").is_none()
                } else {
                    c["command"] == json!(path)
                }
            })
            .ok_or_else(|| format!("missing command {path:?}"))?;
        if command["description"]
            .as_str()
            .is_none_or(|s| s.trim().is_empty())
        {
            return Err(format!("missing description {path:?}"));
        }
        for arg in arguments(command)? {
            if arg["description"]
                .as_str()
                .is_none_or(|s| s.trim().is_empty())
            {
                return Err(format!("missing argument description {arg}"));
            }
        }
    }
    for path in [
        vec!["serve"],
        vec!["sync"],
        vec!["key", "set"],
        vec!["schema"],
        vec!["help"],
    ] {
        let command = commands
            .iter()
            .find(|c| c["command"] == json!(path))
            .ok_or("missing argument command")?;
        if command["arguments"].as_array().is_none_or(Vec::is_empty) {
            return Err(format!("missing arguments {path:?}"));
        }
    }
    Ok(())
}

fn arguments(command: &Value) -> Result<&[Value], String> {
    match command.get("arguments") {
        None => Ok(&[]),
        Some(value) => value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| "invalid public arguments".into()),
    }
}
