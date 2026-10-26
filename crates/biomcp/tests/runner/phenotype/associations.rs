//! Feature phenotype-associations: exact direct native rows and safe identity pivots.
use super::{invoke, setup, Stub};
use libtest_mimic::Trial;
use serde_json::{json, Value};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("phenotype-associations/direct-pages-preserve-native-rows", || exact().map_err(Into::into))]
}
fn row() -> Value {
    json!({"id":"synthetic-first","subject":"HGNC:9910","subject_label":"RBMX","subject_taxon":"NCBITaxon:9606","object":"HP:0001250","object_label":"Seizure","predicate":"biolink:has_phenotype","original_subject":"NCBIGene:27316","has_evidence":null,"publications":["PMID:1"],"negated":false,"frequency_qualifier":"HP:0040282","subject_closure":["HGNC:9910"],"object_closure":["HP:0001250"]})
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", |request| {
        if !request.starts_with("GET /v3/api/association?object=HP%3A0001250&subject_category=biolink%3AGene&subject_taxon=NCBITaxon%3A9606&direct=true&limit=1&offset=0 ") { return Err(format!("wrong direct request: {request}")); }
        Ok(json!({"limit":1,"offset":0,"total":3,"items":[row()]}).to_string())
    })?;
    let env = setup(&stub)?;
    let result = invoke(&env, &["get","phenotype","HP:0001250","genes","--source","monarch"], 0)?;
    for (pointer, expected) in [
        ("/page/total", json!(3)),
        ("/page/returned", json!(1)),
        ("/result/direct", json!(true)),
        ("/result/items/0/association/requested_id", json!("HP:0001250")),
        ("/result/items/0/association/original_subject", json!("NCBIGene:27316")),
        ("/result/items/0/association/negated", json!(false)),
        ("/next/0/args", json!(["get","gene","HGNC:9910"])),
    ] { if result.pointer(pointer) != Some(&expected) { return Err(format!("{pointer} changed: {result}")); } }
    if stub.finish()?.len() != 1 { return Err("direct page fetched another resource".into()); }
    Ok(())
}
