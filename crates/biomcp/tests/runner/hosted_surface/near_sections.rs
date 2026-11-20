//! A hosted refusal of a word near a section lists and suggests only sections whose sources
//! are on, and always keeps a next step that works.
use super::{Check, query, server};
use serde_json::{Value, json};

/// Gene sections answered only by the sources this server turns off.
const OFF: [&str; 4] = ["pathways", "clinvar", "constraint", "expression"];

pub(super) fn check(mode: &str) -> Check {
    let (server, _config) = server(
        mode,
        &["reactome", "clinvar", "gnomad", "hpa"],
        "replies/gene/main",
    )?;
    for (words, input) in [
        (
            vec!["get", "gene", "BRAF", "pathwys"],
            json!({"id":"BRAF","section":"pathwys"}),
        ),
        (
            vec!["get", "gene", "BRAF", "clin-var"],
            json!({"id":"BRAF","section":"clin-var"}),
        ),
    ] {
        let reply = query(&server, mode, "get_gene", &words, input)?;
        refused(&reply)?;
    }
    Ok(())
}

fn refused(reply: &Value) -> Check {
    if reply.pointer("/error/code") != Some(&json!("usage.arguments")) {
        return Err(format!("near-section word not refused: {reply}"));
    }
    let listed = reply
        .pointer("/error/valid_values")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str);
    if let Some(section) = listed.into_iter().find(|section| OFF.contains(section)) {
        return Err(format!(
            "refusal lists {section}, whose source is off: {reply}"
        ));
    }
    let next = reply
        .get("next")
        .and_then(Value::as_array)
        .filter(|steps| !steps.is_empty())
        .ok_or_else(|| format!("refusal left no next step: {reply}"))?;
    let text = Value::Array(next.clone()).to_string();
    if let Some(section) = OFF
        .iter()
        .find(|section| text.contains(&format!("\"{section}\"")))
    {
        return Err(format!(
            "next step names {section}, whose source is off: {reply}"
        ));
    }
    Ok(())
}
