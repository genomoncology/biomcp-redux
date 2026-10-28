//! Feature drug-next-steps: source-listed EGFR context captured once on 2026-10-25.
//! Combined-source capture is projected to each product's explicit one-stream selector.
use crate::runner::transcript::trial as transcript_trial;
use super::super::{launch::Environment, network_stub::Stub, proof};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
const BODY: &str = include_str!("../../../../../tests/fixtures/drug/civic/next-context.body");
const META: &str =
    include_str!("../../../../../tests/fixtures/drug/civic/next-context.provenance.json");
pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "drug-next-steps/native-d05-source-listed-lookup",
        || run().map_err(Into::into),
    )]
}
fn run() -> Result<(), String> {
    use sha2::{Digest, Sha256};
    let root = evidence_root()?;
    fs::write(root.join("original-capture.body"), BODY).map_err(|e| e.to_string())?;
    fs::write(root.join("original-capture.json"), META).map_err(|e| e.to_string())?;
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
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    for stream in ["evidence", "assertions"] {
        check(&env, stream, &root)?;
    }
    let requests = stub.finish()?;
    fs::write(
        root.join("local-deliveries.json"),
        serde_json::to_vec_pretty(&requests).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if requests.len() != 2 {
        return Err(format!(
            "expected two local deliveries, received {}",
            requests.len()
        ));
    }
    Ok(())
}
fn check(env: &Environment, stream: &str, root: &Path) -> Result<(), String> {
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
    let recording = root.join(format!("recorded-{stream}"));
    let mut record = vec!["--record".into(), recording.display().to_string()];
    record.extend_from_slice(&args);
    record.push("--fresh".into());
    let result = proof::retained_cli(env, &record, 0, &root.join(format!("record-cli-{stream}")))?;
    let mut replay_args = vec!["--replay".into(), recording.display().to_string()];
    replay_args.extend_from_slice(&args);
    // Freshness belongs to each request, including the MCP tool invocation.
    replay_args.push("--fresh".into());
    let replay =
        proof::retained_both(env, &replay_args, 0, &root.join(format!("replay-{stream}")))?;
    for key in ["result", "next", "query", "status"] {
        if result.get(key) != replay.get(key) {
            return Err(format!("native record/replay differ at {key}"));
        }
    }
    verify(&result, stream)?;
    let size = super::super::source_terms::size(&result, "")?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "NATIVE_DRUG_NEXT_SIZE stream={stream} full_bytes={} comparable_bytes={size}",
        result.to_string().len()
    )
    .map_err(|e| e.to_string())?;
    // Each observed envelope has 15 percent headroom; publisher terms are reported separately.
    let (full_cap, comparable_cap) = if stream == "evidence" {
        (3116, 3019)
    } else {
        (2907, 2810)
    };
    if result.to_string().len() > full_cap || size > comparable_cap {
        return Err("native lookup exceeded measured envelope ceilings".into());
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

fn evidence_root() -> Result<PathBuf, String> {
    let parent = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/native-proof/drug-next");
    fs::create_dir_all(&parent).map_err(|e| e.to_string())?;
    let root = tempfile::Builder::new()
        .prefix("d05-")
        .tempdir_in(parent)
        .map_err(|e| e.to_string())?
        .keep();
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "NATIVE_DRUG_NEXT_PROOF={}",
        root.display()
    )
    .map_err(|e| e.to_string())?;
    Ok(root)
}
