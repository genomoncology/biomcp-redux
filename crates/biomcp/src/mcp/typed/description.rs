//! Source and section provenance comes from the same declarations as the front map.
#[path = "../../../coverage.rs"]
mod coverage;
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
        let node = super::arguments::resolve(&schema["properties"][key], schema);
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
        groups
            .into_iter()
            .map(|(sources, sections)| format!("{}:{sources}", sections.join(",")))
            .collect::<Vec<_>>()
            .join("; ")
    };
    format!(
        "{} Source evidence. {}.",
        schema["description"]
            .as_str()
            .unwrap_or("Retrieve records."),
        provenance
    )
}
