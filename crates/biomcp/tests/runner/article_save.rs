//! Explicit saves preserve source bytes and leave ownership with the caller.
use super::launch::{Environment, fixtures};
use serde_json::Value;
use std::fs;

pub(super) fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let path = env.root().join("paper.md");
    let path = path.to_str().ok_or("path")?;
    let output = run(&env, "article/fulltext/live-introduction", &["article", "text", "28944238", "--save", path])?;
    if !output.status.success() {
        return Err(format!("text save failed: {}", String::from_utf8_lossy(&output.stdout)));
    }
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let body = fs::read_to_string(path).map_err(|e| e.to_string())?;
    if !body.contains("# ") || !body.contains("PMC5606870") || !body.contains("cc_by") || !body.contains("Introduction") {
        return Err("saved Markdown lost source header or body".into());
    }
    if value.pointer("/result/content/data/saved/parts").and_then(Value::as_u64).is_none_or(|n| n == 0) {
        return Err(format!("save receipt lacks part count: {value}"));
    }
    let again = run(&env, "article/fulltext/live-introduction", &["article", "text", "28944238", "--save", path])?;
    if again.status.success() || fs::read_to_string(path).map_err(|e| e.to_string())? != body {
        return Err("existing file was overwritten".into());
    }
    Ok(())
}
fn run(env: &Environment, capture: &str, args: &[&str]) -> Result<std::process::Output, String> {
    let mut all: Vec<String> = args.iter().map(|s| (*s).into()).collect();
    all.extend(env.globals(Some(&fixtures().join("replies").join(capture)), false));
    env.command(env!("CARGO_BIN_EXE_biomcp"), &all).output().map_err(|e| e.to_string())
}
