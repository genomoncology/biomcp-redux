//! Source and section provenance comes from the same declarations as the front map.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn render(
    verb: &str,
    entity: &str,
    schema: &Value,
    sources: &BTreeSet<String>,
    excluded: &BTreeSet<biomcp_types::Source>,
) -> String {
    let provenance = provenance(verb, entity, schema, sources, excluded);
    let summary = biomcp_types::COMMANDS
        .iter()
        .find(|row| row.verb == verb && row.entity == entity)
        .map_or("", |row| row.summary.trim());
    let summary = super::catalog::summary(summary, excluded);
    if provenance.is_empty() {
        summary
    } else {
        format!("{summary} {provenance}.")
    }
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
        // Command help lists publishers. Keep section mappings here because section
        // names are open strings in typed discovery.
        String::new()
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

/// What a person would ask the tool for, differences in search and get source coverage,
/// examples and routing guidance. Action objects and help describe the individual actions.
pub(super) fn tool(
    (name, asks, _, [search, get], not_for): &super::catalog::front_map::EntityTool,
    sources: &BTreeSet<String>,
    source_scope: Option<(&BTreeSet<String>, &BTreeSet<String>)>,
    actions: &[&str],
    excluded: &BTreeSet<biomcp_types::Source>,
) -> String {
    let plain = |text: &str| super::catalog::summary(text, excluded);
    let mut parts = vec![plain(&format!("{asks}."))];
    if let Some((search, get)) = source_scope {
        let names = sources
            .iter()
            .filter_map(|word| {
                let source = word.parse().ok()?;
                let name = biomcp_query::sources::display_name(source);
                Some(match (search.contains(word), get.contains(word)) {
                    (true, false) => format!("{name} (search)"),
                    (false, true) => format!("{name} (get)"),
                    _ => name.to_owned(),
                })
            })
            .collect::<BTreeSet<_>>();
        if !names.is_empty() {
            let names = names.into_iter().collect::<Vec<_>>();
            parts.push(format!("Sources: {}.", names.join(", ")));
        }
    }
    for (action, example) in [("search", search), ("get", get)] {
        if actions.contains(&action) {
            parts.push(format!(
                "{{\"action\":\"{action}\",\"{action}\":{example}}}."
            ));
        }
    }
    for route in crate::help::source_routes::ROUTES
        .iter()
        .filter(|route| route.entity == *name && sources.contains(route.source))
    {
        parts.push(route.typed.into());
    }
    parts.push(plain(&format!("{not_for}.")));
    if *name == "drug" && sources.contains("fda-dsc") {
        parts.push(concat!(
            "For fda-dsc get, use the communication UUID from search. ",
            "Zero matches does not establish that FDA issued no communication about this drug. ",
            "The reply counts communications searched and those with page text. ",
            "FDA index https://www.fda.gov/drugs/drug-safety-and-availability/drug-safety-communications."
        ).into());
    }
    if *name == "diagnostic" && sources.contains("fda-cdx") {
        parts.push(crate::help::CDX_SHARED_FIELDS.trim_start().into());
    }
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

/// Repeated placements use the same arguments as the primary action. Continuation
/// tokens come from next calls. Command help retains the full argument descriptions.
pub(super) fn compact_arguments(input: &mut Value, secondary: bool) {
    if let Some(properties) = input.get_mut("properties").and_then(Value::as_object_mut) {
        for (name, property) in properties {
            if (secondary || matches!(name.as_str(), "pool" | "snapshot"))
                && let Some(fields) = property.as_object_mut()
            {
                fields.shift_remove("description");
            }
        }
    }
}
