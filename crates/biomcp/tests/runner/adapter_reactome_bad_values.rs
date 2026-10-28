//! Feature pathway-reactome: malformed metadata leaves valid sibling records readable.
use super::{Stub, invoke};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    for (section, native, field) in [
        ("summary", "stIdVersion", "version"),
        ("summary", "displayName", "name"),
        ("summary", "summation", "summations"),
        ("summary", "literatureReference", "references"),
        ("summary", "title", "references.title"),
        ("summary", "pubMedIdentifier", "references.pubmed"),
        ("summary", "url", "references.url"),
        ("summary", "text", "summations.text"),
        ("events", "displayName", "events.name"),
        ("events", "schemaClass", "events.schema_class"),
        ("events", "stIdVersion", "events.version"),
        ("genes", "displayName", "participants.name"),
        ("genes", "schemaClass", "participants.schema_class"),
    ] {
        for invalid in [json!({"bad":true}), json!("")] {
            let stub = Stub::for_sources(
                &["reactome.org"],
                |_| "application/json",
                move |request| {
                    let mut card = json!({"stId":"R-HSA-5673001","displayName":"Synthetic pathway",
                "speciesName":"Homo sapiens","schemaClass":"Pathway","stIdVersion":"R-HSA-5673001.1",
                "summation":[{"dbId":4,"text":"Valid summary"}],
                "literatureReference":[{"dbId":5,"title":"Valid title","pubMedIdentifier":123,"url":"https://example.org"}]});
                    let mut event = json!({"dbId":6,"displayName":"Valid event","schemaClass":"Reaction","stIdVersion":"R-HSA-6.1"});
                    let mut participant = json!({"peDbId":7,"displayName":"Valid participant","schemaClass":"Complex","refEntities":[{"stId":"UniProt:P31749","identifier":"P31749","schemaClass":"ReferenceGeneProduct"}]});
                    if section == "summary" {
                        let pointer = match native {
                            "title" | "pubMedIdentifier" | "url" => {
                                format!("/literatureReference/0/{native}")
                            }
                            "text" => "/summation/0/text".into(),
                            _ => format!("/{native}"),
                        };
                        *card.pointer_mut(&pointer).ok_or("missing fixture field")? =
                            invalid.clone();
                    } else if section == "events" {
                        event[native] = invalid.clone();
                    } else {
                        participant[native] = invalid.clone();
                    }
                    Ok(if request.contains("/database/version") { json!(97) }
                else if request.contains("/containedEvents") { json!([event, {"dbId":8,"displayName":"Sibling"}]) }
                else if request.contains("/participants/") { json!([participant, {"peDbId":9,"displayName":"Sibling","schemaClass":"Complex","refEntities":[{"stId":"UniProt:P31749","identifier":"P31749","schemaClass":"ReferenceGeneProduct"}]}]) }
                else { card }.to_string())
                },
            )?;
            let env = super::super::super::variant::source_environment(&stub)?;
            let args = if section == "summary" {
                vec!["get", "pathway", "R-HSA-5673001", "summary"]
            } else {
                vec!["pathway", section, "R-HSA-5673001"]
            };
            let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
            if !value
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|p| p.iter().any(|p| p.get("field") == Some(&json!(field))))
            {
                return Err(format!("bad {field} lost disclosure: {value}"));
            }
            if section == "summary"
                && native != "displayName"
                && value.pointer("/result/pathway/id/id") != Some(&json!("R-HSA-5673001"))
            {
                return Err(format!("bad {field} lost pathway: {value}"));
            }
            if section != "summary"
                && value
                    .pointer("/result/items")
                    .and_then(Value::as_array)
                    .is_none_or(Vec::is_empty)
            {
                return Err(format!("bad {field} lost valid sibling: {value}"));
            }
            stub.finish()?;
        }
    }
    Ok(())
}
