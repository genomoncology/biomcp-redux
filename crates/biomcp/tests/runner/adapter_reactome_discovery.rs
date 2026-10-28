//! Feature pathway-reactome: discovery drops unusable names and keeps sibling identities.
use super::{Stub, invoke};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    for top in [false, true] {
        for invalid in [json!(""), json!({"bad": true})] {
            let stub = Stub::for_sources(
                &["reactome.org"],
                |_| "application/json",
                move |_| {
                    let cards = json!([
                        {"stId":"R-HSA-5673001", "displayName":invalid, "speciesName":"Homo sapiens",
                         "schemaClass":"Pathway", "stIdVersion":"R-HSA-5673001.1"},
                        {"stId":"R-HSA-5673002", "displayName":"Sibling", "speciesName":"Homo sapiens",
                         "schemaClass":"Pathway", "stIdVersion":"R-HSA-5673002.1"}
                    ]);
                    Ok(if top { cards } else {
                    json!({"results":[{"entries":[
                        {"stId":"R-HSA-5673001", "name":invalid, "species":["Homo sapiens"], "exactType":"Pathway"},
                        {"stId":"R-HSA-5673002", "name":"Sibling", "species":["Homo sapiens"], "exactType":"Pathway"}
                    ]}], "rowCount":2, "numberOfMatches":2})
                }.to_string())
                },
            )?;
            let env = super::super::super::variant::source_environment(&stub)?;
            let args = if top {
                vec!["search", "pathway", "--top-level"]
            } else {
                vec!["search", "pathway", "synthetic", "--source", "reactome"]
            };
            let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
            if value.pointer("/result/items/0/id/id") != Some(&json!("R-HSA-5673002"))
                || value
                    .pointer("/result/items")
                    .and_then(Value::as_array)
                    .map(Vec::len)
                    != Some(1)
                || !value
                    .get("problems")
                    .and_then(Value::as_array)
                    .is_some_and(|p| p.iter().any(|p| p.get("field") == Some(&json!("name"))))
            {
                return Err(format!(
                    "discovery name lost its sibling or disclosure: {value}"
                ));
            }
            stub.finish()?;
        }
    }
    pathway_mapping().map_err(|e| e.to_string())
}
fn pathway_mapping() -> Result<(), Box<dyn std::error::Error>> {
    for invalid in [json!(""), json!({"bad": true})] {
        let value = super::checked_steps(
            "gene/pathways",
            |source, capture| {
                if source != "reactome" {
                    return;
                }
                if let Some(name) = capture.pointer_mut("/exchanges/0/body/value/0/displayName") {
                    *name = invalid.clone();
                }
            },
            json!([{"run":["get", "gene", "BRAF", "pathways"], "exit":0}]),
        )?;
        if !value
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|p| {
                p.iter()
                    .any(|p| p.get("field") == Some(&json!("pathways.name")))
            })
            || value
                .pointer("/result/items/0/sections/pathways/data/items")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(
                format!("pathway mapping lost valid siblings or name problem: {value}").into(),
            );
        }
    }
    Ok(())
}
