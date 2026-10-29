//! Required publisher provenance is measured separately from unchanged payload caps.
#[path = "../../src/view/escape.rs"]
pub(super) mod text;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "source-controls/stored-outcomes-use-current-registry-terms",
            || stored_terms().map_err(Into::into),
        ),
        transcript_trial("source-controls/outcomes-retain-registry-terms", || {
            envelopes().map_err(Into::into)
        }),
        transcript_trial(
            "schema-discovery/markdown-terms-subtraction-keeps-payload-ceilings",
            || markdown_subtraction().map_err(Into::into),
        ),
        transcript_trial("schema-discovery/terms-subtraction-keeps-payload-ceilings", || {
            subtraction().map_err(Into::into)
        }),
    ]
}

fn strip_row(row: &mut Value) -> Result<(), String> {
    let outcome: biomcp_types::SourceOutcome =
        serde_json::from_value(row.clone()).map_err(|e| e.to_string())?;
    let source = outcome.source;
    let terms = biomcp_adapters::registry::terms(source).ok_or("unregistered outcome source")?;
    let object = row
        .as_object_mut()
        .ok_or("source outcome is not an object")?;
    for (field, expected) in [
        ("use", json!(terms.usage)),
        ("terms", json!(terms.name)),
        ("terms_url", json!(terms.url)),
    ] {
        if object.remove(field) != Some(expected) {
            return Err(format!("{source}: missing or changed registry {field}"));
        }
    }
    Ok(())
}

/// Validate every source outcome and remove exactly its three new registry fields.
pub(super) fn comparable(value: &Value, pointer: &str) -> Result<Value, String> {
    let mut result = value.clone();
    if pointer.is_empty() {
        if let Some(rows) = result.get_mut("sources").and_then(Value::as_array_mut) {
            for row in rows {
                strip_row(row)?;
            }
        }
    } else if pointer == "/sources" {
        for row in result.as_array_mut().ok_or("sources is not an array")? {
            strip_row(row)?;
        }
    } else if pointer
        .strip_prefix("/sources/")
        .is_some_and(|p| p.parse::<usize>().is_ok())
    {
        strip_row(&mut result)?;
    }
    Ok(result)
}

pub(super) fn size(value: &Value, pointer: &str) -> Result<usize, String> {
    serde_json::to_vec(&comparable(value, pointer)?)
        .map(|b| b.len())
        .map_err(|e| e.to_string())
}

pub(super) fn raw_size(bytes: &[u8]) -> Result<usize, String> {
    let Ok(value) = serde_json::from_slice::<Value>(bytes) else {
        return Ok(bytes.len());
    };
    let compact = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    Ok(bytes.len() - (compact - size(&value, "")?))
}

fn envelopes() -> Result<(), String> {
    let registry = super::schema::Registry::embedded()?;
    let root = crate::runner::launch::fixtures().join("cases");
    for case in [
        "gene/braf.yaml",
        "article/get/card.yaml",
        "trial/condition.yaml",
        "gene/failure-500.yaml",
    ] {
        let value = super::run_case(&root.join(case), &registry)?;
        let rows = value
            .get("sources")
            .and_then(Value::as_array)
            .ok_or("missing source outcomes")?;
        if rows.is_empty() {
            return Err("no source outcomes tested".into());
        }
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let payload = size(&value, "")?;
        use std::io::Write;
        writeln!(
            std::io::stdout(),
            "{case}: actual={full}, new_terms={}, comparable={payload}",
            full - payload
        )
        .map_err(|e| e.to_string())?;
    }
    let schema: Value =
        serde_json::from_str(include_str!("../../../biomcp-types/schemas/source.json"))
            .map_err(|e| e.to_string())?;
    for name in schema
        .get("x-extensible-enum")
        .and_then(Value::as_array)
        .ok_or("missing source enum")?
    {
        let source = serde_json::from_value(name.clone()).map_err(|e| e.to_string())?;
        let outcome =
            biomcp_adapters::registry::outcome(source, biomcp_types::SourceState::Ok, None);
        let mut value = serde_json::to_value(outcome).map_err(|e| e.to_string())?;
        strip_row(&mut value)?;
    }
    Ok(())
}

fn subtraction() -> Result<(), String> {
    let outcome = biomcp_adapters::registry::outcome(
        biomcp_types::Source::Hgnc,
        biomcp_types::SourceState::Ok,
        None,
    );
    let mut envelope = json!({"sources":[outcome], "result":{"value":1}});
    let cap = size(&envelope, "")?;
    let stdout = serde_json::to_vec(&envelope).map_err(|e| e.to_string())?;
    for (limit, amount) in [("max_bytes", cap), ("max_tokens", cap.div_ceil(4))] {
        for full in [false, true] {
            let step: super::Step = serde_json::from_value(json!({
                "run": [], "full_output_bytes": full, limit: amount,
            }))
            .map_err(|e| e.to_string())?;
            let result = super::assertions::check(&stdout, &step, &Vec::new());
            if result.is_err() != full {
                return Err(format!(
                    "{limit}: complete-output basis did not count policy fields"
                ));
            }
        }
    }
    envelope
        .get_mut("result")
        .and_then(Value::as_object_mut)
        .ok_or("missing result")?
        .insert("added".into(), json!("ordinary payload"));
    if size(&envelope, "")? <= cap {
        return Err("ordinary payload earned a terms deduction".into());
    }
    envelope
        .pointer_mut("/sources/0")
        .and_then(Value::as_object_mut)
        .ok_or("missing source")?
        .insert("terms".into(), json!("wrong terms"));
    if size(&envelope, "").is_ok() {
        return Err("malformed terms earned a deduction".into());
    }
    envelope
        .pointer_mut("/sources/0")
        .and_then(Value::as_object_mut)
        .ok_or("missing row")?
        .remove("terms");
    if size(&envelope, "").is_ok() {
        return Err("missing terms earned a deduction".into());
    }
    Ok(())
}

/// Measure only new publisher columns in the source table of the paired JSON view.
pub(super) fn markdown_bytes(text: &str, value: &Value) -> Result<usize, String> {
    comparable(value, "")?;
    let Some(rows) = value
        .get("sources")
        .and_then(Value::as_array)
        .filter(|r| !r.is_empty())
    else {
        return Ok(0);
    };
    let table = text
        .split_once("## Sources\n\n")
        .ok_or("missing source table")?
        .1;
    if !table.starts_with('|') {
        return markdown_object_bytes(table, rows);
    }
    let lines: Vec<_> = table
        .lines()
        .take_while(|line| line.starts_with('|'))
        .collect();
    if lines.len() != rows.len() + 2 {
        return Err("source table lost outcome rows".into());
    }
    let columns: Vec<_> = lines
        .first()
        .ok_or("missing source columns")?
        .split('|')
        .enumerate()
        .filter_map(|(i, cell)| match cell {
            " Terms " => Some((i, "terms")),
            " Terms url " => Some((i, "terms_url")),
            " Use " => Some((i, "use")),
            _ => None,
        })
        .collect();
    if columns.len() != 3
        || columns
            .iter()
            .map(|(_, field)| *field)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != 3
    {
        return Err("source table lost required publisher columns".into());
    }
    let mut bytes = 0;
    for (number, line) in lines.iter().enumerate() {
        let cells: Vec<_> = line.split('|').collect();
        for (column, field) in &columns {
            let cell = cells.get(*column).ok_or("missing publisher cell")?;
            if number == 1 && *cell != " --- " {
                return Err("changed source separator".into());
            }
            if number > 1 {
                let expected = rows
                    .get(number - 2)
                    .and_then(|r| r.get(*field))
                    .and_then(Value::as_str)
                    .ok_or("missing publisher value")?;
                if *cell
                    != format!(
                        " {} ",
                        if *field == "terms_url" {
                            text::source_link(expected)
                        } else {
                            text::escape(expected)
                        }
                    )
                {
                    return Err("changed Markdown publisher terms".into());
                }
            }
            bytes += cell.len() + 1;
        }
    }
    Ok(bytes)
}
fn markdown_object_bytes(table: &str, rows: &[Value]) -> Result<usize, String> {
    let mut rest = table;
    let mut bytes = 0;
    for row in rows {
        for field in ["terms", "terms_url", "use"] {
            let value = row
                .get(field)
                .and_then(Value::as_str)
                .ok_or("missing publisher value")?;
            let rendered = if field == "terms_url" {
                text::source_link(value)
            } else {
                text::escape(value)
            };
            let line = format!("**{}:** {rendered}\n\n", text::escape(field));
            let (before, after) = rest
                .split_once(&line)
                .ok_or("changed Markdown publisher terms")?;
            if before.contains("**use:**") {
                return Err("source objects lost publisher fields".into());
            }
            bytes += line.len();
            rest = after;
        }
    }
    Ok(bytes)
}
fn markdown_subtraction() -> Result<(), String> {
    let outcome = biomcp_adapters::registry::outcome(
        biomcp_types::Source::Hgnc,
        biomcp_types::SourceState::Ok,
        None,
    );
    let text = format!(
        "## Sources\n\n| Source | State | Terms | Terms url | Use |\n| --- | --- | --- | --- | --- |\n| hgnc | ok | {} | {} | open |\n\n",
        text::escape(&outcome.terms),
        text::source_link(&outcome.terms_url)
    );
    let value = json!({"sources":[outcome]});
    let expected = "## Sources\n\n| Source | State |\n| --- | --- |\n| hgnc | ok |\n\n";
    if text.len() - markdown_bytes(&text, &value)? != expected.len() {
        return Err("Markdown changed prior table bytes".into());
    }
    let added = format!("{text}ordinary added content");
    if added.len() - markdown_bytes(&added, &value)? <= expected.len() {
        return Err("ordinary Markdown earned a deduction".into());
    }
    for changed in [
        text.replace(&text::escape("CC0 1.0"), "wrong terms"),
        text.replace(" Terms ", " Missing "),
    ] {
        if markdown_bytes(&changed, &value).is_ok() {
            return Err("invalid Markdown terms earned a deduction".into());
        }
    }
    Ok(())
}

fn stored_terms() -> Result<(), String> {
    use super::{
        configuration::{cached, invoke},
        launch::{Environment, fixtures},
    };
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("stored-terms");
    let fixture = fixtures().join("replies/article/search/synthetic-small");
    let args = ["search", "article", "AKT1 E17K", "--limit", "2"];
    let first = invoke(&env, cached(&args, &case, Some(&fixture)), 0)?;
    let connection =
        rusqlite::Connection::open(case.join("replay.sqlite")).map_err(|e| e.to_string())?;
    let saved: String = connection
        .query_row("SELECT outcomes FROM pool LIMIT 1", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let mut stale: Value = serde_json::from_str(&saved).map_err(|e| e.to_string())?;
    for row in stale.as_array_mut().ok_or("missing saved outcomes")? {
        let row = row.as_object_mut().ok_or("invalid saved outcome")?;
        row.insert("use".into(), json!("noncommercial"));
        row.insert("terms".into(), json!("obsolete source terms"));
        row.insert(
            "terms_url".into(),
            json!("https://invalid.example/obsolete"),
        );
    }
    connection
        .execute(
            "UPDATE pool SET outcomes=?1",
            [serde_json::to_string(&stale).map_err(|e| e.to_string())?],
        )
        .map_err(|e| e.to_string())?;
    drop(connection);
    let warm = invoke(&env, cached(&args, &case, None), 0)?;
    if first.pointer("/page/pool/id") != warm.pointer("/page/pool/id") {
        return Err("stored source pool was not reused".into());
    }
    comparable(&warm, "")?;
    Ok(())
}
