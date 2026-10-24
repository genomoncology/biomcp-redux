//! Cohort identity, missing source fields and rejection cache boundaries.
use super::{launch::Environment, network_stub::Stub, recurrence::{args, call, cbio, config, row}};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant-recurrence/missing-fields-never-become-denominators", || missing().map_err(Into::into)),
        Trial::test("variant-recurrence/wrong-cohort-never-enters-cache", || mismatch().map_err(Into::into)),
        Trial::test("variant-recurrence/provider-rejections-never-enter-cache", || rejected().map_err(Into::into)),
    ]
}
fn peer(reply: fn(&str)->Result<String,String>) -> Result<(Environment, Stub),String> {
    let env = Environment::new().map_err(|e|e.to_string())?;
    let stub = Stub::for_sources(&["www.cbioportal.org", "reg.genome.network", "storage.googleapis.com"], |r| if r.contains("hgnc_complete_set") { "text/tab-separated-values" } else { "application/json" }, reply)?;
    config(&env, &stub)?;
    Ok((env, stub))
}
fn missing() -> Result<(),String> {
    let (env, stub) = peer(missing_reply)?;
    let value = call(&env,&args(),0)?;
    if value.pointer("/result/selected_sample_count").is_some() || value.pointer("/result/denominator").is_some()
        || value.pointer("/result/observed_sample_count") != Some(&json!(1))
        || value.pointer("/status") != Some(&json!("partial")) || value.pointer("/problems/0/field") != Some(&json!("mutation.exact_allele")) {
        return Err(format!("missing assembly/count was invented or uncertain allele joined: {value}"));
    }
    stub.finish()?;
    Ok(())
}
fn missing_reply(request: &str) -> Result<String,String> {
    let mut body: Value = serde_json::from_str(&cbio(request)?).map_err(|e|e.to_string())?;
    if request.contains("/sample-lists/") { body.as_object_mut().ok_or("sample-list object")?.remove("sampleCount"); }
    if request.contains("/mutations?") {
        let mut missing = row("missing-build","T"); missing.as_object_mut().ok_or("row")?.remove("ncbiBuild");
        let mut wrong = row("wrong-build","T"); wrong["ncbiBuild"] = json!("GRCh38");
        body = json!([missing,wrong,row("exact","T")]);
    }
    Ok(body.to_string())
}
fn mismatch() -> Result<(),String> {
    let (env, stub) = peer(mismatch_reply)?;
    for _ in 0..2 {
        let value = call(&env,&args(),4)?;
        if value.pointer("/sources/2/reason") != Some(&json!("bad_reply")) || value.pointer("/result/observed_sample_count").is_some() {
            return Err(format!("wrong cohort was accepted: {value}"));
        }
    }
    let mut offline = args(); offline.push("--offline".into());
    let value = call(&env,&offline,4)?;
    if value.pointer("/sources/2/reason") != Some(&json!("offline")) { return Err(format!("wrong cohort entered cache: {value}")); }
    if stub.finish()?.iter().filter(|(_,r)| r.contains("/mutations?")).count() != 2 { return Err("rejected cohort response was reused".into()); }
    Ok(())
}
fn mismatch_reply(request: &str) -> Result<String,String> {
    if request.contains("/mutations?") { let mut bad = row("s1","T"); bad["studyId"] = json!("wrong-study"); return Ok(json!([bad]).to_string()); }
    cbio(request)
}
fn rejected() -> Result<(),String> {
    let (env, stub) = peer(rejected_reply)?;
    for _ in 0..2 {
        let value = call(&env,&args(),4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("rejected")) || value.pointer("/result/observed_sample_count").is_some() { return Err(format!("provider rejection lost: {value}")); }
    }
    let mut offline=args(); offline.push("--offline".into());
    let value = call(&env,&offline,4)?;
    if value.pointer("/sources/0/reason") != Some(&json!("offline")) || stub.finish()?.len()!=2 { return Err("rejection entered identity cache or downstream source ran".into()); }
    Ok(())
}
fn rejected_reply(_: &str) -> Result<String,String> { Ok(json!({"errorType":"IncorrectReferenceAllele"}).to_string()) }
