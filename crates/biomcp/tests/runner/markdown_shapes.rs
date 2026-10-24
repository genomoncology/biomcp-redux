//! People can read every command shape and copy its next commands.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::Path;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("view/markdown-command-shapes", || {
        shapes().map_err(Into::into)
    })]
}
fn shapes() -> Result<(), String> {
    let mut measurements = Vec::new();
    for (args, replies, required) in [
        (
            vec!["get", "gene", "BRAF"],
            "gene/main",
            vec!["# BRAF", "**status:**", "biomcp get gene"],
        ),
        (
            vec!["get", "gene", "CDKN2A", "protein"],
            "gene/protein",
            vec!["# CDKN2A", "Protein", "**state:**", "P42771", "Q8N726"],
        ),
        (
            vec!["search", "gene", "KRAS"],
            "gene/main",
            vec!["| Symbol |", "Hgnc id", "KRAS"],
        ),
        (
            vec!["search", "article", "AKT1 E17K"],
            "article/search/live",
            vec!["| Pmid |", "Title", "biomcp"],
        ),
        (
            vec!["get", "article", "33123537"],
            "article/get/synthetic-derived",
            vec!["# ", "**status:**", "biomcp"],
        ),
        (
            vec!["article", "text", "PMC5606870", "--part", "1"],
            "article/fulltext/synthetic",
            vec!["## Introduction", "*text*", "biomcp article text"],
        ),
        (
            vec!["get", "gene"],
            "gene/main",
            vec!["Error", "**status:**", "usage"],
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/replies")
            .join(replies);
        let globals = env.globals(Some(&replay), false);
        let tool_args: Vec<String> = args.into_iter().map(str::to_owned).collect();
        let mut command = globals.clone();
        command.extend(tool_args.iter().cloned());
        let json_output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value =
            serde_json::from_slice(&json_output.stdout).map_err(|e| e.to_string())?;
        env.config(&json!({"format":"md"}))
            .map_err(|e| e.to_string())?;
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        for required in required {
            if !text.contains(required) {
                return Err(format!("{command:?} lacks {required}: {text}"));
            }
        }
        if output.status != json_output.status || !output.stderr.is_empty() {
            return Err("Markdown changed command status or emitted diagnostics".into());
        }
        let mut tool_args = tool_args;
        tool_args.extend(["--format".into(), "md".into()]);
        let tool = super::mcp::call_with_globals(
            &env,
            &tool_args,
            !json_output.status.success(),
            &globals,
        )?;
        if tool != value {
            return Err("MCP changed JSON under Markdown configuration".into());
        }
        measurements.push(format!("{} bytes: {}", command.join(" "), text.len()));
        command.extend(["--format".into(), "json".into()]);
        let explicit = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        if explicit.stdout != json_output.stdout {
            return Err("explicit JSON changed under Markdown configuration".into());
        }
    }
    Err(measurements.join("\n"))
}
