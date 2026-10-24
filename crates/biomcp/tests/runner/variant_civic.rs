//! CIViC exact membership crosses the real CLI, source policy, POST body and TLS cache.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "variant/civic-exact-profile-membership",
        || exact().map_err(Into::into),
    )]
}
fn configure(environment: &Environment, stub: &Stub) -> Result<(), String> {
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())
}
fn run(environment: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|a| (*a).into()).collect::<Vec<_>>();
    let out = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("{e}: {}", String::from_utf8_lossy(&out.stdout)))
}
fn exact() -> Result<(), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let civic = Stub::for_source("civicdb.org", "application/json", civic_body)?;
    configure(&environment, &civic)?;
    let value = run(
        &environment,
        &[
            "get",
            "variant",
            "CA123643",
            "civic",
            "--disable-sources",
            "myvariant",
        ],
    )?;
    if value.pointer("/result/candidates/0/sections/civic/data/profiles/0/membership")
        != Some(&json!("exact"))
    {
        return Err(format!("exact source profile not retained: {value}"));
    }
    check_claims(&value)?;
    walk_streams(&environment, &value)?;
    let args = [
        "get",
        "variant",
        "CA123643",
        "civic",
        "--disable-sources",
        "myvariant",
    ]
    .map(Into::into);
    let mcp = super::mcp::call_with_globals(&environment, &args, false, &[])?;
    if mcp.get("result") != value.get("result") {
        return Err("CIViC CLI/MCP source mapping differs".into());
    }
    let requests = civic.finish()?;
    let (_, request) = requests
        .first()
        .filter(|_| requests.len() == 3)
        .ok_or("expected default and two native stream requests")?;
    let (_, body) = request.split_once("\r\n\r\n").ok_or("missing POST body")?;
    let body: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if !request.starts_with("POST /api/graphql HTTP/1.1")
        || body.pointer("/variables/allele") != Some(&json!("CA123643"))
        || !body
            .get("query")
            .and_then(Value::as_str)
            .is_some_and(|q| q.contains("alleleRegistryId"))
    {
        return Err(format!(
            "CIViC request did not use checked allele membership: {body}"
        ));
    }
    Ok(())
}
fn check_claims(value: &Value) -> Result<(), String> {
    let base = "/result/candidates/0/sections/civic/data/profiles/0";
    for (path, expected) in [
        ("/statements/0/statement/subject/level", json!("protein")),
        (
            "/statements/0/statement/proposition",
            json!("therapeutic_response"),
        ),
        (
            "/statements/0/statement/predicate",
            json!("predicts_resistance_to"),
        ),
        ("/statements/0/statement/direction", json!("disputes")),
        ("/statements/0/direction", json!("DOES_NOT_SUPPORT")),
        (
            "/statements/1/statement/classification/coding/code",
            json!("tier_i"),
        ),
        (
            "/statements/1/statement/strength/coding/code",
            json!("level_a"),
        ),
        ("/evidence_page/next/civic_profile", json!(12)),
        ("/evidence_page/next/civic_kind", json!("evidence")),
        ("/assertion_page/next/civic_kind", json!("assertions")),
    ] {
        if value.pointer(&format!("{base}{path}")) != Some(&expected) {
            return Err(format!("source claim changed at {path}: {value}"));
        }
    }
    Ok(())
}
fn walk_streams(environment: &Environment, value: &Value) -> Result<(), String> {
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("source continuations missing")?;
    for (kind, expected) in [("evidence", "EID1410"), ("assertions", "AID8")] {
        let args = next
            .iter()
            .filter_map(|row| row.get("args").and_then(Value::as_array))
            .find(|args| args.iter().any(|v| v.as_str() == Some(kind)))
            .ok_or("native stream action missing")?;
        let args = args
            .iter()
            .map(|v| v.as_str().ok_or("nontext action argument"))
            .collect::<Result<Vec<_>, _>>()?;
        let page = run(environment, &args)?;
        let base = "/result/candidates/0/sections/civic/data/profiles/0";
        if page.pointer(&format!("{base}/statements/0/record/id")) != Some(&json!(expected)) {
            return Err(format!(
                "native {kind} continuation repeated or dropped a row: {page}"
            ));
        }
        let path = format!(
            "{base}/{}_page/next",
            if kind == "assertions" {
                "assertion"
            } else {
                "evidence"
            }
        );
        if page.pointer(&path).is_some() {
            return Err("completed native stream offers another page".into());
        }
    }
    Ok(())
}
fn civic_body(request: &str) -> Result<String, String> {
    let evidence = json!({"totalCount":2,"nodes":[{"id":1409,"status":"ACCEPTED","evidenceType":"PREDICTIVE","evidenceLevel":"A","evidenceDirection":"DOES_NOT_SUPPORT","significance":"RESISTANCE","disease":{"name":"melanoma","doid":"1909"},"therapies":[{"name":"dabrafenib"}],"source":{"citationId":"25741868","sourceType":"PUBMED"}}],"pageInfo":{"hasNextPage":true,"endCursor":"synthetic-e1"}});
    let assertions = json!({"totalCount":2,"nodes":[{"id":7,"status":"ACCEPTED","assertionType":"PREDICTIVE","assertionDirection":"SUPPORTS","significance":"SENSITIVITYRESPONSE","ampLevel":"TIER_I_LEVEL_A","disease":{"name":"melanoma","doid":"1909"},"therapies":[{"name":"dabrafenib"},{"name":"trametinib"}],"variantOrigin":"SOMATIC"}],"pageInfo":{"hasNextPage":true,"endCursor":"synthetic-a1"}});
    let mut reply = json!({"data":{"molecularProfiles":{"totalCount":1,"nodes":[{"id":12,"name":"BRAF V600E","link":"/molecular-profiles/12","isComplex":false,"variants":[{"id":12,"name":"V600E","link":"/variants/12","alleleRegistryId":"CA123643","hgvsDescriptions":["NP_004324.2:p.Val600Glu"],"feature":{"name":"BRAF"}}],"evidenceItems":evidence,"assertions":assertions}],"pageInfo":{"hasNextPage":false,"endCursor":null}}} });
    let (_, body) = request
        .split_once("\r\n\r\n")
        .ok_or("request body missing")?;
    let request: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    let base = "/data/molecularProfiles/nodes/0";
    let empty = json!({"totalCount":2,"nodes":[],"pageInfo":{"hasNextPage":true,"endCursor":null}});
    if request
        .pointer("/variables/evidence_after")
        .and_then(Value::as_str)
        .is_some()
    {
        *reply
            .pointer_mut(&format!("{base}/evidenceItems/nodes/0/id"))
            .ok_or("evidence id")? = json!(1410);
        *reply
            .pointer_mut(&format!("{base}/evidenceItems/pageInfo/hasNextPage"))
            .ok_or("evidence page")? = json!(false);
        *reply
            .pointer_mut(&format!("{base}/assertions"))
            .ok_or("assertions")? = empty;
    } else if request
        .pointer("/variables/assertion_after")
        .and_then(Value::as_str)
        .is_some()
    {
        *reply
            .pointer_mut(&format!("{base}/assertions/nodes/0/id"))
            .ok_or("assertion id")? = json!(8);
        *reply
            .pointer_mut(&format!("{base}/assertions/pageInfo/hasNextPage"))
            .ok_or("assertion page")? = json!(false);
        *reply
            .pointer_mut(&format!("{base}/evidenceItems"))
            .ok_or("evidence")? = empty;
    }
    Ok(reply.to_string())
}
