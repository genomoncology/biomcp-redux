//! Named normalization preserves provider outcomes through the real command.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant-normalization/native-mapping-walk-is-warm", || {
            walk().map_err(Into::into)
        }),
        Trial::test("variant-normalization/warning-is-rejection", || {
            warning().map_err(Into::into)
        }),
        Trial::test(
            "variant-normalization/all-keeps-independent-results",
            || independent().map_err(Into::into),
        ),
    ]
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("normalization-ca.pem");
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
    if requests.len() != 1
        || !requests[0]
            .1
            .starts_with("GET /api/normalize/NM_004333.4%3Ac.1799T%3EA HTTP/1.1")
    {
        return Err("warm pages made another call or unchanged request was altered".into());
    }
    Ok(())
}
fn normalized(_: &str) -> Result<String, String> {
    Ok(json!({"input_description":"NM_004333.4:c.1799T>A","normalized_description":"NM_004333.4:c.1799T>A","corrected_description":"NM_004333.4:c.1799T>A","protein":{"description":"NP_004324.2:p.(Val600Glu)"},"rna":{"description":"NM_004333.4:r.(1799u>a)"},"equivalent_descriptions":{"g":[{"description":"NC_000007.14:g.140753336A>T"}]},"infos":[{"code":"ISOURCE","details":"  native whitespace  "}]}).to_string())
}
fn warning() -> Result<(), String> {
    let stub = Stub::for_source("rest.variantvalidator.org", "application/json", rejected)?;
    let environment = environment(&stub)?;
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
        || value.pointer("/result/results/0/version/0/value") != Some(&json!("synthetic-version"))
        || value.pointer("/sources/0/reason") != Some(&json!("rejected"))
    {
        return Err(format!("warning was resolved or changed: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("validator made an undeclared fallback".into());
    }
    Ok(())
}
fn rejected(_: &str) -> Result<String, String> {
    Ok(json!({"flag":"warning","metadata":{"variantvalidator_version":"synthetic-version"},"validation_warning_1":{"submitted_variant":"LRG_321p1:p.Arg273His","hgvs_transcript_variant":"","validation_warnings":["  provider warning  "]}}).to_string())
}
fn independent() -> Result<(), String> {
    let stub = Stub::for_source("mutalyzer.nl", "application/json", normalized)?;
    let environment = environment(&stub)?;
    let value = run(
        &environment,
        &[
            "variant",
            "normalize",
            "--source",
            "all",
            "--hgvs",
            "NM_004333.4:c.1799T>A",
            "--build",
            "GRCh38",
            "--disable-sources",
            "car,variantvalidator",
        ],
    )?;
    if value.pointer("/status") != Some(&json!("partial"))
        || value.pointer("/result/results/0/outcome") != Some(&json!("unavailable"))
        || value.pointer("/result/results/1/outcome") != Some(&json!("resolved"))
        || value.pointer("/result/results/2/outcome") != Some(&json!("unavailable"))
    {
        return Err(format!("one failure hid another service: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled services made requests".into());
    }
    Ok(())
}
