//! Housekeeping runs through the isolated installed command boundary.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Output};
type Check<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "housekeeping/offline-provenance-and-config",
        || check().map_err(|error| error.to_string().into()),
    )]
}
fn run(env: &Environment, args: &[&str], empty: bool) -> Check<Output> {
    let args: Vec<_> = args.iter().map(|s| (*s).to_owned()).collect();
    let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    if empty {
        for (key, _) in super::launch::ENV_PATHS {
            command.env_remove(key);
        }
    }
    Ok(command.output()?)
}
fn success(output: &Output) -> Check<&str> {
    if !output.status.success() {
        return Err(format!("housekeeping failed: {:?}", output).into());
    }
    Ok(std::str::from_utf8(&output.stdout)?)
}
fn check() -> Check {
    let env = Environment::new()?;
    env.config(&json!({"keys":{"unknown":"never-read"},"format":"md"}))?;
    let config = env.root().join("config/biomcp/config.json");
    fs::write(&config, b"invalid config must never be read")?;
    for empty in [false, true] {
        let output = run(&env, &["version", "--offline"], empty)?;
        if success(&output)? != format!("biomcp {}\n", env!("CARGO_PKG_VERSION")) {
            return Err("wrong version".into());
        }
        let output = run(&env, &["version", "--verbose", "--offline"], empty)?;
        let text = success(&output)?;
        for field in ["Commit: ", "Target: ", "Schemas: ", "Config: ", "Cache: "] {
            if !text.contains(field) {
                return Err(format!("missing {field}: {text}").into());
            }
        }
        if !empty && !text.contains(&config.display().to_string()) {
            return Err("config path does not match the settings boundary".into());
        }
        for client in [None, Some("mcp-servers"), Some("servers"), Some("json")] {
            check_config(&env, client, empty)?;
        }
        let output = run(
            &env,
            &["mcp-config", "--client", "unknown", "--offline"],
            empty,
        )?;
        if output.status.code() != Some(2) {
            return Err("unknown client was not a usage error".into());
        }
    }
    if env.root().join("cache/biomcp").exists() {
        return Err("housekeeping created a cache".into());
    }
    Ok(())
}
fn check_config(env: &Environment, client: Option<&str>, empty: bool) -> Check {
    let mut args = vec!["mcp-config", "--offline", "--absolute-path"];
    if let Some(client) = client {
        args.extend(["--client", client]);
    }
    let output = run(env, &args, empty)?;
    let value: Value = serde_json::from_str(success(&output)?)?;
    let key = if client == Some("servers") {
        "servers"
    } else {
        "mcpServers"
    };
    let entry = value
        .get(key)
        .and_then(|v| v.get("biomcp"))
        .ok_or("missing server")?;
    let path = entry
        .get("command")
        .and_then(Value::as_str)
        .ok_or("missing executable")?;
    if !Path::new(path).is_absolute()
        || !Path::new(path).is_file()
        || entry.get("args") != Some(&json!(["serve"]))
        || value.as_object().is_none_or(|map| map.len() != 1)
    {
        return Err(format!("invalid MCP configuration: {value}").into());
    }
    Ok(())
}
