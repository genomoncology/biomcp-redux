//! The text view must print source Markdown once; JSON remains the MCP contract.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::Value;
use std::path::Path;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "article/markdown-view-preserves-source-structure",
        || check().map_err(Into::into),
    )]
}
fn check() -> Result<(), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let replay = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/article/fulltext/synthetic");
    let mut args = environment.globals(Some(&replay), false);
    args.extend(["article", "text", "PMC5606870", "--part", "1"].map(str::to_owned));
    let json = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !json.status.success() {
        return Err("JSON reader failed".into());
    }
    let value: Value = serde_json::from_slice(&json.stdout).map_err(|e| e.to_string())?;
    let markdown = value
        .pointer("/result/content/data/markdown")
        .and_then(Value::as_str)
        .ok_or("Markdown missing")?;
    for required in [
        "## Introduction",
        "*text*",
        "IC<sub>50</sub>",
        "[11]",
        "- First **choice**.",
        "**Figure 1.**",
    ] {
        if !markdown.contains(required) {
            return Err(format!("source structure missing {required}"));
        }
    }
    args.extend(["--format".to_owned(), "md".to_owned()]);
    let result = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(result.stdout).map_err(|e| e.to_string())?;
    if !result.status.success()
        || text.matches(markdown).count() != 1
        || text.contains("\\*choice\\*")
    {
        return Err("Markdown view escaped or duplicated source structure".into());
    }
    Ok(())
}
