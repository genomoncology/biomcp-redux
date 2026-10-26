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
        if !text.contains(&format!("Commit: {}\n", env!("BIOMCP_BUILD_COMMIT")))
            || !text.contains(&format!("Target: {}\n", env!("BIOMCP_BUILD_TARGET")))
            || !text.contains("biomcp.help/2")
            || !text.contains("biomcp.error/1")
        {
            return Err("verbose build or schema provenance differs from this artifact".into());
        }
        if !empty && !text.contains(&config.display().to_string()) {
            return Err("config path does not match the settings boundary".into());
        }
        for client in [None, Some("mcp-servers"), Some("servers"), Some("json")] {
            check_config(&env, client, empty)?;
        }
        shell_client(&env, empty)?;
        invalid_client(&env, empty)?;
    }
    env.config(&json!({"format":"md","keys":{"ncbi_api_key":"never-read"}}))?;
    invalid_client(&env, false)?;
    env.config(&json!({"format":"json"}))?;
    help(&env)?;
    if env.root().join("cache/biomcp").exists() {
        return Err("housekeeping created a cache".into());
    }
    Ok(())
}
fn check_config(env: &Environment, client: Option<&str>, empty: bool) -> Check {
    let mut args = vec!["mcp-config", "--offline"];
    if let Some(client) = client {
        args.extend(["--client", client, "--absolute-path"]);
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
    let expected = env.command(env!("CARGO_BIN_EXE_biomcp"), &[]);
    if Path::new(path).canonicalize()? != Path::new(expected.get_program()).canonicalize()?
        || !Path::new(path).is_absolute()
        || !Path::new(path).is_file()
        || entry.get("args") != Some(&json!(["serve"]))
        || value.as_object().is_none_or(|map| map.len() != 1)
    {
        return Err(format!("invalid MCP configuration: {value}").into());
    }
    Ok(())
}

fn help(env: &Environment) -> Check {
    for (name, flag) in [("version", "--verbose"), ("mcp-config", "--client")] {
        let output = run(env, &["help", name], false)?;
        let value: Value = serde_json::from_str(success(&output)?)?;
        let commands = value
            .pointer("/result/commands")
            .and_then(Value::as_array)
            .ok_or("missing help")?;
        if commands.len() != 1
            || commands
                .first()
                .and_then(|c| c.get("arguments"))
                .and_then(Value::as_array)
                .is_none_or(|args| !args.iter().any(|arg| arg.get("name") == Some(&json!(flag))))
        {
            return Err(format!("missing {name} {flag} help: {value}").into());
        }
    }
    Ok(())
}

fn invalid_client(env: &Environment, empty: bool) -> Check {
    let output = run(
        env,
        &["mcp-config", "--client", "unknown", "--offline"],
        empty,
    )?;
    if output.status.code() != Some(2) {
        return Err("unknown client was not a usage error".into());
    }
    let value: Value = serde_json::from_slice(&output.stdout)?;
    if value.pointer("/next/0/args") != Some(&json!(["mcp-config", "--client", "mcp-servers"])) {
        return Err("invalid client omitted the supported replacement command".into());
    }
    Ok(())
}

fn shell_client(env: &Environment, empty: bool) -> Check {
    let output = run(
        env,
        &["mcp-config", "--client", "codex", "--offline"],
        empty,
    )?;
    let words = shlex::split(success(&output)?).ok_or("invalid shell command")?;
    let expected = env.command(env!("CARGO_BIN_EXE_biomcp"), &[]);
    let path = Path::new(expected.get_program()).canonicalize()?;
    let expected = [
        "codex",
        "mcp",
        "add",
        "biomcp",
        "--",
        path.to_str().ok_or("invalid executable path")?,
        "serve",
    ];
    if words != expected {
        return Err(format!("wrong shell client configuration: {words:?}").into());
    }
    Ok(())
}
