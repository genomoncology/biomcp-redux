//! Feature disease-mydisease: exact annotation without fallback or hidden enrichment.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "disease-mydisease/exact-alias-and-independent-pages",
        || exact().map_err(Into::into),
    )]
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("mydisease-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| (*s).into()).collect()
}
fn invoke(env: &Environment, words: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), words)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "unexpected exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn replay(directory: &Path, steps: Value) -> Result<Value, String> {
    let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    fs::write(file.path(), serde_json::to_vec(&json!({"feature":"disease-mydisease","story":"Selected source facts stay complete and independently paged","replay":directory,"steps":steps})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    super::run_case(file.path(), &super::schema::Registry::embedded()?)
}
fn annotation() -> Value {
    json!({"_id":"MONDO:0005061","_version":1,
      "mondo":{"mondo":"MONDO:0005061","label":"Synthetic lung concept","definition":"Complete synthetic definition, including its original punctuation. [Native reference]","synonym":{"exact":["first native synonym","second native synonym"],"related":["related wording"]},"xrefs":{"doid":["DOID:3910","DOID:0111503"],"efo":["EFO:0000571"],"icd10cm":["C34.9"],"oncotree":["LUAD"]},"parents":["MONDO:0004970","MONDO:0005233"]},
      "disease_ontology":{"doid":"DOID:3910","name":"Different native DOID label","def":"Separate native DOID definition.","synonyms":{"exact":["DOID synonym"]},"xrefs":{"ncit":["C27745","C3512"],"snomedct_us_2025_09_01":"254626006"},"parents":["DOID:299","DOID:3908"]}})
}
fn reply(request: &str) -> Result<String, String> {
    if !request.starts_with("GET /v1/disease/DOID%3A3910?fields=mondo%2Cdisease_ontology ") {
        return Err(format!("unexpected source request: {request}"));
    }
    Ok(annotation().to_string())
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source("mydisease.info", "application/json", reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("exact");
    let mut command = args(&["get", "disease", "DOID:3910"]);
    command.extend(["--record".into(), directory.to_string_lossy().into()]);
    invoke(&env, &command, 0)?;
    replay(
        &directory,
        json!([{"run":["get","disease","DOID:3910"],"mcp":["get","disease","DOID:3910"],"expect":{"/result/requested_id":"DOID:3910","/result/returned_id":"MONDO:0005061","/result/alias/object/id":"DOID:3910","/result/definition":"Complete synthetic definition, including its original punctuation. [Native reference]","/result/sections":["synonyms","xrefs","hierarchy"]}}]),
    )?;
    if stub.finish()?.len() != 1 {
        return Err("unexpected source fanout".into());
    }
    Ok(())
}
