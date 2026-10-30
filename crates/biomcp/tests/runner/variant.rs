//! Exact source request spelling crosses the real command and TLS transport.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    // The real peer checks resolver order, source requests and warm cache admission.
    vec![transcript_trial(
        crate::runner::required_contract::VALIDATOR_RESOLVER.0,
        || validator_order().map_err(Into::into),
    )]
}

type Requests = Vec<(String, String)>;
fn call(
    host: &'static str,
    body: fn(&str) -> Result<String, String>,
    args: &[&str],
) -> Result<(Value, Requests), String> {
    let stub = Stub::for_source(host, "application/json", body)?;
    let environment = source_environment(&stub)?;
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    Ok((value, stub.finish()?))
}

pub(super) fn source_environment(stub: &Stub) -> Result<Environment, String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    stub.retain(&environment);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}", stub.address), "ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(environment)
}

fn validator_order() -> Result<(), String> {
    let (value, requests) = call(
        "rest.variantvalidator.org",
        validator_body,
        &[
            "get",
            "variant",
            "NM_004333.4:c.1799T>A",
            "--resolver",
            "variantvalidator",
            "--build",
            "GRCh38",
        ],
    )?;
    if requests.len() != 1
        || value.pointer("/result/candidates/0/record/id") != Some(&json!("NM_004333.4:c.2A>T"))
        || value.pointer("/result/candidates/1/record/id") != Some(&json!("NM_004333.4:c.1A>T"))
        || value.pointer("/result/candidates/0/identity/representations/0/expression/value")
            != Some(&json!("NM_004333.4:c.2A>T"))
    {
        return Err(format!(
            "validator changed object order or lost transcript-only source forms: {value}"
        ));
    }
    Ok(())
}
fn validator_body(_: &str) -> Result<String, String> {
    Ok(
        "{\"flag\":\"gene_variant\",\"metadata\":{\"variantvalidator_version\":\"synthetic\"},\
                    \"z\":{\"submitted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.2A>T\"},\"a\":{\
        \"submitted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.1A>T\"}}"
            .into(),
    )
}
