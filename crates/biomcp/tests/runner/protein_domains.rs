//! Feature protein-domains-complexes: source facts and exact membership.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
mod complexes;
mod native;
pub(super) fn trials() -> Vec<Trial> {
    let root=super::launch::fixtures().join("protein-domain-native");
    let absent=!root.is_dir();
    vec![Trial::test("protein-domains-complexes/retained-native-facts-and-costs",move||native::run(&root).map_err(Into::into)).with_ignored_flag(absent),Trial::test("protein-domains-complexes/exact-members-and-retained-pages", || complexes::run().map_err(Into::into)), Trial::test(
        "protein-domains-complexes/native-types-and-fragments",
        || domains().map_err(Into::into),
    )]
}
fn domains() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", |_| {
        Ok(json!({
        "count":2,"next":null,"previous":null,"results":[
            {"metadata":{"accession":"IPR002117","name":"Native family","source_database":"interpro","type":"family","member_databases":{}},"proteins":[{"accession":"p04637","protein_length":393,"organism":"9606","entry_protein_locations":[{"fragments":[{"start":2,"end":10,"dc-status":"N_TERMINAL_DISC"},{"start":20,"end":30,"dc-status":"C_TERMINAL_DISC"}]}]}]},
            {"metadata":{"accession":"IPR010991","name":"Native domain","source_database":"interpro","type":"domain","member_databases":{}},"proteins":[{"accession":"p04637","protein_length":393,"organism":"9606","entry_protein_locations":[]}]}
        ]}).to_string())
    })?;
    let env = setup(&stub)?;
    let value = call(&env, &["protein", "domains", "P04637"], 0)?;
    if value.pointer("/result/items/0/entry_type") != Some(&json!("family"))
        || value
            .pointer("/result/items/0/locations/0/fragments")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(2)
    {
        return Err(format!("native types or fragments changed: {value}"));
    }
    let selected=call(&env,&["protein","domains","P04637","--type","domain"],0)?;
    if selected.pointer("/page/total")!=Some(&json!(1)) || selected.pointer("/result/native_total")!=Some(&json!(2)) || selected.pointer("/result/items/0/id")!=Some(&json!("IPR010991")){return Err("type selection changed the native count or classified a family as a domain".into());}
    let empty=call(&env,&["protein","domains","P04637","--type","repeat"],0)?;
    if empty.pointer("/page/total")!=Some(&json!(0)) || empty.pointer("/status")!=Some(&json!("complete")){return Err("empty type selection became a missing source".into());}
    let skipped=call(&env,&["protein","domains","P04637","--disable-sources","interpro"],4)?;
    if skipped.pointer("/sources/0/state")!=Some(&json!("skipped")) || skipped.get("result").is_some(){return Err("disabled source returned a retained collection".into());}
    stub.finish()?;
    Ok(())
}
fn call(env: &Environment, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut words = env.globals(None, false);
    words.extend(args.iter().map(|s| (*s).to_owned()));
    invoke(env, words, exit).map_err(|e| format!("{args:?}: {e}"))
}

fn setup(stub: &Stub) -> Result<Environment, String> {
    let env=Environment::new().map_err(|e|e.to_string())?;
    let ca=env.root().join("source-ca.pem");fs::write(&ca,&stub.certificate).map_err(|e|e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca})).map_err(|e|e.to_string())?;
    Ok(env)
}
