//! Required public surface comes from shipped help and the advertised MCP registry.
#[path = "../runner/case_data.rs"]
mod case_data;
#[path = "../../../biomcp-fetch/tests/support/environment.rs"]
mod environment;
#[path = "required/tally.rs"]
mod tally;
use super::Check;
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, io::Write, path::Path, process::Stdio};

fn metadata() -> Result<(Value, Vec<Value>), Box<dyn std::error::Error>> {
    let home = environment::new()?;
    let binary = Path::new(env!("CARGO_BIN_EXE_biomcp"));
    if !binary.is_file() {
        return Err("required coverage: build the current product binary before ratchets".into());
    }
    let output = environment::command(binary, home.path(), &["help".into()]).output()?;
    if !output.status.success() {
        return Err("required coverage: shipped help failed".into());
    }
    let help: Value = serde_json::from_slice(&output.stdout)?;
    let mut process = environment::command(binary, home.path(), &["serve".into()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut input = process.stdin.take().ok_or("MCP stdin missing")?;
    for frame in [
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
            "protocolVersion":"2026-07-28","capabilities":{},"clientInfo":{"name":"required-coverage","version":"1"}}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}),
    ] {
        serde_json::to_writer(&mut input, &frame)?;
        input.write_all(b"\n")?;
    }
    drop(input);
    let output = process.wait_with_output()?;
    if !output.status.success() {
        return Err("required coverage: MCP tool registry failed".into());
    }
    for line in output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|s| !s.is_empty())
    {
        let frame: Value = serde_json::from_slice(line)?;
        if frame["id"] == 2 {
            return Ok((
                help,
                frame
                    .pointer("/result/tools")
                    .and_then(Value::as_array)
                    .ok_or("MCP tool registry missing")?
                    .clone(),
            ));
        }
    }
    Err("MCP tool registry response missing".into())
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Repository ratchets own their working directory boundary"
)]
fn required_surface_has_asserted_calls() -> Check {
    let root = std::env::current_dir()?.join("../..");
    let (help, tools) = metadata()?;
    let mut counts = tally::Inventory::new(&help, &tools)?;
    let mut paths = Vec::new();
    case_data::collect(&root.join("tests/cases"), &mut paths)?;
    paths.sort();
    for path in paths {
        let case = case_data::read_case(&path)?;
        for step in case.steps {
            counts.step(&step);
        }
    }
    let pending = fs::read_to_string(root.join("crates/biomcp/tests/ratchets/required.pending"))?;
    let pending: BTreeSet<_> = pending.lines().map(str::to_owned).collect();
    let mut output = std::io::stdout().lock();
    for (item, count) in &counts.calls {
        writeln!(output, "required {count:5} {item}")?;
    }
    counts.check(&pending)
}
