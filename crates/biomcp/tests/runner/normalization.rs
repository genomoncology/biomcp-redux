//! Named normalization preserves provider outcomes through the real command.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    // Keep dynamic state, source bindings or wire checks that YAML cannot express.
    vec![transcript_trial(
        crate::runner::required_contract::NORMALIZATION_PAGES.0,
        || walk().map_err(Into::into),
    )]
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("normalization-ca.pem");
    stub.retain(&environment);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(environment)
}
fn run(environment: &Environment, args: &[&str]) -> Result<Value, String> {
    let output = environment
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &args.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
        )
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn walk() -> Result<(), String> {
    let stub = Stub::for_source("mutalyzer.nl", "application/json", normalized)?;
    let environment = environment(&stub)?;
    let mut descriptions = Vec::new();
    for offset in ["0", "1", "2"] {
        let value = run(
            &environment,
            &[
                "variant",
                "normalize",
                "--source",
                "mutalyzer",
                "--hgvs",
                "NM_004333.4:c.1799T>A",
                "--limit",
                "1",
                "--offset",
                offset,
            ],
        )?;
        if value.pointer("/result/results/0/normalized") != Some(&json!("NM_004333.4:c.1799T>A"))
            || value.pointer("/result/results/0/total_mappings") != Some(&json!(3))
        {
            return Err(format!("provider result or total lost: {value}"));
        }
        descriptions.push(
            value
                .pointer("/result/results/0/mappings/0/value")
                .cloned()
                .ok_or("mapping absent")?,
        );
        if (offset != "2")
            != value
                .get("next")
                .and_then(Value::as_array)
                .is_some_and(|v| !v.is_empty())
        {
            return Err("mapping continuation lost".into());
        }
    }
    if descriptions
        != [
            json!("NP_004324.2:p.(Val600Glu)"),
            json!("NM_004333.4:r.(1799u>a)"),
            json!("NC_000007.14:g.140753336A>T"),
        ]
    {
        return Err("native mapping walk repeated or reordered rows".into());
    }
    let requests = stub.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err("warm pages made another request".into());
    };
    if !request.starts_with("GET /api/normalize/NM_004333.4%3Ac.1799T%3EA HTTP/1.1") {
        return Err(format!("unchanged request was altered: {request}"));
    }
    Ok(())
}
fn normalized(_: &str) -> Result<String, String> {
    Ok(json!(
        {"input_description":"NM_004333.4:c.1799T>A","normalized_description":"NM_004333.4:c.1799T>A",
        "corrected_description":"NM_004333.4:c.1799T>A","protein":{"description":"NP_004324.2:p.(Val600Glu)"},
        "rna":{"description":"NM_004333.4:r.(1799u>a)"},"equivalent_descriptions":{"g":[{"description":"NC_000007.14:g.140753336A>T"}]},
        "infos":[{"code":"ISOURCE","details":"  native whitespace  "}]}
    )
    .to_string())
}
fn warning() -> Result<(), String> {
    let stub = Stub::for_source("rest.variantvalidator.org", "application/json", rejected)?;
    let environment = environment(&stub)?;
    for _ in 0..2 {
        let value = run(
            &environment,
            &[
                "variant",
                "normalize",
                "--source",
                "variantvalidator",
                "--hgvs",
                "LRG_321p1:p.Arg273His",
                "--build",
                "GRCh38",
            ],
        )?;
        if value.pointer("/result/results/0/outcome") != Some(&json!("rejected"))
            || value.pointer("/result/results/0/messages/0/text")
                != Some(&json!("  provider warning  "))
            || value.pointer("/result/results/0/version/0/value")
                != Some(&json!("synthetic-version"))
            || value.pointer("/sources/0/reason") != Some(&json!("rejected"))
        {
            return Err(format!("warning was resolved or changed: {value}"));
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("validator cached a warning or made an undeclared fallback".into());
    }
    Ok(())
}

fn rejected(_: &str) -> Result<String, String> {
    Ok(json!({"flag":"warning",
"metadata":{"variantvalidator_version":"synthetic-version"},
"validation_warning_1":{"submitted_variant":"LRG_321p1:p.Arg273His",
"hgvs_transcript_variant":"",
"validation_warnings":["  provider warning  "]}})
    .to_string())
}

fn nested() -> Result<(), String> {
    let stub = Stub::for_source("mutalyzer.nl", "application/json", nested_body)?;
    let environment = environment(&stub)?;
    let args = [
        "variant",
        "normalize",
        "--source",
        "mutalyzer",
        "--hgvs",
        "NC_000017.10:g.7576633A>G",
    ];
    for _ in 0..2 {
        let value = run(&environment, &args)?;
        if value.pointer("/result/results/0/outcome") != Some(&json!("rejected"))
            || value.pointer("/result/results/0/messages/0/code")
                != Some(&json!("ESEQUENCEMISMATCH"))
        {
            return Err(format!("nested source rejection lost: {value}"));
        }
    }
    let mut offline = args.to_vec();
    offline.push("--offline");
    let value = run(&environment, &offline)?;
    if value.pointer("/sources/0/reason") != Some(&json!("offline")) || stub.finish()?.len() != 2 {
        return Err("provider rejection entered shared reply cache".into());
    }
    Ok(())
}
fn nested_body(_: &str) -> Result<String, String> {
    Ok(
        json!({"custom":{"input_description":"NC_000017.10:g.7576633A>G",
"errors":[{"code":"ESEQUENCEMISMATCH",
"details":"source reference mismatch"}]},
"message":"Errors encountered."})
        .to_string(),
    )
}

fn transcripts() -> Result<(), String> {
    let stub = Stub::for_source(
        "rest.variantvalidator.org",
        "application/json",
        transcripts_body,
    )?;
    let environment = environment(&stub)?;
    let value = run(
        &environment,
        &[
            "variant",
            "normalize",
            "--source",
            "variantvalidator",
            "--hgvs",
            "NM_004333.4:c.1799T>A",
            "--build",
            "GRCh38",
        ],
    )?;
    if value.pointer("/result/results/0/total_mappings") != Some(&json!(2))
        || value.pointer("/result/results/0/mappings/0/value") != Some(&json!("NM_004333.4:c.2A>T"))
        || value.pointer("/result/results/0/mappings/1/value") != Some(&json!("NM_004333.4:c.1A>T"))
    {
        return Err(format!("service transcript was dropped or chosen: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("normalization made an undeclared fallback".into());
    }
    Ok(())
}
fn transcripts_body(_: &str) -> Result<String, String> {
    Ok(
        "{\"flag\":\"gene_variant\",\"metadata\":{\"variantvalidator_version\":\"synthetic\"},\
                    \"z\":{\"submitted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.2A>T\"},\"a\":{\
        \"submitted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.1A>T\"}}"
            .into(),
    )
}
