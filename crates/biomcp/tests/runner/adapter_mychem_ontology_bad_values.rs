//! Features drug-mychem, disease-ols and disease-mydisease: malformed optional fields retain usable replies.
use super::super::{configuration::invoke, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    ["mychem", "ols", "mydisease"]
        .into_iter()
        .map(|family| {
            let feature = if family == "mychem" {
                "drug-mychem"
            } else if family == "ols" {
                "disease-ols"
            } else {
                "disease-mydisease"
            };
            Trial::test(format!("{feature}/bad-values-keep-reply"), move || {
                check(family).map_err(Into::into)
            })
        })
        .collect()
}
fn check(family: &str) -> Result<(), String> {
    let cases: Vec<(&str, Value)> = match family {
        "mychem" => vec![
            ("name", json!("")),
            ("name", json!(42)),
            ("synonyms", json!(["valid", ""])),
            ("id", json!("")),
            ("xrefs", json!({"chebi":[""]})),
            ("molecule_hierarchy", json!({"parent_chembl_id":""})),
        ],
        "ols" => vec![
            ("description", json!(["valid", ""])),
            ("synonyms", json!(["valid", ""])),
            ("term_replaced_by", json!(["invalid:"])),
            ("annotation", json!({"consider":["invalid:"]})),
            ("obo_synonym", json!([{"name":"valid", "scope":""}])),
            ("obo_xref", json!([{"database":"", "id":"12"}])),
        ],
        _ => vec![
            ("definition", json!("")),
            ("definition", json!(42)),
            ("synonym", json!({"exact":["valid", ""]})),
            ("xrefs", json!({"doid":[""]})),
            ("parents", json!(["invalid:"])),
            ("consider", json!(["invalid:"])),
            ("replaced_by", json!(["invalid:"])),
        ],
    };
    for (field, invalid) in cases {
        let (host, mut body, pointer, expected) = match family {
            "mychem" => (
                "mychem.info",
                json!({"_id":"CHEMBL25", "drugbank":{"name":"Valid drug"}}),
                "/drugbank",
                format!("drugbank.{field}"),
            ),
            "ols" => (
                "www.ebi.ac.uk",
                json!({"_embedded":{"terms":[{"obo_id":"MONDO:0005061","label":"Valid disease","ontology_name":"mondo","iri":"http://purl.obolibrary.org/obo/MONDO_0005061","description":[],"synonyms":[],"annotation":{}}]}}),
                "/_embedded/terms/0",
                match field {
                    "annotation" => "annotation.consider",
                    "obo_synonym" => "synonym.scope",
                    "obo_xref" => "xref.id",
                    _ => field,
                }
                .to_owned(),
            ),
            _ => (
                "mydisease.info",
                json!({"_id":"MONDO:0005061","mondo":{"mondo":"MONDO:0005061","label":"Valid disease"}}),
                "/mondo",
                format!(
                    "mondo.{}",
                    if field == "synonym" {
                        "synonym.exact"
                    } else if field == "xrefs" {
                        "xrefs.doid"
                    } else {
                        field
                    }
                ),
            ),
        };
        body.pointer_mut(pointer)
            .and_then(Value::as_object_mut)
            .ok_or("fixture object")?
            .insert(field.into(), invalid);
        if family == "ols" && field == "obo_synonym" {
            body.pointer_mut(pointer)
                .and_then(Value::as_object_mut)
                .ok_or("term")?
                .insert("synonyms".into(), json!(["valid"]));
        }
        let stub = Stub::for_source(host, "application/json", move |_| Ok(body.to_string()))?;
        let env = super::super::variant::source_environment(&stub)?;
        let mut args = if family == "mychem" {
            vec!["get", "drug", "CHEMBL25", "--source", "mychem"]
        } else {
            vec!["get", "disease", "MONDO:0005061", "--source", family]
        };
        if family == "ols" {
            args.extend(["--ontology", "mondo"]);
        }
        let value = invoke(&env, args.into_iter().map(str::to_owned).collect(), 0)?;
        let problems = value
            .get("problems")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{family}/{field} lost problem: {value}"))?;
        if !problems
            .iter()
            .any(|p| p.get("field") == Some(&json!(expected)))
            || value.get("result").is_none_or(Value::is_null)
        {
            return Err(format!(
                "{family}/{field} lost reply or named problem {expected}: {value}"
            ));
        }
        stub.finish()?;
    }
    Ok(())
}
