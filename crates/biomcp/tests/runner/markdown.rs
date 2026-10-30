//! The text view must print source Markdown as literal data once; JSON remains the MCP contract.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![transcript_trial(
        "article-search/markdown-view-preserves-source-text",
        || check().map_err(Into::into),
    )];
    trials.extend(super::source_text::trials());
    trials
}
fn check() -> Result<(), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::launch::fixtures().join("replies/article/fulltext/synthetic");
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
        || text
            .matches(&super::source_terms::text::escape(markdown))
            .count()
            != 1
    {
        return Err("Markdown view changed or duplicated source text".into());
    }
    Ok(())
}
