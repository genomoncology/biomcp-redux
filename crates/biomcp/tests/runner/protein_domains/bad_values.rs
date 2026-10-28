//! Feature protein-domains-complexes: bad optional facts retain source identities.
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "protein-domains-complexes/invalid-optional-fields-are-disclosed",
            || run().map_err(Into::into),
        ),
        Trial::test(
            "protein-domains-complexes/invalid-fragments-keep-valid-siblings",
            || fragments().map_err(Into::into),
        ),
    ]
}
fn run() -> Result<(), String> {
    for (command, path, field, absent) in [
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
    ] {
        for invalid in [json!(""), json!(true)] {
            let stub = Stub::for_source("www.ebi.ac.uk", "application/json", move |request| {
                let mut body = if command == "domains" {
                    json!({"count":1,"next":null,"results":[{"metadata":{"accession":"IPR002117","name":"Native family","source_database":"interpro","type":"family","member_databases":{"Pfam":{"PF00870":"Valid"}}},"proteins":[{"accession":"p38398","organism":"9606","protein_length":2000,"entry_protein_locations":[{"fragments":[{"start":1,"end":10,"dc-status":"CONTINUOUS"}]}]}]}]})
                } else {
                    serde_json::from_str(&super::complexes::reply(request)?)
                        .map_err(|e| e.to_string())?
                };
                *body.pointer_mut(path).ok_or("missing fixture field")? = invalid.clone();
                Ok(body.to_string())
            })?;
            let env = setup(&stub)?;
            let args = if command == "domains" {
                vec!["protein", command, "P38398"]
            } else if command == "complexes" {
                vec!["protein", command, "P38398"]
            } else {
                vec!["protein", command, "P38398", "--complex", "CPX-715"]
            };
            let value = call(&env, &args, 0)?;
            if value.pointer(absent).is_some()
                || !value
                    .get("problems")
                    .and_then(Value::as_array)
                    .is_some_and(|p| p.iter().any(|p| p.get("field") == Some(&json!(field))))
                || value.pointer("/result/items/0/id").is_none()
            {
                return Err(format!("bad {field} lost reply or disclosure: {value}"));
            }
            let cached = call(&env, &args, 0)?;
            if cached.get("problems") != value.get("problems") {
                return Err("retained collection lost problems".into());
            }
            stub.finish()?;
        }
    }
    Ok(())
}

fn fragments() -> Result<(), String> {
    for (field, invalid) in [
        ("start", json!(0)),
        ("start", json!("bad")),
        ("end", json!(0)),
        ("end", json!(2001)),
        ("end", json!({})),
    ] {
        let stub = Stub::for_source("www.ebi.ac.uk", "application/json", move |_| {
            let mut bad = json!({"start":10,"end":20});
            bad[field] = invalid.clone();
            Ok(json!({"count":1,"next":null,"results":[{"metadata":{"accession":"IPR002117","name":"Valid","source_database":"interpro","type":"family"},"proteins":[{"accession":"p38398","organism":"9606","protein_length":2000,"entry_protein_locations":[{"fragments":[bad,{"start":30,"end":40}]}]}]}]}).to_string())
        })?;
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
