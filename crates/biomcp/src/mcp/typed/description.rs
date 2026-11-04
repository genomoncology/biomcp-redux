//! Source and section provenance comes from the same declarations as the front map.
use crate::help::coverage;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn render(verb: &str, entity: &str, schema: &Value) -> String {
    let sources = coverage::command_sources(verb, entity, schema);
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
                            .map(|s| s.to_string())
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_else(|| sources.clone());
            groups
                .entry(publishers.into_iter().collect::<Vec<_>>().join(","))
                .or_default()
                .push(section.into());
        }
    }
    let provenance = if groups.is_empty() {
        sources.into_iter().collect::<Vec<_>>().join(",")
    } else {
        let mut same = Vec::new();
        let mut other = Vec::new();
        for (sources, sections) in groups {
            if sections.len() == 1 && sections.first() == Some(&sources) {
                same.push(sources);
            } else {
                other.push(format!("{}({sources})", sections.join(",")));
            }
        }
        same.extend(other);
        format!("Sections=source: {}", same.join(";"))
    };
    let prefix = format!("{verb} {entity} source records. ");
    let input = input_description(schema);
    let budget = 300_usize.saturating_sub(prefix.chars().count() + provenance.chars().count() + 2);
    let input = input.chars().take(budget).collect::<String>();
    format!("{prefix}{input} {provenance}.")
}

fn input_description(schema: &Value) -> String {
    let input = schema["properties"]
        .as_object()
        .into_iter()
        .flatten()
        .find(|(_, node)| node.get("x-positional").is_some())
        .and_then(|(_, node)| node.get("description"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .split(". ")
        .next()
        .unwrap_or_default();
    input
        .replace("identifiers", "IDs")
        .replace(" accession", "")
        .replace("reference-qualified ", "")
        .replace("VCF-style genomic coordinates", "VCF")
        .replace(", in input order", "")
        .replace(", in caller order", "")
}
