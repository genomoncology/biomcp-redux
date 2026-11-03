//! Feature article-fulltext: compact outline presentation retains card and part facts.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::Value;
use std::path::Path;

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript::trial("article-fulltext/compact-outline", || {
        check().map_err(Into::into)
    })]
}

fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let binary = Path::new(env!("CARGO_BIN_EXE_biomcp"));
    let json = read(&env, binary, "json")?;
    let reply: Value = serde_json::from_str(&json).map_err(|error| error.to_string())?;
    let markdown = read(&env, binary, "md")?;
    let item = &reply["result"]["items"][0];
    let find = serde_json::json!(["article", "text", "28944238", "--find", "<text>"]);
    if !reply["next"].as_array().is_some_and(|steps| steps.iter().any(|step| step["args"] == find)) {
        return Err("outline did not offer the literal --find fill-in suggestion".into());
    }
    let title = item["answer"]["card"]["title"].as_str().ok_or("missing card title")?;
    if markdown.matches(title).count() != 1 {
        return Err("article title was repeated or lost".into());
    }
    let parts = item["sections"]["fulltext"]["data"]["parts"]
        .as_array().ok_or("missing outline parts")?;
    if parts.len() < 2 || !parts.windows(2).any(|pair| pair[0]["title"] == pair[1]["title"]) {
        return Err("fixture must exercise repeated part titles".into());
    }
    let mut previous = None;
    for part in parts {
        let title = part["title"].as_str().ok_or("missing part title")?;
        if previous != Some(title) && !markdown.contains(&format!("###### {title}\n")) {
            return Err(format!("outline lost title {title}"));
        }
        let row = format!("| {} | {} | {} |", part["kind"].as_str().unwrap_or_default(), part["number"], part["tokens"]);
        if !markdown.contains(&row) {
            return Err(format!("outline lost part metadata: {row}"));
        }
        previous = Some(title);
    }
    compare_baseline(&env, &reply, &markdown)
}

fn read(env: &Environment, binary: &Path, format: &str) -> Result<String, String> {
    let replay = super::launch::fixtures().join("replies/article/fulltext/synthetic");
    let args = ["get", "article", "28944238", "fulltext", "--format", format].map(str::to_owned);
    let output = env.command(binary, &super::launch::arguments(&args, &env.globals(Some(&replay), false)))
        .output().map_err(|error| error.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err(format!("outline failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn compare_baseline(env: &Environment, reply: &Value, markdown: &str) -> Result<(), String> {
    let Some(binary) = std::env::var_os("BIOMCP_LEGACY_EXECUTABLE") else { return Ok(()); };
    let before: Value = serde_json::from_str(&read(env, Path::new(&binary), "json")?).map_err(|error| error.to_string())?;
    if before["result"] != reply["result"] || before["query"] != reply["query"] {
        return Err("compact outline changed article facts or query".into());
    }
    let old = read(env, Path::new(&binary), "md")?;
    if markdown.len() >= old.len() {
        return Err(format!("outline did not shrink: {} -> {} bytes", old.len(), markdown.len()));
    }
    println!("article outline Markdown bytes: {} -> {} (saved {})", old.len(), markdown.len(), old.len() - markdown.len());
    Ok(())
}
