//! CIViC exact membership crosses the real CLI, source policy, POST body and TLS cache.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
#[path = "civic_native_profile.rs"]
mod native_profile;
#[path = "civic_values.rs"]
mod values;
#[path = "civic_vocabulary.rs"]
mod vocabulary;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("drug-civic/native-boolean-profile-context", || {
            native_profile::context().map_err(Into::into)
        }),
        transcript_trial(
            "variant/civic-invalid-values-keep-identified-claims",
            || values::claims().map_err(Into::into),
        ),
        transcript_trial("variant/civic-amp-na-and-therapy-group-stay-native", || {
            values::amp_and_group().map_err(Into::into)
        }),
        transcript_trial("drug-civic/unknown-status-keeps-native-claims", || {
            vocabulary::drug_status().map_err(Into::into)
        }),
        transcript_trial(
            "variant/civic-unknown-stream-vocabulary-keeps-claims",
            || vocabulary::stream_context().map_err(Into::into),
        ),
        transcript_trial("variant/civic-unknown-vocabulary-keeps-claims", || {
            vocabulary::unknown().map_err(Into::into)
        }),
        transcript_trial("variant/civic-exact-profile-membership", || {
            exact().map_err(Into::into)
        }),
        transcript_trial("variant/civic-unjoined-profile-is-not-cached", || {
            unjoined().map_err(Into::into)
        }),
        transcript_trial("variant/civic-poisoned-warm-cache-is-rechecked", || {
            poisoned().map_err(Into::into)
        }),
        transcript_trial(
            "variant/civic-graphql-errors-retain-valid-uncached-facts",
            || graphql_errors().map_err(Into::into),
        ),
        transcript_trial("variant/civic-native-cursor-must-progress", || {
            stalled_cursor().map_err(Into::into)
        }),
        transcript_trial(
            "variant/civic-compound-profile-retains-all-native-members",
            || compound().map_err(Into::into),
        ),
    ]
}
fn configure(environment: &Environment, stub: &Stub) -> Result<(), String> {
    let ca = environment.root().join("synthetic-ca.pem");
    stub.retain(environment);
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
    if super::source_terms::size(&value, "")? > 4572 {
        return Err("CIViC default envelope exceeded its measured ceiling".into());
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
    let disabled = run(
        &environment,
        &[
            "get",
            "variant",
            "CA123643",
            "civic",
            "--disable-sources",
            "civic",
        ],
    )?;
    if disabled.pointer("/result/candidates/0/sections/civic/reason") != Some(&json!("disabled")) {
        return Err("disabled CIViC reused its warm source reply".into());
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
    for (_, request) in &requests {
        check_profile_filter(request)?;
    }
    Ok(())
}
fn check_profile_filter(request: &str) -> Result<(), String> {
    let (_, body) = request.split_once("\r\n\r\n").ok_or("missing POST body")?;
    let body: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    let query = body
        .get("query")
        .and_then(Value::as_str)
        .ok_or("missing query")?;
    let selected = body
        .pointer("/variables/evidence_after")
        .and_then(Value::as_str)
        .is_some()
        || body
            .pointer("/variables/assertion_after")
            .and_then(Value::as_str)
            .is_some();
    if selected {
        if body.pointer("/variables/ids") != Some(&json!([120]))
            || !query.contains("$ids:[Int!]")
            || !query.contains("ids:$ids")
        {
            return Err(format!(
                "selected molecular profile filter was dropped: {body}"
            ));
        }
    } else if body.pointer("/variables/ids").is_some() || query.contains("$ids") {
        return Err(format!(
            "absent molecular profile selection became an ids filter: {body}"
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
            "/statements/2/native/classification/codings/0/code",
            json!("tier_i"),
        ),
        (
            "/statements/2/native/strength/codings/0/code",
            json!("level_a"),
        ),
        ("/evidence_page/next/civic_profile", json!(120)),
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
    for (kind, expected) in [("evidence", "EID1411"), ("assertions", "AID8")] {
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
pub(super) fn civic_body(request: &str) -> Result<String, String> {
    let evidence = json!(
        {"totalCount":2,"nodes":[{"id":1409,"status":"ACCEPTED","evidenceType":"PREDICTIVE","evidenceLevel":"A",
        "evidenceDirection":"DOES_NOT_SUPPORT","significance":"RESISTANCE","disease":{"name":"melanoma","doid":"1909"},
        "therapies":[{"name":"dabrafenib"}],"therapyInteractionType":null,
        "source":{"citationId":"25741868","sourceType":"PUBMED"}}],"pageInfo":{"hasNextPage":true,
        "endCursor":"synthetic-e1"}}
    );
    let assertions = json!(
        {"totalCount":2,"nodes":[{"id":7,"status":"ACCEPTED","assertionType":"PREDICTIVE","assertionDirection":"SUPPORTS",
        "significance":"SENSITIVITYRESPONSE","ampLevel":"TIER_I_LEVEL_A","disease":{"name":"melanoma","doid":"1909"},
        "therapies":[{"name":"dabrafenib"},{"name":"trametinib"}],"therapyInteractionType":"COMBINATION",
        "variantOrigin":"SOMATIC"}],"pageInfo":{"hasNextPage":true,
        "endCursor":"synthetic-a1"}}
    );
    let mut reply = json!(
        {"data":{"molecularProfiles":{"totalCount":1,"nodes":[{"id":120,"name":"BRAF V600E","link":"/molecular-profiles/120",
        "isComplex":false,"variants":[{"id":12,"name":"V600E","link":"/variants/12","alleleRegistryId":"CA123643",
        "hgvsDescriptions":["NP_004324.2:p.Val600Glu"],"feature":{"name":"BRAF"}}],"evidenceItems":evidence,"assertions":assertions}],
        "pageInfo":{"hasNextPage":false,"endCursor":null}}} }
    );
    let evidence_base = "/data/molecularProfiles/nodes/0/evidenceItems";
    let mut second = reply
        .pointer(&format!("{evidence_base}/nodes/0"))
        .ok_or("first evidence row")?
        .clone();
    *second.get_mut("id").ok_or("second evidence ID")? = json!(9851);
    reply
        .pointer_mut(&format!("{evidence_base}/nodes"))
        .and_then(Value::as_array_mut)
        .ok_or("evidence rows")?
        .push(second);
    *reply
        .pointer_mut(&format!("{evidence_base}/totalCount"))
        .ok_or("evidence total")? = json!(3);
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
            .ok_or("evidence id")? = json!(1411);
        reply
            .pointer_mut(&format!("{base}/evidenceItems/nodes"))
            .and_then(Value::as_array_mut)
            .ok_or("evidence rows")?
            .truncate(1);
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

fn source_environment(
    body: fn(&str) -> Result<String, String>,
) -> Result<(Environment, Stub), String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let source = Stub::for_source("civicdb.org", "application/json", body)?;
    configure(&environment, &source)?;
    Ok((environment, source))
}
fn unjoined() -> Result<(), String> {
    let (environment, source) = source_environment(wrong_body)?;
    let recording = environment.root().join("record");
    fs::create_dir(&recording).map_err(|e| e.to_string())?;
    let recording = fs::canonicalize(recording).map_err(|e| e.to_string())?;
    let recording = recording.to_str().ok_or("recording path is not UTF-8")?;
    for attempt in 0..3 {
        let mut args = vec!["get", "variant", "CA123643", "civic"];
        if attempt == 0 {
            args.extend(["--record", recording]);
        }
        if attempt == 2 {
            args.extend(["--civic-profile", "99", "--civic-kind", "evidence"]);
        }
        let value = run(&environment, &args)?;
        let base = "/result/candidates/0/sections/civic/data/profiles/0";
        if value.pointer(&format!("{base}/membership")) != Some(&json!("candidate"))
            || value.pointer(&format!("{base}/statements")).is_some()
            || value
                .pointer(&format!("{base}/evidence_page/next"))
                .is_some()
        {
            return Err(format!(
                "unjoined native profile acquired allele claims: {value}"
            ));
        }
    }
    let replay = run(
        &environment,
        &["--replay", recording, "get", "variant", "CA123643", "civic"],
    )?;
    if replay.pointer("/result/candidates/0/sections/civic/data/profiles/0/membership")
        != Some(&json!("candidate"))
    {
        return Err("replayed source member mismatch became an exact claim".into());
    }
    let offline = run(
        &environment,
        &["--offline", "get", "variant", "CA123643", "civic"],
    )?;
    if offline.pointer("/sources/0/reason") != Some(&json!("offline"))
        || source.finish()?.len() != 3
    {
        return Err("unjoined profile entered or reused the source cache".into());
    }
    Ok(())
}
fn poisoned() -> Result<(), String> {
    let (environment, source) = source_environment(civic_body)?;
    run(&environment, &["get", "variant", "CA123643", "civic"])?;
    let path = environment.root().join(format!(
        "cache/biomcp/cache-{}-{}.sqlite",
        biomcp_fetch::CACHE_NAMESPACE_VERSION,
        biomcp_query::store::CACHE_NAMESPACE_VERSION
    ));
    let database = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
    let poison = wrong_body("POST /api/graphql HTTP/1.1\r\n\r\n{}")?;
    let rows = database
        .execute(
            "UPDATE reply SET body=?1 WHERE source='civic'",
            [poison.as_bytes()],
        )
        .map_err(|e| e.to_string())?;
    if rows != 1 {
        return Err("expected one source-owned cached POST reply".into());
    }
    drop(database);
    let value = run(&environment, &["get", "variant", "CA123643", "civic"])?;
    if value.pointer("/result/candidates/0/sections/civic/data/profiles/0/membership")
        != Some(&json!("exact"))
        || source.finish()?.len() != 2
    {
        return Err(format!("poisoned warm source identity was reused: {value}"));
    }
    Ok(())
}
fn compound() -> Result<(), String> {
    let (environment, source) = source_environment(compound_body)?;
    let value = run(&environment, &["get", "variant", "CA123643", "civic"])?;
    let base = "/result/candidates/0/sections/civic/data/profiles/0";
    if value.pointer(&format!("{base}/compound")) != Some(&json!(true))
        || value.pointer(&format!("{base}/statements/0/statement_unmapped")) != Some(&json!(true))
        || value
            .pointer(&format!("{base}/members"))
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(2)
        || value.pointer(&format!("{base}/members/1/feature")) != Some(&json!("KIAA1549::BRAF"))
        || value
            .pointer(&format!("{base}/statements/0/statement"))
            .is_some()
        || value.pointer(&format!("{base}/statements/0/native/therapies/0/label"))
            != Some(&json!("dabrafenib"))
    {
        return Err(format!(
            "compound OR profile was collapsed or its claim lost: {value}"
        ));
    }
    source.finish()?;
    Ok(())
}
fn wrong_body(request: &str) -> Result<String, String> {
    let mut raw: Value = serde_json::from_str(&civic_body(request)?).map_err(|e| e.to_string())?;
    let (_, body) = request
        .split_once("\r\n\r\n")
        .ok_or("missing request body")?;
    let request: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if request.pointer("/variables/ids/0") != Some(&json!(99)) {
        *raw.pointer_mut("/data/molecularProfiles/nodes/0/variants/0/alleleRegistryId")
            .ok_or("member identity")? = json!("CA999999");
    }
    Ok(raw.to_string())
}
fn compound_body(request: &str) -> Result<String, String> {
    let mut raw: Value = serde_json::from_str(&civic_body(request)?).map_err(|e| e.to_string())?;
    let base = "/data/molecularProfiles/nodes/0";
    *raw.pointer_mut(&format!("{base}/name"))
        .ok_or("profile name")? = json!("BRAF V600E OR KIAA1549::BRAF Fusion");
    *raw.pointer_mut(&format!("{base}/isComplex"))
        .ok_or("profile type")? = json!(true);
    raw.pointer_mut(&format!("{base}/variants"))
        .and_then(Value::as_array_mut)
        .ok_or("members")?
        .push(json!({"id":23, "name":"Fusion", "link":"/variants/23", "feature":{"name":"KIAA1549::BRAF"}}));
    Ok(raw.to_string())
}

fn graphql_errors() -> Result<(), String> {
    let (environment, source) = source_environment(error_body)?;
    for _ in 0..2 {
        let value = run(&environment, &["get", "variant", "CA123643", "civic"])?;
        if value.pointer("/result/candidates/0/sections/civic/data/profiles/0/membership")
            != Some(&json!("exact"))
            || value.get("status") != Some(&json!("partial"))
            || value.pointer("/problems/0/field") != Some(&json!("graphql.errors"))
        {
            return Err(format!(
                "GraphQL partial error lost valid facts or its partial notice: {value}"
            ));
        }
    }
    if source.finish()?.len() != 2 {
        return Err("GraphQL error reply entered source cache".into());
    }
    Ok(())
}
fn error_body(request: &str) -> Result<String, String> {
    let mut raw: Value = serde_json::from_str(&civic_body(request)?).map_err(|e| e.to_string())?;
    raw.as_object_mut().ok_or("GraphQL body")?.insert(
        "errors".into(),
        json!([{"message":"synthetic source error"}]),
    );
    Ok(raw.to_string())
}

fn stalled_cursor() -> Result<(), String> {
    let (environment, source) = source_environment(stalled_body)?;
    for cursor in ["synthetic-e1", "empty"] {
        let value = run(
            &environment,
            &[
                "get",
                "variant",
                "CA123643",
                "civic",
                "--civic-profile",
                "120",
                "--civic-kind",
                "evidence",
                "--civic-cursor",
                cursor,
            ],
        )?;
        if value.pointer("/result/candidates/0/sections/civic/reason") != Some(&json!("bad-reply"))
        {
            return Err(format!(
                "nonprogressing native stream emitted evidence or a repeating action: {value}"
            ));
        }
    }
    source.finish()?;
    Ok(())
}
fn stalled_body(request: &str) -> Result<String, String> {
    let mut raw: Value = serde_json::from_str(&civic_body(request)?).map_err(|e| e.to_string())?;
    let base = "/data/molecularProfiles/nodes/0/evidenceItems";
    *raw.pointer_mut(&format!("{base}/pageInfo/hasNextPage"))
        .ok_or("page flag")? = json!(true);
    if request.contains("\"evidence_after\":\"empty\"") {
        *raw.pointer_mut(&format!("{base}/nodes")).ok_or("nodes")? = json!([]);
        *raw.pointer_mut(&format!("{base}/pageInfo/endCursor"))
            .ok_or("cursor")? = Value::Null;
    }
    Ok(raw.to_string())
}
