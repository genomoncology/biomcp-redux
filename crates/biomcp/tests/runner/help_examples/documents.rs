//! Bind guide commands to the existing executable help witnesses.
use super::super::{launch, mcp};
use std::{collections::BTreeSet, io::Write, process::Stdio};

type Check = Result<(), String>;

pub(super) fn check(witnessed: &BTreeSet<Vec<String>>) -> Check {
    for text in [
        include_str!("../../../../../README.md"),
        include_str!("../../../../../docs/recipes.md"),
    ] {
        check_text(witnessed, text)?;
    }
    Ok(())
}

fn check_text(witnessed: &BTreeSet<Vec<String>>, text: &str) -> Check {
    for args in commands(text)? {
        if !witnessed.contains(&args) {
            local(&args)?;
        }
    }
    Ok(())
}

fn commands(text: &str) -> Result<BTreeSet<Vec<String>>, String> {
    let mut found = BTreeSet::new();
    for snippet in text.split('`').chain(text.lines()) {
        if let Some(command) = snippet.trim().strip_prefix("biomcp ") {
            let args = shlex::split(command).ok_or("invalid documented shell quoting")?;
            found.insert(args);
        }
    }
    Ok(found)
}

pub(super) fn local(args: &[String]) -> Check {
    let environment = launch::Environment::new().map_err(|e| e.to_string())?;
    if args == ["serve"] {
        let frames = [
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize",
            "params":{"protocolVersion":"2026-07-28","capabilities":{},
            "clientInfo":{"name":"docs-check","version":"1"}}}),
        ];
        let replies = mcp::exchange(&environment, &frames)?;
        if replies.first().and_then(|v| v.get("result")).is_none() {
            return Err("documented stdio startup did not initialize".into());
        }
        return Ok(());
    }
    if args == ["serve", "--http"] {
        return super::super::http::lifecycle();
    }
    let replay = environment.root().join("empty-recordings");
    std::fs::create_dir(&replay).map_err(|e| e.to_string())?;
    // Schema artifacts use the exact raw command path before the option parser.
    let mut globals = if args.first().is_some_and(|arg| arg == "schema") {
        Vec::new()
    } else {
        environment.globals(
            (!args.iter().any(|v| v == "--offline")).then_some(replay.as_path()),
            false,
        )
    };
    globals.extend_from_slice(args);
    let mut command = environment.command(env!("CARGO_BIN_EXE_biomcp"), &globals);
    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut process = super::super::process::Process::new(child)?;
    if args == ["key", "set", "oncokb"] {
        process
            .child
            .stdin
            .take()
            .ok_or("credential stdin")?
            .write_all(b"synthetic-documentation-key\n")
            .map_err(|e| e.to_string())?;
    } else {
        drop(process.child.stdin.take());
    }
    let (status, stdout, stderr) = process.finish()?;
    command
        .record(status, &stdout, "cli")
        .map_err(|e| e.to_string())?;
    if !status.success() || stdout.is_empty() {
        return Err(format!(
            "documented command {args:?} exited {status}: {}",
            String::from_utf8_lossy(&stderr)
        ));
    }
    Ok(())
}

pub(super) fn rejects_invalid() -> Check {
    for command in [
        "biomcp nonexistent-doc-command",
        "biomcp get gene --invented-option",
        "biomcp search drug Tibsovo --source ema",
    ] {
        let result = check_text(&BTreeSet::new(), command);
        if !result.is_err_and(|message| message.contains("exited")) {
            return Err("documentation check accepted a planted invalid command".into());
        }
    }
    Ok(())
}
