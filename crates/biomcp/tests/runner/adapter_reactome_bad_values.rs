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
            let stub = metadata_stub(section, native, invalid)?;
            let env = super::super::super::variant::source_environment(&stub)?;
            let args = if section == "summary" {
                vec!["get", "pathway", "R-HSA-5673001", "summary"]
            } else {
                vec!["pathway", section, "R-HSA-5673001"]
            };
            let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
            check_result(&value, section, native, field)?;
            stub.finish()?;
        }
    }
    Ok(())
}

fn metadata_stub(
    section: &'static str,
    native: &'static str,
    invalid: Value,
) -> Result<Stub, String> {
    Stub::for_sources(
        &["reactome.org"],
        |_| "application/json",
        move |request| {
            let mut card = json!({"stId":"R-HSA-5673001","displayName":"Synthetic pathway",
                "speciesName":"Homo sapiens","schemaClass":"Pathway","stIdVersion":"R-HSA-5673001.1",
                "summation":[{"dbId":4,"text":"Valid summary"},{"dbId":10,"text":"Sibling summary"}],
                "literatureReference":[{"dbId":5,"title":"Valid title","pubMedIdentifier":123,"url":"https://example.org"},
                    {"dbId":11,"title":"Sibling reference"}]});
            let mut event = json!({"dbId":6,"displayName":"Valid event","schemaClass":"Reaction","stIdVersion":"R-HSA-6.1"});
            let mut participant = json!({
                "peDbId":7, "displayName":"Valid participant", "schemaClass":"Complex",
                "refEntities":[{"stId":"UniProt:P31749", "identifier":"P31749", "schemaClass":"ReferenceGeneProduct"}]
            });
            if section == "summary" {
                let pointer = match native {
                    "title" | "pubMedIdentifier" | "url" => {
                        format!("/literatureReference/0/{native}")
                    }
                    "text" => "/summation/0/text".into(),
                    _ => format!("/{native}"),
                };
                *card.pointer_mut(&pointer).ok_or("missing fixture field")? = invalid.clone();
            } else if section == "events" {
                *event.get_mut(native).ok_or("missing event fixture field")? = invalid.clone();
            } else {
                *participant
                    .get_mut(native)
                    .ok_or("missing participant fixture field")? = invalid.clone();
            }
            let body = if request.contains("/database/version") {
                json!(97)
            } else if request.contains("/containedEvents") {
                json!([event, {"dbId":8,"displayName":"Sibling"}])
            } else if request.contains("/participants/") {
                json!([participant, {
                    "peDbId":9,"displayName":"Sibling","schemaClass":"Complex",
                    "refEntities":[{"stId":"UniProt:P31749","identifier":"P31749","schemaClass":"ReferenceGeneProduct"}]
                }])
            } else {
                card
            };
            Ok(body.to_string())
        },
    )
}

fn check_result(value: &Value, section: &str, native: &str, field: &str) -> Result<(), String> {
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
    let absent = match field {
        "version" => Some("/result/pathway/version"),
        "name" => Some("/result/pathway"),
        "summations" => Some("/result/pathway/summations"),
        "references" => Some("/result/pathway/references"),
        "references.title" => Some("/result/pathway/references/0/title"),
        "references.pubmed" => Some("/result/pathway/references/0/pubmed"),
        "references.url" => Some("/result/pathway/references/0/url"),
        "events.name" => Some("/result/items/0/name"),
        "events.schema_class" => Some("/result/items/0/schema_class"),
        "events.version" => Some("/result/items/0/version"),
        _ => None,
    };
    if absent.is_some_and(|p| value.pointer(p).is_some_and(|v| !v.is_null())) {
        return Err(format!("bad {field} remained in reply: {value}"));
    }
    if field == "summations.text"
        && value.pointer("/result/pathway/summations/0/db_id") != Some(&json!(10))
    {
        return Err(format!("bad summary lost its valid sibling: {value}"));
    }
    if section == "genes"
        && (value.pointer("/result/physical_entities") != Some(&json!(2))
            || value.pointer("/result/reference_occurrences") != Some(&json!(2))
            || value.pointer("/result/items/0/physical_context_count") != Some(&json!(1)))
    {
        return Err(format!(
            "bad {field} changed native counts or invented context: {value}"
        ));
    }
    Ok(())
}
