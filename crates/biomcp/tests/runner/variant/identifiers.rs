//! Feature variant-identity: every printed identity and statement subject is reusable input.
use crate::runner::transcript::trial as transcript_trial;
use crate::runner::{configuration::invoke, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("variant/printed-identifiers-roundtrip", || {
            check().map_err(Into::into)
        }),
        transcript_trial("variant/direct-record-identities-are-refused", || {
            refused().map_err(Into::into)
        }),
    ]
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

fn refused() -> Result<(), String> {
    for (host, input, field, bad) in [
        (
            "reg.genome.network",
            "CA123643",
            "@id",
            json!("http://reg.genome.network/allele/CA999"),
        ),
        ("reg.genome.network", "CA123643", "@id", json!(42)),
        (
            "myvariant.info",
            "chr7:g.140753336A>T",
            "identity",
            json!("chr7:g.140753336A>G"),
        ),
        (
            "myvariant.info",
            "chr7:g.140753336A>T",
            "identity",
            json!(42),
        ),
    ] {
        let stub = Stub::for_source(host, "application/json", move |_| {
            Ok(if host == "reg.genome.network" {
                json!({"@id":bad})
            } else {
                json!({"_id":bad})
            }
            .to_string())
        })?;
        let env = super::source_environment(&stub)?;
        let args = if host == "reg.genome.network" {
            strings(&["get", "variant", input])
        } else {
            strings(&["get", "variant", input, "--build", "GRCh38"])
        };
        for _ in 0..2 {
            let output = env
                .command(env!("CARGO_BIN_EXE_biomcp"), &args)
                .output()
                .map_err(|e| e.to_string())?;
            let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
            let problems = value
                .get("problems")
                .and_then(Value::as_array)
                .ok_or("missing identity problem")?;
            if problems.len() != 1
                || problems.first().and_then(|p| p.get("field")) != Some(&json!(field))
                || value.pointer("/result/candidates").is_some()
                || value.pointer("/sources/0/state") != Some(&json!("ok"))
            {
                return Err(format!(
                    "bad direct identity was admitted or failed the source: {value}"
                ));
            }
        }
        let mut offline = args;
        offline.push("--offline".into());
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &offline)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/sources/0/reason") != Some(&json!("offline"))
            || stub.finish()?.len() != 2
        {
            return Err(format!(
                "bad direct identity entered the reply cache: {value}"
            ));
        }
    }
    Ok(())
}
