//! Real CC BY papers prove source structure through the public command.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("article-search/fulltext/source-subsection-markdown", || {
            subsection().map_err(Into::into)
        }),
        transcript_trial("article-search/fulltext/source-table-markdown", || {
            table().map_err(Into::into)
        }),
        transcript_trial("article-search/fulltext/abstract-markdown", || {
            abstract_part().map_err(Into::into)
        }),
    ]
}
fn run(replay: &str, args: &[&str], max_bytes: usize) -> Result<Value, String> {
    serde_json::from_slice(&output(replay, args, max_bytes)?).map_err(|e| e.to_string())
}
fn output(replay: &str, args: &[&str], max_bytes: usize) -> Result<Vec<u8>, String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::launch::fixtures().join("replies").join(replay);
    let mut command = environment.globals(Some(&replay), false);
    command.extend(args.iter().map(|s| (*s).to_owned()));
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &command)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() || super::source_terms::raw_size(&output.stdout)? > max_bytes {
        return Err(format!(
            "command failed or exceeded {max_bytes} bytes: status {}, {} bytes",
            output.status,
            output.stdout.len()
        ));
    }
    Ok(output.stdout)
}
fn contains(value: &Value, pointer: &str, required: &[&str]) -> Result<(), String> {
    let text = value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or("missing source Markdown")?;
    for required in required {
        if !text.contains(required) {
            return Err(format!("source Markdown missing {required}"));
        }
    }
    Ok(())
}
fn subsection() -> Result<(), String> {
    let value = run(
        "article/fulltext/live-results",
        &["article", "text", "35440569", "--part", "3"],
        17000,
    )?;
    if value.pointer("/result/content/data/heading_path")
        != Some(&json!([
            "Results",
            "Identification of candidate driver mutations in AKT1, AKT2, and AKT3"
        ]))
    {
        return Err("source heading ancestry changed".into());
    }
    contains(
        &value,
        "/result/content/data/markdown",
        &["### Identification", "*AKT1*", "[11]"],
    )
}
fn table() -> Result<(), String> {
    let value = run(
        "article/fulltext/live-introduction",
        &["article", "text", "28944238", "--part", "14"],
        6000,
    )?;
    contains(
        &value,
        "/result/content/data/markdown",
        &[
            "**Table 1.** Sample characteristics\n",
            "| Ontario |",
            "Mayo Clinic",
            "Age at diagnosis of cases (range)",
        ],
    )?;
    let found = run(
        "article/fulltext/live-introduction",
        &["article", "text", "28944238", "--find", "Age at diagnosis"],
        2000,
    )?;
    if found.pointer("/result/content/data/hits/0/part") != Some(&json!(14)) {
        return Err("table match lost its part identity".into());
    }
    contains(
        &found,
        "/result/content/data/hits/0/markdown",
        &[
            "**Table 1.** Sample characteristics",
            "Ontario",
            "Mayo Clinic",
            "| --- |",
            "Age at diagnosis of cases (range)",
        ],
    )?;
    let markdown = found
        .pointer("/result/content/data/hits/0/markdown")
        .and_then(Value::as_str)
        .ok_or("missing match")?;
    if markdown
        .lines()
        .filter(|line| line.starts_with('|'))
        .count()
        != 3
    {
        return Err("table find included unrelated rows".into());
    }
    Ok(())
}
fn abstract_part() -> Result<(), String> {
    let italic = run(
        "article/get/synthetic-derived",
        &["get", "article", "28944238"],
        8000,
    )?;
    contains(
        &italic,
        "/result/items/0/answer/card/abstract_parts/2/text",
        &["*APC*", "*MLH1*", "*MUTYH*"],
    )?;
    let value = run(
        "article/get/synthetic-derived",
        &["get", "article", "35440569"],
        4000,
    )?;
    let parts = value
        .pointer("/result/items/0/answer/card/abstract_parts")
        .and_then(Value::as_array)
        .ok_or("missing abstract parts")?;
    if !parts.iter().any(|part| {
        part.get("text")
            .and_then(Value::as_str)
            .is_some_and(|text| text.contains("metabolism<sup>1-8</sup>"))
    }) {
        return Err("abstract superscript was stripped".into());
    }
    let view = output(
        "article/get/synthetic-derived",
        &["get", "article", "35440569", "--format", "md"],
        4000,
    )?;
    let view = String::from_utf8(view).map_err(|e| e.to_string())?;
    for part in parts {
        let text = part
            .get("text")
            .and_then(Value::as_str)
            .ok_or("missing part text")?;
        if view
            .matches(&super::source_terms::text::escape(text))
            .count()
            != 1
        {
            return Err("abstract source text was changed or duplicated".into());
        }
    }
    Ok(())
}
