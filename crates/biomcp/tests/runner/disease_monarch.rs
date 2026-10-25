//! Feature disease-monarch: native association facts stay source-owned.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("disease-monarch/native-endpoints-and-paging", || exact().map_err(Into::into))]
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("monarch-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca})).map_err(|e|e.to_string())?;
    Ok(env)
}
fn invoke(env: &Environment, words: &[&str], exit: i32) -> Result<Value, String> {
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &words.iter().map(|s| (*s).into()).collect::<Vec<String>>()).output().map_err(|e|e.to_string())?;
    if output.status.code() != Some(exit) {return Err(format!("unexpected exit {:?}: {}",output.status.code(),String::from_utf8_lossy(&output.stdout)));}
    serde_json::from_slice(&output.stdout).map_err(|e|e.to_string())
}
fn association() -> Value {
    json!({"id":"synthetic-row","subject":"HGNC:9967","subject_label":"Synthetic gene","object":"MONDO:0007958","object_label":"Synthetic child","predicate":"biolink:associated_with","original_subject":"NCBIGene:5979","original_object":"Orphanet:99361","primary_knowledge_source":"infores:orphanet","aggregator_knowledge_source":["infores:monarchinitiative","infores:hpo-annotations"],"provided_by":"synthetic_collection","has_evidence":["ECO:0000501"],"qualifiers":null,"negated":false,"subject_closure":null,"object_closure":["MONDO:0019003"]})
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source_status("api-v3.monarchinitiative.org","application/json",|request| {
        if !request.starts_with("GET /v3/api/association?object=MONDO%3A0019003&subject_category=biolink%3AGene&limit=5&offset=0 ") {return Err(format!("unexpected request: {request}"));}
        Ok(json!({"limit":5,"offset":0,"total":8,"items":[association()]}).to_string())
    }, |_| 200)?;
    let env = setup(&stub)?;
    let value = invoke(&env, &["get","disease","MONDO:0019003","genes","--source","monarch"],0)?;
    let row = value.pointer("/result/items/0/association").ok_or("missing association")?;
    for (key, expected) in [("requested_id",json!("MONDO:0019003")),("object",json!({"id":"MONDO:0007958","label":"Synthetic child"})),("aggregator_knowledge_source",json!(["infores:monarchinitiative","infores:hpo-annotations"])),("qualifiers",json!([]))] {
        if row.get(key) != Some(&expected) { return Err(format!("lost {key}: {row}")); }
    }
    if value.pointer("/page/total") != Some(&json!(8)) || value.pointer("/page/returned") != Some(&json!(1)) {return Err("association page counts changed".into());}
    if stub.finish()?.len() != 1 {return Err("unexpected fanout".into());}
    Ok(())
}
