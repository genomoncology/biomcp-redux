//! Closed parser edge cases reject undeclared namespace and selector shapes.
use super::{Selection, Step, projection::Projection, request};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        super::operation::trial(),
        Trial::test("parity/closed-projection-shapes", || {
            projections().map_err(Into::into)
        }),
        Trial::test("parity/effective-limit-selector", || {
            selectors().map_err(Into::into)
        }),
    ]
}
fn project(shape: Value, value: Value) -> Result<Value, String> {
    let projection: Projection = serde_json::from_value(shape).map_err(|e| e.to_string())?;
    projection.select(&value)
}
fn projections() -> Result<(), String> {
    let source = json!({"released_source":"/source"});
    for value in [
        json!(false),
        json!("clingen"),
        json!("CLINGEN"),
        json!("erepo"),
    ] {
        if project(source.clone(), json!({"source":value})).is_ok() {
            return Err("undeclared released provider accepted".into());
        }
    }
    let object = json!({"object":{
        "source":{"released_source":"/sources/0"},
        "state":{"pointer":"/outcome"}
    }});
    for value in [
        json!({"sources":[false],"outcome":"empty"}),
        json!({"sources":["ClinGen"],"outcome":[]}),
    ] {
        if project(object.clone(), value).is_ok() {
            return Err("nonstring provider or outcome accepted".into());
        }
    }
    if project(
        json!({"object":{"id":{"released_source":"/provider"}}}),
        json!({"provider":"ClinGen"}),
    )
    .is_ok()
    {
        return Err("released provider manufactured an identifier member".into());
    }
    let author = json!({"author_ids":"/ids/*"});
    for value in [
        "semantic-scholar:123",
        "semanticscholar:",
        "semanticscholar:1x",
        "prefix:semanticscholar:123",
        "semanticscholar:+123",
        "semanticscholar:１２３",
    ] {
        if project(author.clone(), json!({"ids":[value]})).is_ok() {
            return Err("undeclared author namespace accepted".into());
        }
    }
    for shape in [
        json!({"author_ids":"/id","prefix":"x"}),
        json!({"released_source":"/source","expected":"clingen"}),
    ] {
        if serde_json::from_value::<Projection>(shape).is_ok() {
            return Err("projection accepted an undeclared transformation".into());
        }
    }
    Ok(())
}
fn selectors() -> Result<(), String> {
    let plan: Value = serde_json::from_slice(
        &std::fs::read(super::super::launch::fixtures().join("parity/sample.json"))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let template = plan.pointer("/cases/0").ok_or("sample selection missing")?;
    for (args, baseline, binding, valid) in [
        (vec!["search", "adverse-event", "aspirin"], "5", true, true),
        (
            vec!["search", "adverse-event", "aspirin"],
            "10",
            true,
            false,
        ),
        (
            vec!["search", "adverse-event", "aspirin", "--limit", "3"],
            "3",
            true,
            true,
        ),
        (
            vec!["search", "adverse-event", "aspirin", "--limit", "3"],
            "5",
            true,
            false,
        ),
        (
            vec!["search", "adverse-event", "aspirin", "--limit", "0"],
            "0",
            true,
            false,
        ),
        (vec!["search", "gene", "BRAF"], "5", true, false),
        (
            vec!["search", "adverse-event", "aspirin"],
            "5",
            false,
            false,
        ),
    ] {
        let mut row = template.clone();
        let fields = row.as_object_mut().ok_or("selection must be an object")?;
        fields.insert(
            "baseline_command".into(),
            json!(["search", "adverse-event", "aspirin", "--limit", baseline]),
        );
        fields.insert(
            "command_selectors".into(),
            json!({"--limit":{"effective_adverse_event_limit":binding}}),
        );
        let selection: Selection = serde_json::from_value(row).map_err(|e| e.to_string())?;
        let step: Step = serde_json::from_value(json!({"run":args})).map_err(|e| e.to_string())?;
        if request::validate_command(&selection, &step).is_ok() != valid {
            return Err(format!(
                "incorrect effective limit selector verdict for {args:?}"
            ));
        }
    }
    Ok(())
}
