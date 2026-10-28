//! Features drug-mychem, disease-ols and disease-mydisease: malformed optional fields retain usable replies.
use crate::runner::{configuration::invoke, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    ["mychem", "ols", "mydisease", "mydisease-doid"]
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
                check(family)
                    .and_then(|_| {
                        if matches!(family, "mychem" | "ols") {
                            search(family)
                        } else {
                            Ok(())
                        }
                    })
                    .map_err(Into::into)
            })
        })
        .collect()
}
fn check(family: &str) -> Result<(), String> {
    let cases = cases(family);
    for (field, invalid, expected) in cases {
        let (host, mut body, pointer) = match family {
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
        };
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
        let stub = Stub::for_source(host, "application/json", move |_| Ok(body.to_string()))?;
        let env = crate::runner::variant::source_environment(&stub)?;
        let section = if field.contains("xrefs")
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
        if let Some(section) = section {
            if family != "mychem" || section == "xrefs" {
                args.insert(3, section);
            }
        }
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
            || value.get("result").is_some_and(contains_invalid)
        {
            return Err(format!(
                "{family}/{field} lost reply or named problem {expected}: {value}"
            ));
        }
        stub.finish()?;
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
fn cases(family: &str) -> Vec<(&'static str, Value, &'static str)> {
    match family {
        "mychem" => {
            let mut cases: Vec<_> = [
                "drugbank.name",
                "drugbank.id",
                "drugcentral.id",
                "chembl.molecule_chembl_id",
                "chembl.pref_name",
                "chembl.molecule_type",
                "gtopdb.ligand_id",
                "gtopdb.name",
                "gtopdb.type",
                "gtopdb.chembl_id",
                "unii.unii",
                "unii.display_name",
                "unii.ingredient_type",
                "unii.rxcui",
                "unii.ncit",
                "ndc.product_id",
                "ndc.productndc",
                "ndc.producttypename",
                "ndc.nonproprietaryname",
                "ndc.proprietaryname",
                "ndc.applicationnumber",
                "chembl.molecule_hierarchy.parent_chembl_id",
                "chembl.molecule_hierarchy.active_chembl_id",
            ]
            .into_iter()
            .map(|field| (field, json!(""), field))
            .collect();
            cases.extend([
                ("drugbank.name", json!(42), "drugbank.name"),
                (
                    "drugbank.synonyms",
                    json!(["valid", ""]),
                    "drugbank.synonyms",
                ),
                (
                    "drugcentral.synonyms",
                    json!(["valid", ""]),
                    "drugcentral.synonyms",
                ),
                ("gtopdb.synonyms", json!(["valid", ""]), "gtopdb.synonyms"),
                (
                    "chembl.molecule_synonyms",
                    json!([{"molecule_synonym":""}]),
                    "chembl.molecule_synonyms.molecule_synonym",
                ),
                (
                    "drugcentral.xrefs.chebi",
                    json!([""]),
                    "drugcentral.xrefs.chebi",
                ),
                (
                    "gtopdb.xrefs.pubchem",
                    json!(["invalid:"]),
                    "gtopdb.xrefs.pubchem",
                ),
                ("drugbank.id", json!("invalid:"), "drugbank.id"),
                (
                    "openfda.generic_name",
                    json!(["valid", ""]),
                    "openfda.generic_name",
                ),
                (
                    "openfda.brand_name",
                    json!(["valid", ""]),
                    "openfda.brand_name",
                ),
                ("openfda.spl_set_id", json!([""]), "openfda.spl_set_id"),
                (
                    "openfda.application_number",
                    json!([""]),
                    "openfda.application_number",
                ),
            ]);
            cases
        }
        "ols" => vec![
            ("label", json!(""), "label"),
            ("iri", json!(""), "iri"),
            ("description", json!(["valid", ""]), "description"),
            ("description", json!(42), "description"),
            ("synonyms", json!(["valid", ""]), "synonyms"),
            ("term_replaced_by", json!(["invalid:"]), "term_replaced_by"),
            (
                "annotation",
                json!({"consider":["invalid:"]}),
                "annotation.consider",
            ),
            (
                "obo_synonym",
                json!([{"name":"valid", "scope":""}]),
                "synonym.scope",
            ),
            ("obo_synonym", json!([{"name":""}]), "synonym.name"),
            ("obo_xref", json!([{"database":"", "id":"12"}]), "xref.id"),
            ("obo_xref", json!([{"database":"DOID", "id":""}]), "xref.id"),
            (
                "obo_xref",
                json!([{"database":"DOID", "id":"12", "description":""}]),
                "xref.description",
            ),
        ],
        "mydisease-doid" => vec![
            ("name", json!(""), "disease_ontology.name"),
            ("def", json!(""), "disease_ontology.def"),
            (
                "synonyms",
                json!({"exact":["valid", ""]}),
                "disease_ontology.synonyms.exact",
            ),
            (
                "xrefs",
                json!({"mondo":[""]}),
                "disease_ontology.xrefs.mondo",
            ),
            ("parents", json!(["invalid:"]), "disease_ontology.parents"),
            ("consider", json!(["invalid:"]), "disease_ontology.consider"),
            (
                "replaced_by",
                json!(["invalid:"]),
                "disease_ontology.replaced_by",
            ),
        ],
        _ => vec![
            ("label", json!(""), "mondo.label"),
            ("definition", json!(""), "mondo.definition"),
            ("definition", json!(42), "mondo.definition"),
            (
                "synonym",
                json!({"exact":["valid", ""]}),
                "mondo.synonym.exact",
            ),
            ("xrefs", json!({"doid":[""]}), "mondo.xrefs.doid"),
            ("parents", json!(["invalid:"]), "mondo.parents"),
            ("consider", json!(["invalid:"]), "mondo.consider"),
            ("replaced_by", json!(["invalid:"]), "mondo.replaced_by"),
        ],
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
    let stub = Stub::for_source(host, "application/json", move |_| Ok(body.to_string()))?;
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
        || value.pointer("/result/items/0/header/name").is_some()
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
