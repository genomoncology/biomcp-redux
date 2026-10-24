//! Source assertion identity, source-local absence and cache admission through the CLI.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
pub(super) fn trials() -> Vec<Trial> {
    vec![
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
    let empty = Arc::new(AtomicBool::new(false));
    let body_empty = empty.clone();
    let status_empty = empty.clone();
    let stub = Stub::for_source_status(
        "erepo.clinicalgenome.org",
        "application/json",
        move |_| {
            Ok(if body_empty.load(Ordering::SeqCst) {
                json!({"status":{"code":404,"name":"No Records Found"}}).to_string()
            } else {
                body("CA000434")
            })
        },
        move |_| {
            if status_empty.load(Ordering::SeqCst) {
                404
            } else {
                200
            }
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
    empty.store(true, Ordering::SeqCst);
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
