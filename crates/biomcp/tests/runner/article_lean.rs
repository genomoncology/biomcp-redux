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
    super::response_cases::copy_capture("article/fulltext/synthetic", &env.root().join("outline-replay"), |_, record| {
        for exchange in record["exchanges"].as_array_mut().into_iter().flatten() {
            if let Some(value) = exchange.pointer_mut("/body/value")
                && let Some(xml) = value.as_str().filter(|xml| xml.starts_with("<article>")) {
                *value = Value::String(xml.replace("</body>", "<sec><title>Introduction</title><p>Later introduction.</p></sec><sec><title>Other section</title><p>Other text.</p></sec><sec><title>Introduction</title><p>Final introduction.</p></sec></body>"));
            }
        }
    }).map_err(|error| error.to_string())?;
    let binary = Path::new(env!("CARGO_BIN_EXE_biomcp"));
    let json = read(&env, binary, "json", false)?;
    let reply: Value = serde_json::from_str(&json).map_err(|error| error.to_string())?;
    let markdown = read(&env, binary, "md", false)?;
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
    outline(&markdown, parts)?;
    let disabled: Value = serde_json::from_str(&read(&env, binary, "json", true)?).map_err(|error| error.to_string())?;
    if disabled["next"].as_array().is_some_and(|steps| steps.iter().any(|step| step["args"] == find)) {
        return Err("disabled fulltext source retained a find suggestion".into());
    }
    compare_baseline(&env, &reply, &markdown)
}

fn read(env: &Environment, binary: &Path, format: &str, disabled: bool) -> Result<String, String> {
    let replay = env.root().join("outline-replay");
    let mut args = ["get", "article", "28944238", "fulltext", "--format", format].map(str::to_owned).to_vec();
    if disabled { args.extend(["--disable-sources".into(), "pmc-oa".into()]); }
    let output = env.command(binary, &super::launch::arguments(&args, &env.globals(Some(&replay), false)))
        .output().map_err(|error| error.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err(format!("outline failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn outline(markdown: &str, parts: &[Value]) -> Result<(), String> {
    let mut current = "";
    let mut rows = Vec::new();
    for line in markdown.lines() {
        if let Some(title) = line.strip_prefix("###### ") {
            current = title;
        } else if line.starts_with("| ") {
            let cells = line.split('|').skip(1).map(str::trim).collect::<Vec<_>>();
            if cells.len() == 4 && matches!(cells[0], "section" | "table") {
                rows.push((current.to_owned(), cells[0].to_owned(), cells[1].to_owned(), cells[2].to_owned()));
            }
        }
    }
    let expected = parts.iter().map(|part| (
        part["title"].as_str().unwrap_or("Parts without a source title").to_owned(),
        part["kind"].as_str().unwrap_or_default().to_owned(),
        part["number"].to_string(), part["tokens"].to_string(),
    )).collect::<Vec<_>>();
    if rows != expected { return Err(format!("outline title-to-part association changed: {rows:?} != {expected:?}")); }
    Ok(())
}

fn compare_baseline(env: &Environment, reply: &Value, markdown: &str) -> Result<(), String> {
    let Some(binary) = std::env::var_os("BIOMCP_LEGACY_EXECUTABLE") else { return Ok(()); };
    let before: Value = serde_json::from_str(&read(env, Path::new(&binary), "json", false)?).map_err(|error| error.to_string())?;
    if before["result"] != reply["result"] || before["query"] != reply["query"] {
        return Err("compact outline changed article facts or query".into());
    }
    let old = read(env, Path::new(&binary), "md", false)?;
    if markdown.len() >= old.len() {
        return Err(format!("outline did not shrink: {} -> {} bytes", old.len(), markdown.len()));
    }
    println!("article outline Markdown bytes: {} -> {} (saved {})", old.len(), markdown.len(), old.len() - markdown.len());
    Ok(())
}
