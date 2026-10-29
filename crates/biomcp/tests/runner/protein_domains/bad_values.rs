//! Feature protein-domains-complexes: bad optional facts retain source identities.
use super::{Stub, Trial, Value, call, json, setup};
use crate::runner::transcript::trial as transcript_trial;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "protein-domains-complexes/invalid-collection-members-keep-siblings",
            || nested().map_err(Into::into),
        ),
        transcript_trial(
            "protein-domains-complexes/invalid-optional-fields-are-disclosed",
            || run().map_err(Into::into),
        ),
        transcript_trial(
            "protein-domains-complexes/invalid-fragments-keep-valid-siblings",
            || fragments().map_err(Into::into),
        ),
    ]
}
fn run() -> Result<(), String> {
    for &(command, path, field, absent) in OPTIONAL {
        for invalid in [json!(""), json!(true), Value::Null] {
            let invalid = if field == "predictedComplex" && invalid == json!(true) {
                json!(1)
            } else {
                invalid
            };
            let absent_value = invalid.is_null();
            if absent_value
                && ["/identifier", "/PF00870", "/Pfam"]
                    .iter()
                    .any(|suffix| path.ends_with(suffix))
            {
                continue;
            }
            let stub = Stub::for_sources(
                &["www.ebi.ac.uk"],
                |_| "application/json",
                move |request| {
                    let mut body = if command == "domains" {
                        domain_reply(json!([{"start":1,"end":10,"dc-status":"CONTINUOUS"}]))
                    } else {
                        serde_json::from_str(&super::complexes::reply(request)?)
                            .map_err(|e| e.to_string())?
                    };
                    *body.pointer_mut(path).ok_or("missing fixture field")? = invalid.clone();
                    Ok(body.to_string())
                },
            )?;
            let env = setup(&stub)?;
            let args = if command != "complex-members" {
                vec!["protein", command, "P38398"]
            } else {
                vec!["protein", command, "P38398", "--complex", "CPX-715"]
            };
            let value = call(&env, &args, 0)?;
            check_field(&value, command, field, absent, absent_value)?;
            if command == "domains" && field == "type" {
                let filtered = call(
                    &env,
                    &["protein", "domains", "P38398", "--type", "family"],
                    0,
                )?;
                if filtered.pointer("/result/items/0").is_some()
                    || filtered.pointer("/page/total") != Some(&json!(0))
                    || filtered.get("problems") != value.get("problems")
                {
                    return Err(format!(
                        "missing native type was guessed or lost its problem: {filtered}"
                    ));
                }
            }
            let cached = call(&env, &args, 0)?;
            if cached.get("problems") != value.get("problems") {
                return Err("retained collection lost problems".into());
            }
            if stub.finish()?.len() != 1 {
                return Err("retained source values were refetched".into());
            }
        }
    }
    Ok(())
}
fn fragments() -> Result<(), String> {
    for (field, invalid) in [
        ("start", json!(0)),
        ("start", json!("bad")),
        ("end", json!(0)),
        ("end", json!(9)),
        ("end", json!(2001)),
        ("end", json!({})),
    ] {
        let stub = Stub::for_sources(
            &["www.ebi.ac.uk"],
            |_| "application/json",
            move |_| {
                let mut bad = json!({"start":10,"end":20});
                *bad.get_mut(field).ok_or("fragment coordinate")? = invalid.clone();
                Ok(domain_reply(json!([bad,{"start":30,"end":40}])).to_string())
            },
        )?;
        let env = setup(&stub)?;
        let value = call(&env, &["protein", "domains", "P38398"], 0)?;
        if value.pointer("/result/items/0/locations/0/fragments/0/span/start/min")
            != Some(&json!(29))
            || value
                .pointer("/result/items/0/locations/0/fragments/1")
                .is_some()
            || value.pointer("/problems/0/field") != Some(&json!(format!("fragments.{field}")))
        {
            return Err(format!(
                "bad coordinate lost siblings or field problem: {value}"
            ));
        }
        stub.finish()?;
    }
    Ok(())
}

fn domain_reply(fragments: Value) -> Value {
    json!({
        "count":1, "next":null,
        "results":[{
            "metadata":{
                "accession":"IPR002117", "name":"Native family", "source_database":"interpro",
                "type":"family", "member_databases":{"Pfam":{"PF00870":"Valid"}}
            },
            "proteins":[{
                "accession":"p38398", "organism":"9606", "protein_length":2000,
                "entry_protein_locations":[{"fragments":fragments}]
            }]
        }]
    })
}

fn nested() -> Result<(), String> {
    for (command, path, field, invalid, present, expected, absent) in nested_domain_cases()
        .into_iter()
        .chain(nested_complex_cases())
    {
        let stub = Stub::for_sources(
            &["www.ebi.ac.uk"],
            |_| "application/json",
            move |request| {
                let mut body = if command == "domains" {
                    domain_reply(json!([]))
                } else {
                    serde_json::from_str(&super::complexes::reply(request)?)
                        .map_err(|e| e.to_string())?
                };
                *body.pointer_mut(path).ok_or("fixture field")? = invalid.clone();
                if field == "participants.crossReferences" {
                    let member = body
                        .pointer_mut("/participants/0")
                        .and_then(Value::as_object_mut)
                        .ok_or("member")?;
                    let value = member.remove("xrefs").ok_or("xrefs")?;
                    member.insert("crossReferences".into(), value);
                }
                Ok(body.to_string())
            },
        )?;
        let env = setup(&stub)?;
        let args = if command == "domains" {
            vec!["protein", command, "P38398"]
        } else {
            vec!["protein", command, "P38398", "--complex", "CPX-715"]
        };
        let value = call(&env, &args, 0)?;
        if value.pointer(present) != Some(&expected)
            || value.pointer(absent).is_some()
            || value.pointer("/problems/0/field") != Some(&json!(field))
        {
            return Err(format!(
                "bad {field} lost valid siblings or disclosure: {value}"
            ));
        }
        stub.finish()?;
    }
    Ok(())
}

const OPTIONAL: &[(&str, &str, &str, &str)] = &[
    (
        "domains",
        "/results/0/proteins/0/entry_protein_locations",
        "entry_protein_locations",
        "/result/items/0/locations",
    ),
    (
        "domains",
        "/results/0/proteins/0/entry_protein_locations/0/fragments",
        "fragments",
        "/result/items/0/locations/0/fragments",
    ),
    (
        "domains",
        "/results/0/metadata/name",
        "name",
        "/result/items/0/name",
    ),
    (
        "domains",
        "/results/0/metadata/type",
        "type",
        "/result/items/0/entry_type",
    ),
    (
        "domains",
        "/results/0/proteins/0/protein_length",
        "protein_length",
        "/result/items/0/protein_length",
    ),
    (
        "domains",
        "/results/0/metadata/member_databases",
        "member_databases",
        "/result/items/0/member_signatures",
    ),
    (
        "domains",
        "/results/0/metadata/member_databases/Pfam",
        "member_databases.entries",
        "/result/items/0/member_signatures",
    ),
    (
        "complexes",
        "/elements/0/complexName",
        "name",
        "/result/items/0/name",
    ),
    (
        "complexes",
        "/elements/0/predictedComplex",
        "predictedComplex",
        "/result/items/0/predicted",
    ),
    (
        "complex-members",
        "/participants/0/xrefs",
        "participants.xrefs",
        "/result/items/0/xrefs",
    ),
    (
        "complex-members",
        "/participants/0/xrefs/0/identifier",
        "participants.xrefs",
        "/result/items/0/xrefs",
    ),
    (
        "domains",
        "/results/0/metadata/member_databases/Pfam/PF00870",
        "member_databases.name",
        "/result/items/0/member_signatures/0/name",
    ),
    (
        "domains",
        "/results/0/proteins/0/entry_protein_locations/0/fragments/0/dc-status",
        "dc-status",
        "/result/items/0/locations/0/fragments/0/dc_status",
    ),
    (
        "complex-members",
        "/participants/0/name",
        "participants.name",
        "/result/items/0/name",
    ),
    (
        "complex-members",
        "/participants/0/stochiometry",
        "participants.stochiometry",
        "/result/items/0/stoichiometry",
    ),
    (
        "complex-members",
        "/participants/0/organismName",
        "participants.organismName",
        "/result/items/0/organism",
    ),
];

fn check_field(
    value: &Value,
    command: &str,
    field: &str,
    absent: &str,
    absent_value: bool,
) -> Result<(), String> {
    if value.pointer(absent).is_some()
        || (!absent_value
            && !value
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|p| p.iter().any(|p| p.get("field") == Some(&json!(field)))))
        || value.pointer("/result/items/0/id").is_none()
    {
        return Err(format!("bad {field} lost reply or disclosure: {value}"));
    }
    if absent_value
        && value
            .get("problems")
            .is_some_and(|p| p.as_array().is_none_or(|p| !p.is_empty()))
    {
        return Err(format!("absent {field} created problems: {value}"));
    }
    if !absent_value {
        let expected_record = if command == "domains" {
            "IPR002117"
        } else if command == "complexes" {
            "CPX-100"
        } else {
            "CPX-715"
        };
        if value.pointer("/problems/0/record") != Some(&json!(expected_record))
            || value
                .get("problems")
                .and_then(Value::as_array)
                .map(Vec::len)
                != Some(1)
            || value.get("status") != Some(&json!("partial"))
        {
            return Err(format!("bad field lost its single record problem: {value}"));
        }
    }
    Ok(())
}

type NestedCase = (
    &'static str,
    &'static str,
    &'static str,
    Value,
    &'static str,
    Value,
    &'static str,
);
fn nested_domain_cases() -> Vec<NestedCase> {
    vec![
        (
            "domains",
            "/results/0/metadata/member_databases",
            "member_databases.database",
            json!({"":{"PF00001":"Bad"},"Pfam":{"PF00870":"Valid"}}),
            "/result/items/0/member_signatures/0/accession",
            json!("PF00870"),
            "/result/items/0/member_signatures/1",
        ),
        (
            "domains",
            "/results/0/metadata/member_databases",
            "member_databases.accession",
            json!({"Pfam":{"":"Bad","PF00870":"Valid"}}),
            "/result/items/0/member_signatures/0/accession",
            json!("PF00870"),
            "/result/items/0/member_signatures/1",
        ),
        (
            "domains",
            "/results/0/proteins/0/protein_length",
            "protein_length",
            json!(0),
            "/result/items/0/id",
            json!("IPR002117"),
            "/result/items/0/protein_length",
        ),
    ]
}
fn nested_complex_cases() -> Vec<NestedCase> {
    vec![
        (
            "complex-members",
            "/participants/0/organismName",
            "participants.taxon",
            json!("Homo sapiens; abc"),
            "/result/items/0/organism",
            json!("Homo sapiens; abc"),
            "/result/items/0/taxon",
        ),
        (
            "complex-members",
            "/participants/0/organismName",
            "participants.taxon",
            json!("Homo sapiens; 0"),
            "/result/items/0/organism",
            json!("Homo sapiens; 0"),
            "/result/items/0/taxon",
        ),
        (
            "complex-members",
            "/participants/0/organismName",
            "participants.taxon",
            json!("Homo sapiens; -3"),
            "/result/items/0/organism",
            json!("Homo sapiens; -3"),
            "/result/items/0/taxon",
        ),
        (
            "complex-members",
            "/participants/1/identifier",
            "participants.protein",
            json!("native-unresolved-protein"),
            "/result/items/1/id",
            json!("native-unresolved-protein"),
            "/result/items/1/protein",
        ),
        (
            "complex-members",
            "/participants/0/xrefs",
            "participants.xrefs",
            json!([{"database":"","identifier":"bad"},{"database":"valid","identifier":"good"}]),
            "/result/items/0/xrefs/0",
            json!("valid:good"),
            "/result/items/0/xrefs/1",
        ),
        (
            "complex-members",
            "/participants/0/xrefs",
            "participants.crossReferences",
            json!(true),
            "/result/items/0/id",
            json!("P38398"),
            "/result/items/0/xrefs",
        ),
    ]
}
