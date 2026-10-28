//! Features drug-mychem, disease-ols and disease-mydisease: malformed optional fields retain usable replies.
use crate::runner::{configuration::invoke, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
#[path = "adapter_mychem_ontology_cases.rs"]
mod edge_cases;

pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = ["mychem", "ols", "mydisease", "mydisease-doid"]
        .into_iter()
        .map(|family| {
            let feature = if family == "mychem" {
                "drug-mychem"
            } else if family == "ols" {
                "disease-ols"
            } else {
                "disease-mydisease"
            };
            Trial::test(
                format!("{feature}/bad-values-keep-reply-{family}"),
                move || {
                    check(family)
                        .and_then(|_| {
                            if matches!(family, "mychem" | "ols") {
                                search(family)
                            } else {
                                Ok(())
                            }
                        })
                        .map_err(Into::into)
                },
            )
        })
        .collect();
    for family in ["ols", "mydisease"] {
        let case = if family == "ols" {
            (
                "obo_xref",
                json!([{"database":"DOID", "id":" 12"}]),
                "xref.id",
            )
        } else {
            ("xrefs", json!({"doid":[" DOID:3910"]}), "mondo.xrefs.doid")
        };
        trials.push(Trial::test(
            format!("disease-{family}/padded-native-reference-id-is-disclosed"),
            move || check_cases(family, vec![case.clone()]).map_err(Into::into),
        ));
    }
    trials.push(Trial::test(
        "drug-mychem/malformed-component-member-keeps-siblings",
        || {
            check_cases(
                "mychem",
                vec![(
                    "drugbank",
                    json!([{"name":"valid", "id":"DB00001"},42]),
                    "drugbank",
                )],
            )
            .map_err(Into::into)
        },
    ));
    trials.push(Trial::test(
        "trial-search/invalid-resolver-label-stops-name-request",
        || missing_trial_label().map_err(Into::into),
    ));
    trials
}
fn check(family: &str) -> Result<(), String> {
    check_cases(family, edge_cases::cases(family))
}
fn check_cases(family: &str, cases: Vec<(&str, Value, &str)>) -> Result<(), String> {
    for (field, invalid, expected) in cases {
        let (host, mut body, pointer) = fixture(family);
        replace_field(family, field, &invalid, &mut body, pointer)?;
        let keeps_valid = has_string(&invalid, "valid");
        let stub = Stub::for_source_status(
            host,
            "application/json",
            move |_| Ok(body.to_string()),
            |_| 200,
        )?;
        let env = crate::runner::variant::source_environment(&stub)?;
        let value = invoke(&env, arguments(family, field), 0)?;
        check_reply(family, field, expected, keeps_valid, &value)?;
        stub.finish()?;
    }
    Ok(())
}
fn fixture(family: &str) -> (&'static str, Value, &'static str) {
    match family {
        "mydisease-doid" => (
            "mydisease.info",
            json!({"_id":"DOID:3910","disease_ontology":{"doid":"DOID:3910","name":"Valid disease"}}),
            "/disease_ontology",
        ),
        "mychem" => ("mychem.info", json!({"_id":"CHEMBL25"}), ""),
        "ols" => (
            "www.ebi.ac.uk",
            json!({"_embedded":{"terms":[{"obo_id":"MONDO:0005061","label":"Valid disease","ontology_name":"mondo","iri":"http://purl.obolibrary.org/obo/MONDO_0005061","description":[],"synonyms":["valid"],"annotation":{}}]}}),
            "/_embedded/terms/0",
        ),
        _ => (
            "mydisease.info",
            json!({"_id":"MONDO:0005061","mondo":{"mondo":"MONDO:0005061","label":"Valid disease"}}),
            "/mondo",
        ),
    }
}
fn replace_field(
    family: &str,
    field: &str,
    invalid: &Value,
    body: &mut Value,
    pointer: &str,
) -> Result<(), String> {
    let mut target = body.pointer_mut(pointer).ok_or("fixture root")?;
    let mut path = field.split('.').peekable();
    while let Some(part) = path.next() {
        if path.peek().is_none() {
            target
                .as_object_mut()
                .ok_or("fixture object")?
                .insert(part.into(), invalid.clone());
        } else {
            target = target
                .as_object_mut()
                .ok_or("fixture object")?
                .entry(part)
                .or_insert_with(|| json!({}));
        }
    }
    if family == "ols" && field == "obo_synonym" {
        body.pointer_mut(pointer)
            .and_then(Value::as_object_mut)
            .ok_or("term")?
            .insert("synonyms".into(), json!(["valid"]));
    }
    Ok(())
}
fn arguments(family: &str, field: &str) -> Vec<String> {
    let section = if field == "drugbank"
        || field.contains("xrefs")
        || field.contains("molecule_hierarchy")
        || field == "obo_xref"
    {
        Some("xrefs")
    } else if field.contains("synonym") {
        Some("synonyms")
    } else {
        None
    };
    let mut args = if family == "mychem" {
        vec!["get", "drug", "CHEMBL25", "--source", "mychem"]
    } else {
        vec![
            "get",
            "disease",
            if family == "mydisease-doid" {
                "DOID:3910"
            } else {
                "MONDO:0005061"
            },
            "--source",
            if family == "mydisease-doid" {
                "mydisease"
            } else {
                family
            },
        ]
    };
    if let Some(section) = section
        && (family != "mychem" || section == "xrefs")
    {
        args.insert(3, section);
    }
    if family == "ols" {
        args.extend(["--ontology", "mondo"]);
    }
    args.into_iter().map(str::to_owned).collect()
}
fn check_reply(
    family: &str,
    field: &str,
    expected: &str,
    keeps_valid: bool,
    value: &Value,
) -> Result<(), String> {
    let problems = value
        .get("problems")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{family}/{field} lost problem: {value}"))?;
    let named: Vec<_> = problems
        .iter()
        .filter(|p| p.get("field") == Some(&json!(expected)))
        .collect();
    let record = if family == "mychem" {
        "CHEMBL25"
    } else if family == "mydisease-doid" {
        "DOID:3910"
    } else {
        "MONDO:0005061"
    };
    if named.len() != 1
        || named.first().and_then(|p| p.get("record")) != Some(&json!(record))
        || (keeps_valid && !value.get("result").is_some_and(|v| has_string(v, "valid")))
        || (field == "drugbank"
            && !value
                .get("result")
                .is_some_and(|v| has_string(v, "DB00001")))
        || value.get("status") != Some(&json!("partial"))
        || value.get("result").is_none_or(Value::is_null)
        || value.get("result").is_some_and(contains_invalid)
    {
        return Err(format!(
            "{family}/{field} lost reply or named problem {expected}: {value}"
        ));
    }
    Ok(())
}

fn contains_invalid(value: &Value) -> bool {
    match value {
        Value::String(value) => value.is_empty() || value == "invalid:",
        Value::Array(values) => values.iter().any(contains_invalid),
        Value::Object(values) => values.values().any(contains_invalid),
        _ => false,
    }
}

fn search(family: &str) -> Result<(), String> {
    let (host, body, expected) = if family == "mychem" {
        (
            "mychem.info",
            json!({"total":1,"hits":[{"_id":"CHEMBL25","drugbank":{"name":""}}]}),
            "drugbank.name",
        )
    } else {
        (
            "www.ebi.ac.uk",
            json!({"response":{"numFound":1,"start":0,"docs":[{"obo_id":"MONDO:0005061","label":"","ontology_name":"mondo","iri":""}]}}),
            "label",
        )
    };
    let stub = Stub::for_source_status(
        host,
        "application/json",
        move |_| Ok(body.to_string()),
        |_| 200,
    )?;
    let env = crate::runner::variant::source_environment(&stub)?;
    let mut args = vec![
        "search",
        if family == "mychem" {
            "drug"
        } else {
            "disease"
        },
        "valid",
        "--source",
        family,
    ];
    if family == "ols" {
        args.extend(["--ontology", "mondo"]);
    }
    let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
    if value.pointer("/result/items/0").is_none()
        || value.get("result").is_some_and(contains_invalid)
        || !value
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|values| {
                values
                    .iter()
                    .any(|v| v.get("field") == Some(&json!(expected)))
            })
    {
        return Err(format!("bad search field lost item or problem: {value}"));
    }
    stub.finish()?;
    Ok(())
}

fn missing_trial_label() -> Result<(), String> {
    let stub = Stub::for_source("mydisease.info", "application/json", |_| {
        Ok(json!({"_id":"MONDO:0005061","mondo":{"mondo":"MONDO:0005061","label":""}}).to_string())
    })?;
    let env = crate::runner::variant::source_environment(&stub)?;
    let value = invoke(
        &env,
        [
            "search",
            "trial",
            "--condition-id",
            "MONDO:0005061",
            "--limit",
            "1",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        4,
    )?;
    if value.pointer("/result/condition_resolution/concept/id") != Some(&json!("MONDO:0005061"))
        || value
            .pointer("/result/condition_resolution/concept/label")
            .is_some()
        || value
            .get("sources")
            .and_then(Value::as_array)
            .is_none_or(|sources| sources.len() != 1)
        || value.pointer("/problems/0/field") != Some(&json!("mondo.label"))
    {
        return Err(format!(
            "unusable resolver label caused trial I/O or lost identity: {value}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("unexpected resolver or trial request".into());
    }
    Ok(())
}

fn has_string(value: &Value, expected: &str) -> bool {
    match value {
        Value::String(value) => value == expected,
        Value::Array(values) => values.iter().any(|value| has_string(value, expected)),
        Value::Object(values) => values.values().any(|value| has_string(value, expected)),
        _ => false,
    }
}
