//! Feature drug-next-steps: source-listed EGFR context captured once on 2026-10-25.
//! Combined-source capture is projected to each product's explicit one-stream selector.
use super::super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
const BODY: &str = include_str!("../../../../../tests/fixtures/drug/civic/next-context.body");
const META: &str =
    include_str!("../../../../../tests/fixtures/drug/civic/next-context.provenance.json");
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "drug-next-steps/native-d05-source-listed-lookup",
        || run().map_err(Into::into),
    )]
}
fn run() -> Result<(), String> {
    use sha2::{Digest, Sha256};
    let meta: Value = serde_json::from_str(META).map_err(|e| e.to_string())?;
    let hash: String = Sha256::digest(BODY.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    if meta.pointer("/capture/sha256") != Some(&json!(hash)) {
        return Err("native body hash changed".into());
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source("civicdb.org", "application/json", reply)?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    fs::write(env.root().join("original-capture.body"), BODY).map_err(|e| e.to_string())?;
    fs::write(env.root().join("original-capture.json"), META).map_err(|e| e.to_string())?;
    for stream in ["evidence", "assertions"] {
        check(&env, stream)?;
    }
    let requests = stub.finish()?;
    if requests.len() != 2 {
        return Err(format!(
            "expected two local deliveries, received {}",
            requests.len()
        ));
    }
    Ok(())
}
fn check(env: &Environment, stream: &str) -> Result<(), String> {
    let args = [
        "get",
        "drug",
        "187",
        "civic",
        "--source",
        "civic",
        "--civic-profile",
        "34",
        "--civic-kind",
        stream,
    ]
    .map(str::to_owned);
    let recording = env.root().join(format!("recorded-{stream}"));
    let mut record = args.to_vec();
    record.extend(["--record".into(), recording.display().to_string()]);
    let result = invoke(env, record, 0)?;
    super::super::schema::Registry::embedded()?.validate(&result, &args, true)?;
    verify(&result, stream)?;
    fs::write(
        env.root().join(format!("{stream}.json")),
        result.to_string(),
    )
    .map_err(|e| e.to_string())?;
    let globals = vec![
        "--replay".into(),
        recording.display().to_string(),
        "--fresh".into(),
    ];
    let mut replay_args = args.to_vec();
    replay_args.extend(globals.clone());
    let replay = invoke(env, replay_args, 0)?;
    let mcp = super::super::mcp::call_with_globals(env, &args, false, &globals)?;
    for key in ["result", "next", "query", "status"] {
        if result.get(key) != replay.get(key) || result.get(key) != mcp.get(key) {
            return Err(format!("native CLI/recorder/MCP differ at {key}"));
        }
    }
    let size = super::super::source_terms::size(&result, "")?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "NATIVE_DRUG_NEXT_SIZE stream={stream} full_bytes={} comparable_bytes={size}",
        result.to_string().len()
    )
    .map_err(|e| e.to_string())?;
    if size > 8000 {
        return Err("native lookup exceeded initial bounded ceiling".into());
    }
    Ok(())
}
fn verify(result: &Value, stream: &str) -> Result<(), String> {
    let claim = result
        .pointer(&format!("/result/civic/data/{stream}/records/0"))
        .ok_or("native claim")?;
    let expected = if stream == "evidence" {
        "EID1592"
    } else {
        "AID130"
    };
    for (path, expected) in [
        ("/claim/record/id", json!(expected)),
        ("/profile/record/id", json!("34")),
        ("/oncokb_lookup/member", json!("34")),
        (
            "/oncokb_lookup/query/input",
            json!("NC_000007.13:g.55249071C>T"),
        ),
        ("/oncokb_lookup/query/build", json!("GRCh37")),
        ("/profile/members/0/feature", json!("EGFR")),
    ] {
        if claim.pointer(path) != Some(&expected) {
            return Err(format!("native context changed at {path}: {claim}"));
        }
    }
    if claim.get("oncokb_missing").is_some() {
        return Err("complete lookup context marked missing".into());
    }
    let native: Value = serde_json::from_str(BODY).map_err(|e| e.to_string())?;
    let field = if stream == "evidence" {
        "evidenceItems"
    } else {
        "assertions"
    };
    let tumor = native.pointer(&format!("/data/{field}/nodes/0/disease/name"));
    if claim.pointer("/oncokb_lookup/query/tumor_type") != tumor {
        return Err("native tumor changed".into());
    }
    Ok(())
}
fn reply(request: &str) -> Result<String, String> {
    let (_, body) = request.split_once("\r\n\r\n").ok_or("POST body")?;
    let request: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    let query = request
        .get("query")
        .and_then(Value::as_str)
        .ok_or("GraphQL query")?;
    if request.pointer("/variables/id") != Some(&json!(187))
        || !query.contains("molecularProfileId:34")
    {
        return Err("native delivery received an unproved identity/profile selector".into());
    }
    let mut body: Value = serde_json::from_str(BODY).map_err(|e| e.to_string())?;
    for field in ["evidenceItems", "assertions"] {
        if !query.contains(&format!("{field}(")) {
            body.get_mut("data")
                .and_then(Value::as_object_mut)
                .ok_or("source data")?
                .remove(field);
        }
    }
    serde_json::to_string(&body).map_err(|e| e.to_string())
}
