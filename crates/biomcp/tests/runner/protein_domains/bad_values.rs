//! Feature protein-domains-complexes: bad optional facts retain source identities.
use super::*;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "protein-domains-complexes/invalid-optional-fields-are-disclosed",
        || run().map_err(Into::into),
    )]
}
fn run() -> Result<(), String> {
    for (command, path, field, absent) in [
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
        for invalid in [json!(""), json!({"invalid":true})] {
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
