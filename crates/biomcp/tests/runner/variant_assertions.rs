//! Source assertion identity, source-local absence and cache admission through the CLI.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "variant/assertions-erepo-invalid-document-never-caches",
            || history(2).map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-erepo-invalid-fields-never-cache",
            || invalid_fields().map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-synthetic-drug-response-stays-native",
            || drug_response().map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-current-version-keeps-summary-panel",
            || current_version().map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-disabled-before-identity-and-join",
            || disabled().map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-native-row-walk-preserves-unmapped",
            || native_walk().map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-erepo-selected-version-uses-only-document",
            || history(0).map_err(Into::into),
        ),
        Trial::test(
            "variant/assertions-erepo-wrong-version-is-never-cached",
            || history(1).map_err(Into::into),
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
    json!(
        {"status":{"code":200,"name":"OK"},"data":[{"caId":caid,"PCERDocID":"PCER000001171","docVersion":"1.1.0",
        "classification":"Pathogenic","condition":"Li-Fraumeni syndrome","mondoId":"MONDO:0018875","ep":"TP53 VCEP","gene":"TP53",
        "metCodes":["PP4_Moderate","PM2_Supporting","PS2","PS3","PS4","PM1","PP3"],"unMetCodes":["BS1"],"publishedDate":"2024-08-05",
        "approvedDate":"2024-08-05","versionsList":["1.1.0","1.0.0"],
        "summaryDesc":"Synthetic public-fixture payload; structured PS4 and source prose PS4_Very Strong remain separate."}]}
    )
    .to_string()
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

fn invalid_fields() -> Result<(), String> {
    for (field, invalid) in [("approvedDate", "invalid"), ("docVersion", "")] {
        let mut count = 0;
        let stub = Stub::for_source_status(
            "erepo.clinicalgenome.org",
            "application/json",
            move |_| {
                count += 1;
                let mut reply: Value =
                    serde_json::from_str(&body("CA000434")).map_err(|e| e.to_string())?;
                if count == 1 {
                    *reply
                        .pointer_mut(&format!("/data/0/{field}"))
                        .ok_or("missing synthetic field")? = json!(invalid);
                }
                Ok(reply.to_string())
            },
            |_| 200,
        )?;
        let env = environment(&stub)?;
        let bad = call(&env, &[])?;
        if bad.pointer("/sources/0/reason") != Some(&json!("bad_reply")) {
            return Err(format!("invalid {field} accepted: {bad}"));
        }
        let recovered = call(&env, &[])?;
        if recovered.pointer(
            "/result/candidates/0/sections/erepo/data/assertions/0/statement/attribution/version",
        ) != Some(&json!("1.1.0"))
            || stub.finish()?.len() != 2
        {
            return Err(format!(
                "invalid {field} persisted instead of recovering: {recovered}"
            ));
        }
    }
    Ok(())
}

fn history(mode: u8) -> Result<(), String> {
    let wrong = mode != 0;
    let stub = history_stub(mode)?;
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
                return Err(format!(
                    "wrong historical version accepted: {value}; wire={:?}",
                    stub.finish()
                ));
            }
        } else {
            let Some(row) = value.pointer("/result/assertions/0") else {
                return Err(format!(
                    "no historical row: {value}; wire={:?}",
                    stub.finish()
                ));
            };
            if row.pointer("/statement/attribution/version") != Some(&json!("1.0.0"))
                || row.pointer("/statement/condition/label")
                    != Some(&json!("Synthetic historical condition"))
                || row.pointer("/statement/attribution/submitter").is_some()
                || row.pointer("/statement/attribution/evaluated").is_some()
                || row.pointer("/statement/criteria").is_some()
                || row.pointer("/detail/document/assessments/0/outcome_iri")
                    != Some(&json!("SEPIO:0000224"))
                || row.pointer("/detail/document/assessments/0/outcome_conflict")
                    != Some(&json!(true))
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

fn native_walk() -> Result<(), String> {
    let stub = Stub::for_source(
        "eutils.ncbi.nlm.nih.gov",
        "application/xml",
        native_walk_body,
    )?;
    let env = environment(&stub)?;
    let mut args = [
        "get",
        "variant",
        "clinvar:128042",
        "clinvar",
        "--section-limit",
        "1",
    ]
    .map(String::from)
    .to_vec();
    let mut records = Vec::new();
    for offset in 0..4 {
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if super::mcp::call_with_globals(&env, &args, false, &[])? != value {
            return Err("native assertion CLI/MCP mismatch".into());
        }
        records.push(native_row(&value, offset)?);
        let page = value
            .get("next")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|n| n.get("kind") == Some(&json!("page")));
        if offset < 3 {
            args = serde_json::from_value(
                page.and_then(|p| p.get("args"))
                    .cloned()
                    .ok_or("missing runnable assertion continuation")?,
            )
            .map_err(|e| e.to_string())?;
            if args.first().map(String::as_str) != Some("variant")
                || args.get(1).map(String::as_str) != Some("assertions")
            {
                return Err(format!("invalid continuation spelling: {args:?}"));
            }
        } else if page.is_some() {
            return Err("unexpected fifth assertion page".into());
        }
    }
    if records
        != [
            json!("VCV000128042"),
            json!("SCV000611258"),
            json!("SCV000601142"),
            json!("SCV000607207"),
        ]
        || stub.finish()?.len() != 1
    {
        return Err(format!(
            "native rows merged, reordered or requested again: {records:?}"
        ));
    }
    Ok(())
}

fn disabled() -> Result<(), String> {
    let stub = Stub::for_source("reg.genome.network", "application/json", |_| {
        Err("disabled source attempted HTTP".into())
    })?;
    let env = environment(&stub)?;
    for (input, source) in [("CA000434", "clinvar"), ("rs113488022", "erepo")] {
        let args = ["get", "variant", input, source, "--disable-sources", source].map(String::from);
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let section = value.pointer(&format!("/result/candidates/0/sections/{source}"));
        if section.and_then(|s| s.get("state")) != Some(&json!("skipped"))
            || section.and_then(|s| s.get("reason")) != Some(&json!("disabled"))
            || value.pointer("/sources/0/state") != Some(&json!("skipped"))
            || value.pointer("/result/candidates/0/identity/ids").is_some()
        {
            return Err(format!(
                "disabled assertion source resolved identity or joined: {value}"
            ));
        }
        if super::mcp::call_with_globals(&env, &args, false, &[])? != value {
            return Err("disabled assertion CLI/MCP mismatch".into());
        }
    }
    if !stub.finish()?.is_empty() {
        return Err("disabled assertion source attempted provider work".into());
    }
    Ok(())
}

fn history_stub(mode: u8) -> Result<Stub, String> {
    Stub::for_source_status(
        "erepo.clinicalgenome.org",
        "application/json",
        move |request| {
            if request.starts_with("GET /evrepo/api/summary/classifications?") {
                let mut summary: Value =
                    serde_json::from_str(&body("CA000434")).map_err(|e| e.to_string())?;
                summary
                    .pointer_mut("/data/0")
                    .and_then(Value::as_object_mut)
                    .ok_or("synthetic summary row")?
                    .insert("uuid".into(), json!("8d32df42-16ac-41f2-9bf1-e2b9ce206190"));
                return Ok(summary.to_string());
            }
            if !request.starts_with("GET /evrepo/api/summary/classification/8d32df42-16ac-41f2-9bf1-e2b9ce206190/doc/sepio/version/1.0.0 HTTP/1.1") {
                return Err(format!("unexpected historical source request: {request}"));
            }
            Ok(json!(
        {"data":{ "uuid":"8d32df42-16ac-41f2-9bf1-e2b9ce206190","id":"CG-PCER-VARINT:synthetic-history",
        "type":"VariantPathogenicityInterpretation","metadata":{"version":if mode == 1 {"1.1.0"} else {"1.0.0"},"created":"2000-01-01"},
        "variant":{"id":"CAR:CA000434"},"condition":{"label":"Synthetic historical condition"},"statementOutcome":{"label":"Pathogenic",
        "id":"LN:LA6668-3"},"evidenceLine":[{"evidenceItem":[{"type":"CriterionAssessment",
        "criterion":{"label":if mode == 2 {""} else {"PM2"},"id":"0030","@id":"https://obofoundry.org/sepio/clingen/0030",
        "defaultStrength":{"label":"Pathogenic Supporting","id":"SEPIO:0000329","@id":"https://obofoundry.org/sepio/SEPIO:0000329"}},
        "statementOutcome":{"label":"Met","@id":"SEPIO:0000224"}}]}] }}
    )
            .to_string())
        },
        |_| 200,
    )
}

fn native_walk_body(request: &str) -> Result<String, String> {
    let xml = super::variant_assertion_wire::clinvar_body(request)?;
    // Explicit synthetic XML retains the public record's native not-provided classification.
    Ok(xml.replace(
        "</ClinicalAssertionList>",
        "<ClinicalAssertion><ClinVarAccession Accession=\"SCV000601142\" \
        Version=\"6\"/><Classification><GermlineClassification>pathogenic</GermlineClassification></Classifi\
        cation></ClinicalAssertion><ClinicalAssertion><ClinVarAccession Accession=\"SCV000607207\" \
        Version=\"3\" SubmitterName=\"GenomeConnect, ClinGen\"/><Classification><ReviewStatus>no \
        classification provided</ReviewStatus><GermlineClassification>not \
        provided</GermlineClassification></Classification></ClinicalAssertion></ClinicalAssertionList>",
    ))
}

fn native_row(value: &Value, offset: u32) -> Result<Value, String> {
    let data = value
        .pointer("/result/candidates/0/sections/clinvar/data")
        .or_else(|| value.get("result"))
        .ok_or("missing assertion body")?;
    if data.pointer("/page/offset") != Some(&json!(offset))
        || data.pointer("/page/total") != Some(&json!(4))
    {
        return Err(format!("native assertion page lost bounds: {value}"));
    }
    let row = data
        .pointer("/assertions/0")
        .ok_or("missing native assertion")?;
    let record = row
        .pointer("/statement/attribution/record")
        .or_else(|| row.pointer("/unmapped/attribution/record"))
        .ok_or("missing source record")?;
    if offset == 3
        && (row.get("statement_unmapped") != Some(&json!(true))
            || row.pointer("/unmapped/classification") != Some(&json!("not provided"))
            || row.get("statement").is_some()
            || value.get("problems").is_some())
    {
        return Err(format!(
            "unsupported category invented a proposition or problem: {value}"
        ));
    }
    if offset == 2
        && (row.pointer("/statement/classification/label") != Some(&json!("pathogenic"))
            || row.pointer("/statement/proposition") != Some(&json!("pathogenicity"))
            || row.pointer("/statement/classification/coding").is_some())
    {
        return Err(format!(
            "native casing changed label or inferred framework: {value}"
        ));
    }
    Ok(record.clone())
}

fn current_version() -> Result<(), String> {
    let stub = Stub::for_source("erepo.clinicalgenome.org", "application/json", |_| {
        Ok(body("CA000434"))
    })?;
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
        "1.1.0",
    ]
    .map(String::from);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if value.pointer("/result/assertions/0/statement/attribution/submitter")
        != Some(&json!("TP53 VCEP"))
        || value.pointer("/result/assertions/0/statement/attribution/version")
            != Some(&json!("1.1.0"))
        || value
            .pointer("/result/assertions/0/statement/criteria")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(7)
        || value
            .pointer("/result/assertions/0/statement/attribution/evaluated")
            .is_some()
        || super::mcp::call_with_globals(&env, &args, false, &[])? != value
    {
        return Err(format!("exact current summary details changed: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("current version fetched an unnecessary historical document".into());
    }
    Ok(())
}

// Response-only synthetic label edit. This does not claim a real drug-response assertion.
fn drug_response_body(request: &str) -> Result<String, String> {
    native_walk_body(request).map(|xml| {
        xml.replace(
            "<GermlineClassification>not provided</GermlineClassification>",
            "<GermlineClassification>drug response</GermlineClassification>",
        )
    })
}
fn drug_response() -> Result<(), String> {
    let stub = Stub::for_source(
        "eutils.ncbi.nlm.nih.gov",
        "application/xml",
        drug_response_body,
    )?;
    let env = environment(&stub)?;
    let args = [
        "variant",
        "assertions",
        "clinvar:128042",
        "--source",
        "clinvar",
        "--assertion",
        "SCV000607207",
        "--version",
        "3",
    ]
    .map(String::from);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let row = value
        .pointer("/result/assertions/0")
        .ok_or("missing synthetic drug row")?;
    if row.get("statement_unmapped") != Some(&json!(true))
        || row.get("statement").is_some()
        || row.pointer("/unmapped/classification") != Some(&json!("drug response"))
        || row.pointer("/unmapped/attribution/record") != Some(&json!("SCV000607207"))
        || row.pointer("/unmapped/attribution/version") != Some(&json!("3"))
        || row.pointer("/unmapped/attribution/submitter") != Some(&json!("GenomeConnect, ClinGen"))
        || row.pointer("/unmapped/attribution/evaluated").is_some()
        || value
            .get("problems")
            .is_some_and(|p| p.as_array().is_none_or(|p| !p.is_empty()))
    {
        return Err(format!(
            "synthetic drug-response row changed native context: {value}"
        ));
    }
    if super::mcp::call_with_globals(&env, &args, false, &[])? != value {
        return Err("synthetic drug-response CLI/MCP mismatch".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("synthetic drug-response cache was not reused".into());
    }
    Ok(())
}
