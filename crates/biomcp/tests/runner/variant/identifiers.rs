//! Feature variant-identity: every printed identity and statement subject is reusable input.
use crate::runner::transcript::trial as transcript_trial;
use crate::runner::{configuration::invoke, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "variant/printed-identifiers-roundtrip",
        || check().map_err(Into::into),
    )]
}
fn check() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", body)?;
    let env = super::source_environment(&stub)?;
    let search = invoke(
        &env,
        strings(&[
            "search",
            "variant",
            "--query",
            "BRAF",
            "--source",
            "myvariant",
        ]),
        0,
    )?;
    let ids = search
        .pointer("/result/items/0/identity/ids")
        .and_then(Value::as_array)
        .ok_or("search omitted identity IDs")?;
    if ids != &[json!("rs113488022"), json!("clinvar:13961")] {
        return Err(format!("search changed published identifiers: {ids:?}"));
    }
    for id in ids {
        let id = id.as_str().ok_or("non-text variant identifier")?;
        let card = invoke(&env, strings(&["get", "variant", id]), 0)?;
        if card.pointer("/result/candidates/0/identity/ids") != Some(&json!(ids)) {
            return Err(format!("printed ID did not resolve: {id}: {card}"));
        }
    }
    stub.finish()?;
    subjects()
}
fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).into()).collect()
}
fn body(request: &str) -> Result<String, String> {
    let hit = json!({"_id":"chr7:g.140453136A>T", "dbsnp":{"rsid":"rs113488022"}, "clinvar":{"variant_id":13961}});
    Ok(if request.starts_with("GET /v1/query?") {
        json!({"total":1,"hits":[hit]})
    } else {
        hit
    }
    .to_string())
}

fn subjects() -> Result<(), String> {
    let stub = Stub::for_source("erepo.clinicalgenome.org", "application/json", |_| {
        Ok(crate::runner::variant_assertions::body("CA000434"))
    })?;
    let env = super::source_environment(&stub)?;
    let card = invoke(&env, strings(&["get", "variant", "CA000434", "erepo"]), 0)?;
    if card.pointer("/query/build").is_some() || card.pointer("/query/build_defaulted").is_some() {
        return Err(format!(
            "direct source read gained an unused assembly default: {card}"
        ));
    }
    let rows = card
        .pointer("/result/candidates/0/sections/erepo/data/assertions")
        .and_then(Value::as_array)
        .ok_or("missing statement rows")?;
    for row in rows {
        let id = row
            .pointer("/statement/subject/id")
            .and_then(Value::as_str)
            .ok_or("missing statement subject identifier")?;
        let followed = invoke(&env, strings(&["get", "variant", id, "erepo"]), 0)?;
        if followed != card {
            return Err(format!(
                "printed subject did not roundtrip: {id}: {followed}"
            ));
        }
    }
    stub.finish()?;
    Ok(())
}
