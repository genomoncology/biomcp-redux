//! Source and section provenance comes from the same declarations as the front map.
use crate::help::coverage;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn render(
    verb: &str,
    entity: &str,
    schema: &Value,
    excluded: &BTreeSet<biomcp_types::Source>,
) -> String {
    let sources = coverage::command_sources(verb, entity, schema)
        .into_iter()
        .filter(|word| {
            word.parse::<biomcp_types::Source>()
                .is_ok_and(|source| !excluded.contains(&source))
        })
        .collect::<BTreeSet<_>>();
    let provenance = provenance(verb, entity, schema, &sources, excluded);
    let summary = biomcp_types::COMMANDS
        .iter()
        .find(|row| row.verb == verb && row.entity == entity)
        .map_or("", |row| row.summary.trim());
    let summary = super::catalog::summary(summary, excluded);
    [summary.as_str(), provenance.as_str()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        + "."
}

fn provenance(
    verb: &str,
    entity: &str,
    schema: &Value,
    sources: &BTreeSet<String>,
    excluded: &BTreeSet<biomcp_types::Source>,
) -> String {
    let candidates = sources
        .iter()
        .filter_map(|source| source.parse().ok())
        .collect::<Vec<_>>();
    let mut groups = BTreeMap::<String, Vec<String>>::new();
    for key in ["section", "sections"] {
        let node = super::arguments::resolve(
            schema
                .get("properties")
                .and_then(|fields| fields.get(key))
                .unwrap_or(&Value::Null),
            schema,
        );
        let node = node
            .get("anyOf")
            .and_then(Value::as_array)
            .and_then(|branches| branches.iter().find(|node| node["type"] != "null"))
            .map(|node| super::arguments::resolve(node, schema))
            .unwrap_or(node);
        let node = super::arguments::resolve(node.get("items").unwrap_or(node), schema);
        for section in node
            .get("enum")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .filter(|word| *word != "all")
        {
            let publishers =
                biomcp_types::command_section_sources(verb, entity, section, &candidates)
                    .map(|sources| {
                        sources
                            .into_iter()
                            .filter(|source| !excluded.contains(source))
                            .map(|s| s.to_string())
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_else(|| sources.clone());
            if publishers.is_empty() {
                continue;
            }
            groups
                .entry(publishers.into_iter().collect::<Vec<_>>().join(","))
                .or_default()
                .push(section.into());
        }
    }
    if groups.is_empty() {
        sources.iter().cloned().collect::<Vec<_>>().join(",")
    } else {
        let mut same = Vec::new();
        let mut other = Vec::new();
        for (sources, sections) in groups {
            if sections.len() == 1 && sections.first() == Some(&sources) {
                same.push(sources);
            } else {
                other.push(format!("{}:{sources}", sections.join(",")));
            }
        }
        same.extend(other);
        format!("Sections:{}", same.join(";"))
    }
}

/// What a person would ask the tool for, its sources by full name, one search and one get
/// example, the other actions in a few words, and what the tool is not for.
pub(super) fn tool(
    (_, asks, _, [search, get], not_for): &super::catalog::front_map::EntityTool,
    sources: &BTreeSet<String>,
    actions: &[(&str, &str)],
    excluded: &BTreeSet<biomcp_types::Source>,
) -> String {
    let plain = |text: &str| super::catalog::summary(text, excluded);
    let mut parts = vec![plain(&format!("{asks}."))];
    let names = sources
        .iter()
        .filter_map(|word| word.parse::<biomcp_types::Source>().ok())
        .map(biomcp_query::sources::display_name)
        .collect::<BTreeSet<_>>();
    if !names.is_empty() {
        let names = names.into_iter().collect::<Vec<_>>();
        parts.push(format!("Sources: {}.", names.join(", ")));
    }
    for (label, action, example) in [("Search", "search", search), ("Get", "get", get)] {
        if actions.iter().any(|(name, _)| *name == action) {
            parts.push(format!(
                "{label}: {{\"action\":\"{action}\",\"{action}\":{example}}}."
            ));
        }
    }
    let others = actions
        .iter()
        .filter(|(_, note)| !note.is_empty())
        .map(|(action, note)| format!("{action} ({note})"))
        .collect::<Vec<_>>();
    if !others.is_empty() {
        parts.push(plain(&format!(
            "Other actions: {}.",
            others.join(", ")
        )));
    }
    parts.push(plain(&format!("{not_for}.")));
    parts.join(" ")
}

/// An action's object keeps its command's description; citations says what it lists.
pub(super) fn action(action: &str, description: &str) -> String {
    if action == "citations" {
        format!("{description} Lists citing papers, not counts.")
    } else {
        description.to_owned()
    }
}
