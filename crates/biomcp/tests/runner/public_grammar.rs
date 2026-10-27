//! Feature generated-help: public grammar and safe argument failures at the CLI boundary.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("public-grammar/argument-errors", || {
            errors().map_err(Into::into)
        }),
        Trial::test("public-grammar/described-help", || {
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
        return Err(format!("wrong exit: {:?}", output.status));
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
    for path in [
        vec![],
        vec!["search"],
        vec!["article"],
        vec!["trial"],
        vec!["variant"],
        vec!["protein"],
        vec!["pathway"],
        vec!["cell-line"],
        vec!["author"],
        vec!["health"],
        vec!["list"],
        vec!["store"],
        vec!["version"],
        vec!["mcp-config"],
        vec!["skill"],
        vec!["get"],
        vec!["serve"],
        vec!["sync"],
        vec!["key"],
        vec!["schema"],
        vec!["help"],
    ] {
        let command = commands
            .iter()
            .find(|c| c["command"] == json!(path))
            .ok_or_else(|| format!("missing command {path:?}"))?;
        if command["description"]
            .as_str()
            .is_none_or(|s| s.trim().is_empty())
        {
            return Err(format!("missing description {path:?}"));
        }
        for arg in command["arguments"].as_array().ok_or("missing arguments")? {
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
    for flag in ["--replay", "--record", "--case-cache", "--observe"] {
        if commands.iter().any(|c| {
            c["arguments"]
                .as_array()
                .is_some_and(|args| args.iter().any(|a| a["name"] == flag))
        }) {
            return Err(format!("public harness flag {flag}"));
        }
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
