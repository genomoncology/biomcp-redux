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
    for (args, replies, required) in [
        (
            vec!["get", "gene", "BRAF"],
            "gene/main",
            vec!["# BRAF", "**status:**", "biomcp get gene"],
        ),
        (
            vec!["get", "gene", "BRAF", "protein"],
            "gene/main",
            vec!["# BRAF", "Protein", "**state:**"],
        ),
        (
            vec!["search", "gene", "BRAF"],
            "gene/main",
            vec!["| Symbol |", "Hgnc id", "BRAF"],
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
            vec!["get", "gene"],
            "gene/main",
            vec!["Error", "**status:**", "usage"],
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/replies")
            .join(replies);
        let mut command = env.globals(Some(&replay), false);
        command.extend(args.into_iter().map(str::to_owned));
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
        let tool = super::mcp::call_with_globals(
            &env,
            &["--format".into(), "md".into(), "get".into(), "gene".into()],
            true,
            &[],
        )?;
        if tool.get("schema") != Some(&json!("biomcp.error/1")) {
            return Err("MCP did not keep JSON".into());
        }
        if value.get("status").is_none() {
            return Err("JSON baseline missing status".into());
        }
        if text.len() > 1 {
            return Err(format!(
                "MEASURE {} bytes: {}",
                command.join(" "),
                text.len()
            ));
        }
        command.extend(["--format".into(), "json".into()]);
        let explicit = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        if explicit.stdout != json_output.stdout {
            return Err("explicit JSON changed under Markdown configuration".into());
        }
    }
    Ok(())
}
