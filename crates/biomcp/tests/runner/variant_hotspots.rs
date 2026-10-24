//! Native residue counts remain candidates when transcript equivalence is not established.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant/hotspots-retains-native-transcript-and-counts",
        || candidate().map_err(Into::into),
    )]
}
fn candidate() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["myvariant.info", "www.cancerhotspots.org"],
        "application/json",
        body,
    )?;
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    let args = [
        "get",
        "variant",
        "chr7:g.140753336A>T",
        "--build",
        "GRCh38",
        "hotspots",
    ]
    .map(Into::into);
    let out = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    eprintln!(
        "Hotspots candidate envelope bytes: {}",
        serde_json::to_vec(&value).map_err(|e| e.to_string())?.len()
    );
    let base = "/result/candidates/0/sections/hotspots/data";
    for (path, expected) in [
        ("/reason", json!("no_join_id")),
        ("/records/0/membership", json!("candidate")),
        ("/records/0/transcript", json!("ENST00000288602")),
        ("/records/0/residue", json!("V600")),
        ("/records/0/tumor_count", json!(12)),
        ("/records/0/alternates/0/count", json!(8)),
    ] {
        if value.pointer(&format!("{base}{path}")) != Some(&expected) {
            return Err(format!("native hotspot candidate lost at{path}: {value}"));
        }
    }
    let mcp = super::mcp::call_with_globals(&environment, &args, false, &[])?;
    if mcp.get("result") != value.get("result") {
        return Err("hotspots CLI/MCP source mapping differs".into());
    }
    let requests = stub.finish()?;
    if requests.len() != 2
        || !requests.iter().any(|(_, request)| {
            request.starts_with("GET /api/hotspots/single/byGene/BRAF HTTP/1.1")
        })
    {
        return Err("hotspots did not use its declared bounded gene endpoint".into());
    }
    Ok(())
}
fn body(request: &str) -> Result<String, String> {
    if request.starts_with("GET /v1/variant/") {
        Ok(json!({"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
    } else {
        // Counts are deliberately synthetic; provider data rights remain unverified.
        Ok(json!([{"hugoSymbol":"BRAF","transcriptId":"ENST00000288602","residue":"V600","tumorCount":12,"variantAminoAcid":{"E":8,"K":4}}]).to_string())
    }
}
