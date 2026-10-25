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
    let directory = tempfile::tempdir().unwrap();
    let stub = network_stub::Stub::for_source("civicdb.org", "application/json", reply).unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Record {
            dir: directory.path().join("recorded"),
        },
        proxy: Some(format!("http://{}", stub.address).into()),
        network: Network::new(
            None,
            None,
            "",
            Certificates::from_pem(stub.certificate.as_bytes()).unwrap(),
        )
        .unwrap(),
        ..Default::default()
    })
    .unwrap();
    exercise(&fetch);
    drop(fetch);
    let requests = stub.finish().unwrap();
    assert_eq!(requests.len(), 13);
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().join("recorded"),
        },
        ..Default::default()
    })
    .unwrap();
    exercise(&fetch);
}
fn exercise(fetch: &Fetch) {
    for (q, offset, total, id) in [
        ("imatinib", 0, 11, "27062"),
        ("trastuzumab", 0, 104, "2986"),
        ("ivosidenib", 0, 3, "25758"),
        ("fam-trastuzumab deruxtecan", 0, 0, ""),
        ("trastuzumab", 1, 104, "2988"),
    ] {
        let leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(20)));
        let page = civic::drug_search(&leg, &q.parse().unwrap(), 1, offset).unwrap();
        assert_eq!(page.total, Some(total));
        assert_eq!(
            page.items
                .first()
                .map(|r| r.id.to_string())
                .unwrap_or_default(),
            id
        );
    }
    for id in ["5", "84", "454"] {
        let query: DrugGet = serde_json::from_value(
            json!({"source":"civic","input":id,"section":["civic"],"section_limit":1}),
        )
        .unwrap();
        let value = get(fetch, &query);
        assert_eq!(value["identity"]["id"], id);
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
            let row = &value["civic"]["data"]["evidence"]["records"][0];
            assert_eq!(row["therapies"][0]["id"], "52");
            assert_eq!(row["therapies"][1]["id"], "84");
            assert_eq!(row["therapy_interaction_type"], "SEQUENTIAL");
        }
    }
    let query: DrugGet = serde_json::from_value(json!({"source":"civic","input":"5","section":["civic"],"section_limit":1,"civic_profile":1,"civic_kind":"evidence"})).unwrap();
    let value = get(fetch, &query);
    assert_eq!(
        value.pointer("/civic/data/evidence/page/total"),
        Some(&json!(4))
    );
    let next: DrugGet =
        serde_json::from_value(value["civic"]["data"]["evidence"]["page"]["next"].clone()).unwrap();
    let value = get(fetch, &next);
    assert_eq!(
        value.pointer("/civic/data/evidence/records/0/claim/record/id"),
        Some(&json!("EID260"))
    );
    let query = query
        .with_civic_page(65, CivicClaimKind::Assertions, None)
        .unwrap();
    let value = get(fetch, &query);
    assert_eq!(
        value.pointer("/civic/data/assertions/page/total"),
        Some(&json!(1))
    );
    assert!(value.pointer("/civic/data/assertions/page/next").is_none());
    let query = query
        .with_civic_page(65, CivicClaimKind::Assertions, Some("MQ".parse().unwrap()))
        .unwrap();
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
    serde_json::to_value(civic::drug_civic(&leg, query).unwrap().value).unwrap()
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
    let q = body["query"].as_str().ok_or("missing GraphQL")?;
    let v = &body["variables"];
    let index = if let Some(text) = v["q"].as_str() {
        match (text, v["after"].as_str()) {
            ("imatinib", None) => 24,
            ("trastuzumab", None) => 25,
            ("ivosidenib", None) => 26,
            ("fam-trastuzumab deruxtecan", None) => 27,
            ("trastuzumab", Some("MQ")) => 28,
            _ => return Err(format!("undeclared candidate request {body}")),
        }
    } else if q.contains("molecularProfileId:") {
        match (q.contains("molecularProfileId:65"), v["after"].as_str()) {
            (_, None) => 29,
            (false, Some("MQ")) => 30,
            (true, Some("MQ")) => 31,
            _ => return Err(format!("undeclared claim request {body}")),
        }
    } else {
        match v["id"].as_u64() {
            Some(5) => 21,
            Some(84) => 22,
            Some(454) => 23,
            _ => return Err(format!("undeclared identity {body}")),
        }
    };
    let mut value = fixture(index)?;
    if v["id"].is_number() {
        if value["data"]["therapy"].is_null() {
            value["data"]["therapy"] = fixture(21)?["data"]["therapy"].clone();
        }
        for field in ["evidenceItems", "assertions"] {
            if !q.contains(&format!("{field}(")) {
                value["data"].as_object_mut().ok_or("data")?.remove(field);
            }
        }
    }
    serde_json::to_string(&value).map_err(|e| e.to_string())
}
