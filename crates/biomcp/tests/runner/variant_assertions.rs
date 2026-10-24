//! Source assertion identity, source-local absence and cache admission through the CLI.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "variant/assertions-erepo-selected-version-uses-only-document",
            || history(false).map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-erepo-wrong-version-is-never-cached",
            || history(true).map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-erepo-source-zero-is-not-unknown-allele",
            || erepo_absence().map_err(Into::into),
        ),
        Trial::test("variant/assertions-erepo-route404-is-not-empty", || {
            route_failure().map_err(Into::into)
        }),
        Trial::test(
            "variant/assertions-erepo-private-identity-never-caches",
            || wrong_identity().map_err(Into::into),
        ),
    ]
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn call(env: &Environment, extra: &[&str]) -> Result<Value, String> {
    let args = ["get", "variant", "CA000434", "erepo"]
        .into_iter()
        .chain(extra.iter().copied())
        .map(Into::into)
        .collect::<Vec<_>>();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("{e}: {}", String::from_utf8_lossy(&output.stdout)))
}
fn body(caid: &str) -> String {
    json!({"status":{"code":200,"name":"OK"},"data":[{"caId":caid,"PCERDocID":"PCER000001171","docVersion":"1.1.0","classification":"Pathogenic","condition":"Li-Fraumeni syndrome","mondoId":"MONDO:0018875","ep":"TP53 VCEP","gene":"TP53","metCodes":["PP4_Moderate","PM2_Supporting","PS2","PS3","PS4","PM1","PP3"],"unMetCodes":["BS1"],"publishedDate":"2024-08-05","approvedDate":"2024-08-05","versionsList":["1.1.0","1.0.0"],"summaryDesc":"Synthetic public-fixture payload; structured PS4 and source prose PS4_Very Strong remain separate."}]}).to_string()
}
fn erepo_absence() -> Result<(), String> {
    let mut body_count = 0;
    let mut status_count = 0;
    let stub = Stub::for_source_status(
        "erepo.clinicalgenome.org",
        "application/json",
        move |_| {
            body_count += 1;
            Ok(if body_count > 1 {
                json!({"status":{"code":404,"name":"Not Found","msg":"No records were found for given query"}}).to_string()
            } else {
                body("CA000434")
            })
        },
        move |_| {
            status_count += 1;
            if status_count == 1 { 200 } else { 404 }
        },
    )?;
    let env = environment(&stub)?;
    let first = call(&env, &[])?;
    if first.pointer(
        "/result/candidates/0/sections/erepo/data/assertions/0/statement/attribution/record",
    ) != Some(&json!("PCER000001171"))
    {
        return Err(format!("native expert panel statement absent: {first}"));
    }
    for _ in 0..2 {
        let value = call(&env, &["--fresh"])?;
        if value.pointer("/result/candidates/0/sections/erepo/state") != Some(&json!("empty"))
            || value.pointer("/result/resolution") != Some(&json!("unresolved"))
            || value.pointer("/result/candidates/0/identity/ids").is_some()
        {
            return Err(format!(
                "empty assertion source invented allele absence or identity: {value}"
            ));
        }
    }
    let offline = call(&env, &["--offline"])?;
    if offline.pointer("/result/candidates/0/sections/erepo/reason") != Some(&json!("offline")) {
        return Err(format!(
            "HTTP404 left a positive or negative cache entry: {offline}"
        ));
    }
    let requests = stub.finish()?;
    if requests.len() != 3 {
        return Err(format!(
            "expected3actualrequests; sourceabsencecached or extraidentityprovider: {requests:?}"
        ));
    }
    Ok(())
}
fn route_failure() -> Result<(), String> {
    let stub = Stub::for_source_status(
        "erepo.clinicalgenome.org",
        "application/json",
        |_| Ok(json!({"status":{"code":404,"name":"Not Found"}}).to_string()),
        |_| 404,
    )?;
    let env = environment(&stub)?;
    let value = call(&env, &[])?;
    if value.pointer("/result/candidates/0/sections/erepo/state") != Some(&json!("unavailable"))
        || value.pointer("/result/candidates/0/sections/erepo/reason") != Some(&json!("bad_reply"))
    {
        return Err(format!("unprovedroute404becameempty: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("route404unexpectedretry".into());
    }
    Ok(())
}
fn wrong_identity() -> Result<(), String> {
    let stub = Stub::for_source("erepo.clinicalgenome.org", "application/json", |_| {
        Ok(body("CA123643"))
    })?;
    let env = environment(&stub)?;
    for _ in 0..2 {
        let value = call(&env, &[])?;
        if value.pointer("/result/candidates/0/sections/erepo/reason") != Some(&json!("bad_reply"))
        {
            return Err(format!("wrongCAidattached: {value}"));
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("wrongCAidenteredcache".into());
    }
    Ok(())
}

fn history(wrong: bool) -> Result<(), String> {
    let stub = Stub::for_source_status(
        "erepo.clinicalgenome.org",
        "application/json",
        move |request| {
            if request.starts_with("GET /evrepo/api/summary/classifications?") {
                let mut summary: Value =
                    serde_json::from_str(&body("CA000434")).map_err(|e| e.to_string())?;
                *summary
                    .pointer_mut("/data/0/uuid")
                    .ok_or("synthetic summary row")? =
                    json!("8d32df42-16ac-41f2-9bf1-e2b9ce206190");
                return Ok(summary.to_string());
            }
            if !request.starts_with("GET /evrepo/api/summary/classification/8d32df42-16ac-41f2-9bf1-e2b9ce206190/doc/sepio/version/1.0.0 HTTP/1.1") {
            return Err(format!("unexpected historical source request: {request}"));
        }
            Ok(json!({"data":{
            "uuid":"8d32df42-16ac-41f2-9bf1-e2b9ce206190",
            "id":"CG-PCER-VARINT:synthetic-history",
            "type":"VariantPathogenicityInterpretation",
            "metadata":{"version":if wrong {"1.1.0"} else {"1.0.0"},"created":"2000-01-01"},
            "variant":{"id":"CAR:CA000434"},
            "condition":{"label":"Synthetic historical condition"},
            "statementOutcome":{"label":"Pathogenic","id":"LN:LA6668-3"},
            "evidenceLine":[{"evidenceItem":[{"type":"CriterionAssessment","criterion":{"label":"PM2","defaultStrength":{"label":"Pathogenic Supporting"}},"statementOutcome":{"label":"Met","@id":"SEPIO:0000224"}}]}]
        }}).to_string())
        },
        |_| 200,
    )?;
    let env = environment(&stub)?;
    let args = [
        "variant",
        "assertions",
        "CA000434",
        "--source",
        "erepo",
        "--assertion",
        "PCER000001171",
        "--version",
        "1.0.0",
    ]
    .map(String::from);
    for _ in 0..2 {
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("{e}: {}", String::from_utf8_lossy(&output.stdout)))?;
        if wrong {
            if value.pointer("/sources/0/reason") != Some(&json!("bad_reply")) {
                return Err(format!("wrong historical version accepted: {value}"));
            }
        } else {
            let row = value
                .pointer("/result/assertions/0")
                .ok_or_else(|| format!("no historical row: {value}"))?;
            if row.pointer("/statement/attribution/version") != Some(&json!("1.0.0"))
                || row.pointer("/statement/condition/label")
                    != Some(&json!("Synthetic historical condition"))
                || row.pointer("/statement/attribution/submitter").is_some()
                || row.pointer("/statement/attribution/evaluated").is_some()
                || row.pointer("/statement/criteria").is_some()
                || row.pointer("/detail/document/assessments/0/outcome_iri")
                    != Some(&json!("SEPIO:0000224"))
            {
                return Err(format!(
                    "current summary facts contaminated historical document: {value}"
                ));
            }
        }
    }
    let requests = stub.finish()?;
    if requests.len() != if wrong { 3 } else { 2 } {
        return Err(format!(
            "historical cache admission or warm walk wrong: {requests:?}"
        ));
    }
    Ok(())
}
