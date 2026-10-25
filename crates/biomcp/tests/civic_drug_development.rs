//! Temporary adapter-boundary scaffold until the accepted drug command foundation lands.
#[expect(
    dead_code,
    reason = "This temporary scaffold shares the existing TLS test peer"
)]
#[path = "runner/network_stub.rs"]
mod network_stub;
use biomcp_adapters::civic;
use biomcp_fetch::{Certificates, Fetch, FetchConfig, Mode, Network};
use biomcp_types::{CivicClaimKind, DrugGet};
use serde_json::{Value, json};
use std::time::Duration;

#[test]
fn retained_native_drug_replies_record_and_replay() {
    let directory = tempfile::tempdir().expect("valid development fixture");
    let stub = network_stub::Stub::for_source("civicdb.org", "application/json", reply)
        .expect("valid development fixture");
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: directory.path().join("recorded"),
        },
        proxy: Some(biomcp_fetch::Secret::new(format!(
            "http://{}",
            stub.address
        ))),
        network: Network::new(
            None,
            None,
            "",
            Certificates::from_pem(stub.certificate.as_bytes()).expect("valid development fixture"),
        )
        .expect("valid development fixture"),
        ..Default::default()
    })
    .expect("valid development fixture");
    exercise(&fetch);
    drop(fetch);
    let requests = stub.finish().expect("valid development fixture");
    assert_eq!(requests.len(), 13);
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().join("recorded"),
        },
        ..Default::default()
    })
    .expect("valid development fixture");
    exercise(&fetch);
}
fn exercise(fetch: &Fetch) {
    search(fetch);
    exact(fetch);
    pages(fetch);
}
fn search(fetch: &Fetch) {
    for (q, offset, total, id) in [
        ("imatinib", 0, 11, "27062"),
        ("trastuzumab", 0, 104, "2986"),
        ("ivosidenib", 0, 3, "25758"),
        ("fam-trastuzumab deruxtecan", 0, 0, ""),
        ("trastuzumab", 1, 104, "2988"),
    ] {
        let leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(20)));
        let page = civic::drug_search(
            &leg,
            &q.parse().expect("valid development fixture"),
            1,
            offset,
        )
        .expect("valid development fixture");
        assert_eq!(page.total, Some(total));
        assert_eq!(
            page.items
                .first()
                .map(|r| r.id.to_string())
                .unwrap_or_default(),
            id
        );
    }
}
fn exact(fetch: &Fetch) {
    for id in ["5", "84", "454"] {
        let query: DrugGet = serde_json::from_value(
            json!({"source":"civic","input":id,"section":["civic"],"section_limit":1}),
        )
        .expect("valid development fixture");
        let value = get(fetch, &query);
        assert_eq!(at(&value, "/identity/id"), id);
        if id == "5" {
            assert_eq!(
                value.pointer("/civic/data/evidence/page/total"),
                Some(&json!(169))
            );
            assert_eq!(
                value.pointer("/civic/data/evidence/records/0/profile/record/id"),
                Some(&json!("1"))
            );
            assert_eq!(
                value.pointer("/civic/data/assertions/records/0/profile/record/id"),
                Some(&json!("65"))
            );
        } else if id == "84" {
            let row = at(&value, "/civic/data/evidence/records/0");
            assert_eq!(at(row, "/therapies/0/id"), "52");
            assert_eq!(at(row, "/therapies/1/id"), "84");
            assert_eq!(at(row, "/therapy_interaction_type"), "SEQUENTIAL");
        }
    }
}
fn pages(fetch: &Fetch) {
    let query: DrugGet = serde_json::from_value(json!({"source":"civic","input":"5","section":["civic"],"section_limit":1,"civic_profile":1,"civic_kind":"evidence"})).expect("valid development fixture");
    let value = get(fetch, &query);
    assert_eq!(
        value.pointer("/civic/data/evidence/page/total"),
        Some(&json!(4))
    );
    let next: DrugGet =
        serde_json::from_value(at(&value, "/civic/data/evidence/page/next").clone())
            .expect("valid development fixture");
    let value = get(fetch, &next);
    assert_eq!(
        value.pointer("/civic/data/evidence/records/0/claim/record/id"),
        Some(&json!("EID260"))
    );
    let query = query
        .with_civic_page(65, CivicClaimKind::Assertions, None)
        .expect("valid development fixture");
    let value = get(fetch, &query);
    assert_eq!(
        value.pointer("/civic/data/assertions/page/total"),
        Some(&json!(1))
    );
    assert!(value.pointer("/civic/data/assertions/page/next").is_none());
    let query = query
        .with_civic_page(
            65,
            CivicClaimKind::Assertions,
            Some("MQ".parse().expect("valid development fixture")),
        )
        .expect("valid development fixture");
    let value = get(fetch, &query);
    assert_eq!(
        value.pointer("/civic/data/assertions/page/returned"),
        Some(&json!(0))
    );
    assert_eq!(
        value.pointer("/civic/data/assertions/page/total"),
        Some(&json!(1))
    );
}
fn get(fetch: &Fetch, query: &DrugGet) -> Value {
    let leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(20)));
    serde_json::to_value(
        civic::drug_civic(&leg, query)
            .expect("valid development fixture")
            .value,
    )
    .expect("valid development fixture")
}
fn fixture(index: usize) -> Result<Value, String> {
    let body = match index {
        21 => include_str!("../../../tests/fixtures/drug/civic/021.body"),
        22 => include_str!("../../../tests/fixtures/drug/civic/022.body"),
        23 => include_str!("../../../tests/fixtures/drug/civic/023.body"),
        24 => include_str!("../../../tests/fixtures/drug/civic/024.body"),
        25 => include_str!("../../../tests/fixtures/drug/civic/025.body"),
        26 => include_str!("../../../tests/fixtures/drug/civic/026.body"),
        27 => include_str!("../../../tests/fixtures/drug/civic/027.body"),
        28 => include_str!("../../../tests/fixtures/drug/civic/028.body"),
        29 => include_str!("../../../tests/fixtures/drug/civic/029.body"),
        30 => include_str!("../../../tests/fixtures/drug/civic/030.body"),
        31 => include_str!("../../../tests/fixtures/drug/civic/031.body"),
        _ => return Err("undeclared source fixture".into()),
    };
    serde_json::from_str(body).map_err(|e| e.to_string())
}
fn reply(request: &str) -> Result<String, String> {
    let (_, body) = request.split_once("\r\n\r\n").ok_or("missing POST body")?;
    let body: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    let q = at(&body, "/query").as_str().ok_or("missing GraphQL")?;
    let v = at(&body, "/variables");
    let index = if let Some(text) = at(v, "/q").as_str() {
        match (text, at(v, "/after").as_str()) {
            ("imatinib", None) => 24,
            ("trastuzumab", None) => 25,
            ("ivosidenib", None) => 26,
            ("fam-trastuzumab deruxtecan", None) => 27,
            ("trastuzumab", Some("MQ")) => 28,
            _ => return Err(format!("undeclared candidate request {body}")),
        }
    } else if q.contains("molecularProfileId:") {
        match (
            q.contains("molecularProfileId:65"),
            at(v, "/after").as_str(),
        ) {
            (_, None) => 29,
            (false, Some("MQ")) => 30,
            (true, Some("MQ")) => 31,
            _ => return Err(format!("undeclared claim request {body}")),
        }
    } else {
        match at(v, "/id").as_u64() {
            Some(5) => 21,
            Some(84) => 22,
            Some(454) => 23,
            _ => return Err(format!("undeclared identity {body}")),
        }
    };
    let mut value = fixture(index)?;
    if at(v, "/id").is_number() {
        if at(&value, "/data/therapy").is_null() {
            let therapy = fixture(21)?
                .pointer("/data/therapy")
                .ok_or("retained therapy")?
                .clone();
            value
                .get_mut("data")
                .and_then(Value::as_object_mut)
                .ok_or("data")?
                .insert("therapy".into(), therapy);
        }
        for field in ["evidenceItems", "assertions"] {
            if !q.contains(&format!("{field}(")) {
                value
                    .get_mut("data")
                    .and_then(Value::as_object_mut)
                    .ok_or("data")?
                    .remove(field);
            }
        }
    }
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

fn at<'a>(value: &'a Value, path: &str) -> &'a Value {
    value.pointer(path).unwrap_or(&Value::Null)
}
