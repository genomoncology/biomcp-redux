//! Feature drug-civic: source bodies fetched 2026-10-25 are delivered locally, recorded and replayed.
//! Original fetch dates and request hashes remain in fixtures/drug/civic/provenance.json.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

const CASES: [&str; 12] = [
    "imatinib-search",
    "trastuzumab-search",
    "ivosidenib-search",
    "literal-empty",
    "candidate-next",
    "imatinib-streams",
    "trastuzumab-combination",
    "ivosidenib-streams",
    "profile-evidence",
    "profile-evidence-next",
    "profile-assertion",
    "profile-assertion-terminal",
];
pub(super) fn trials() -> Vec<Trial> {
    CASES
        .into_iter()
        .enumerate()
        .map(|(n, name)| {
            Trial::test(format!("drug-civic/{name}"), move || {
                check(n).map_err(Into::into)
            })
        })
        .collect()
}
fn arguments(n: usize) -> Result<Vec<String>, String> {
    let args = match n {
        0 => vec![
            "search", "drug", "imatinib", "--source", "civic", "--limit", "1",
        ],
        1 => vec![
            "search",
            "drug",
            "trastuzumab",
            "--source",
            "civic",
            "--limit",
            "1",
        ],
        2 => vec![
            "search",
            "drug",
            "ivosidenib",
            "--source",
            "civic",
            "--limit",
            "1",
        ],
        3 => vec![
            "search",
            "drug",
            "fam-trastuzumab deruxtecan",
            "--source",
            "civic",
            "--limit",
            "1",
        ],
        4 => vec![
            "search",
            "drug",
            "trastuzumab",
            "--source",
            "civic",
            "--limit",
            "1",
            "--offset",
            "1",
        ],
        5 => vec!["get", "drug", "5", "civic", "--source", "civic"],
        6 => vec!["get", "drug", "84", "civic", "--source", "civic"],
        7 => vec!["get", "drug", "454", "civic", "--source", "civic"],
        8 => vec![
            "get",
            "drug",
            "5",
            "civic",
            "--source",
            "civic",
            "--section-limit",
            "1",
            "--civic-profile",
            "1",
            "--civic-kind",
            "evidence",
        ],
        9 => vec![
            "get",
            "drug",
            "5",
            "civic",
            "--source",
            "civic",
            "--section-limit",
            "1",
            "--civic-profile",
            "1",
            "--civic-kind",
            "evidence",
            "--civic-cursor",
            "MQ",
        ],
        10 => vec![
            "get",
            "drug",
            "5",
            "civic",
            "--source",
            "civic",
            "--section-limit",
            "1",
            "--civic-profile",
            "65",
            "--civic-kind",
            "assertions",
        ],
        11 => vec![
            "get",
            "drug",
            "5",
            "civic",
            "--source",
            "civic",
            "--section-limit",
            "1",
            "--civic-profile",
            "65",
            "--civic-kind",
            "assertions",
            "--civic-cursor",
            "MQ",
        ],
        _ => return Err("unknown case".into()),
    };
    Ok(args.into_iter().map(String::from).collect())
}
fn check(n: usize) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_source(
        "civicdb.org",
        "application/json",
        super::drug_civic_fixtures::reply,
    )?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = arguments(n)?;
    let recorded = env.root().join("recorded");
    let mut record_args = args.clone();
    record_args.extend(["--record".into(), recorded.display().to_string()]);
    let exit = if n == 3 { 1 } else { 0 };
    let value = invoke(&env, record_args, exit)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    verify(n, &value)?;
    let requests = stub.finish()?;
    let expected = if n == 4 { 2 } else { 1 };
    if requests.len() != expected {
        return Err(format!(
            "expected {expected} local requests, got {}",
            requests.len()
        ));
    }
    let mut replay_args = args.clone();
    replay_args.extend([
        "--replay".into(),
        recorded.display().to_string(),
        "--fresh".into(),
    ]);
    let replay = invoke(&env, replay_args.clone(), exit)?;
    let mcp = super::mcp::call_with_globals(&env, &replay_args, false, &[])?;
    for key in ["result", "status", "query"] {
        if replay.get(key) != mcp.get(key) || replay.get(key) != value.get(key) {
            return Err(format!("CLI, recorder and MCP differ for {key}"));
        }
    }
    let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&value, "")?;
    eprintln!(
        "CIVIC_SIZE case={n} full_bytes={full} full_tokens={} comparable_bytes={comparable} comparable_tokens={}",
        full.div_ceil(4),
        comparable.div_ceil(4)
    );
    if comparable > 6144 {
        return Err(format!(
            "case {n} exceeds CIViC section ceiling: {comparable}"
        ));
    }
    Ok(())
}
fn verify(n: usize, value: &Value) -> Result<(), String> {
    let expected = match n {
        0 => vec![
            ("/result/total", json!(11)),
            ("/result/items/0/id", json!("27062")),
        ],
        1 => vec![
            ("/result/total", json!(104)),
            ("/result/items/0/id", json!("2986")),
        ],
        2 => vec![
            ("/result/total", json!(3)),
            ("/result/items/0/id", json!("25758")),
        ],
        3 => vec![("/result/total", json!(0)), ("/status", json!("absent"))],
        4 => vec![
            ("/result/total", json!(104)),
            ("/result/items/0/id", json!("2988")),
        ],
        5 => vec![
            ("/result/identity/id", json!("5")),
            ("/result/civic/data/evidence/page/total", json!(169)),
            ("/result/civic/data/assertions/page/total", json!(1)),
            (
                "/result/civic/data/evidence/records/0/profile/record/id",
                json!("1"),
            ),
            (
                "/result/civic/data/assertions/records/0/profile/record/id",
                json!("65"),
            ),
            (
                "/result/civic/data/evidence/page/next/civic_kind",
                json!("evidence"),
            ),
        ],
        6 => vec![
            ("/result/identity/id", json!("84")),
            (
                "/result/civic/data/evidence/records/0/therapies/0/id",
                json!("52"),
            ),
            (
                "/result/civic/data/evidence/records/0/therapies/1/id",
                json!("84"),
            ),
            (
                "/result/civic/data/evidence/records/0/therapy_interaction_type",
                json!("SEQUENTIAL"),
            ),
        ],
        7 => vec![("/result/identity/id", json!("454"))],
        8 => vec![
            ("/result/civic/data/evidence/page/total", json!(4)),
            (
                "/result/civic/data/evidence/page/next/civic_profile",
                json!(1),
            ),
            (
                "/result/civic/data/evidence/page/next/civic_cursor",
                json!("MQ"),
            ),
        ],
        9 => vec![
            (
                "/result/civic/data/evidence/records/0/claim/record/id",
                json!("EID260"),
            ),
            ("/result/civic/data/evidence/page/total", json!(4)),
        ],
        10 => vec![
            ("/result/civic/data/assertions/page/total", json!(1)),
            (
                "/result/civic/data/assertions/records/0/profile/record/id",
                json!("65"),
            ),
        ],
        11 => vec![
            ("/result/civic/data/assertions/page/total", json!(1)),
            ("/result/civic/data/assertions/page/returned", json!(0)),
        ],
        _ => return Err("unknown verification".into()),
    };
    for (path, expected) in expected {
        if value.pointer(path) != Some(&expected) {
            return Err(format!("{path}: expected {expected}, received {value}"));
        }
    }
    if matches!(n, 10 | 11)
        && value
            .pointer("/result/civic/data/assertions/page/next")
            .is_some()
    {
        return Err("invented a positive assertion continuation".into());
    }
    if n == 5
        && value
            .pointer("/result/civic/data/evidence/page/next/civic_profile")
            .is_some()
    {
        return Err("therapy-wide total relabeled as a selected profile".into());
    }
    Ok(())
}
