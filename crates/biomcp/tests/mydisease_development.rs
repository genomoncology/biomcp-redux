//! Temporary source-boundary proof until the disease command foundation lands.
#![expect(
    clippy::disallowed_methods,
    reason = "Reuse the existing local TLS peer"
)]
#[expect(dead_code, reason = "Share the existing TLS test peer")]
#[path = "runner/network_stub.rs"]
mod network_stub;
use biomcp_adapters::mydisease;
use biomcp_fetch::{Certificates, Fetch, FetchConfig, Mode, Network};
use serde_json::Value;
use std::{path::Path, time::Duration};
type Error = Box<dyn std::error::Error>;

#[test]
fn wrong_identity_never_enters_the_checked_cache() -> Result<(), Error> {
    let mut attempt = 0;
    let stub = network_stub::Stub::for_source_status(
        "mydisease.info",
        "application/json",
        move |request| {
            assert!(
                request.starts_with(
                    "GET /v1/disease/MONDO%3A0000001?fields=mondo%2Cdisease_ontology "
                )
            );
            attempt += 1;
            let id = if attempt == 1 {
                "MONDO:0000002"
            } else {
                "MONDO:0000001"
            };
            Ok(
                    serde_json::json!({"_id":id,"mondo":{"mondo":id,"label":"Synthetic label","xrefs":{"doid":["MONDO:0000001"]}}})
                        .to_string(),
                )
        },
        |_| 200,
    )?;
    let directory = tempfile::tempdir()?;
    let fetch = record(&stub, directory.path(), true)?;
    let leg = fetch.leg(&mydisease::SPEC, fetch.deadline(Duration::from_secs(20)));
    let id = "MONDO:0000001".parse()?;
    let error = mydisease::get(&leg, &id).unwrap_err();
    assert!(
        matches!(error, biomcp_fetch::RequestError::Source(e) if e.reason == biomcp_types::Reason::BadReply)
    );
    assert_eq!(mydisease::get(&leg, &id)?.returned_id, id);
    assert_eq!(mydisease::get(&leg, &id)?.returned_id, id);
    drop(fetch);
    assert_eq!(stub.finish()?.len(), 2);
    Ok(())
}

#[test]
#[ignore = "Explicit retained-body proof requires BIOMCP_MYDISEASE_NATIVE outside Git"]
fn retained_native_annotations_record_and_replay() -> Result<(), Error> {
    let directory = tempfile::tempdir()?;
    let stub = network_stub::Stub::for_source("mydisease.info", "application/json", native_reply)?;
    let fetch = record(&stub, directory.path(), false)?;
    exercise(&fetch)?;
    drop(fetch);
    assert_eq!(stub.finish()?.len(), 11);
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().join("recorded"),
        },
        ..Default::default()
    })?;
    exercise(&fetch)?;
    Ok(())
}
fn record(stub: &network_stub::Stub, root: &Path, cache: bool) -> Result<Fetch, Error> {
    Ok(Fetch::open(FetchConfig {
        mode: if cache {
            Mode::Live { fresh: false }
        } else {
            Mode::Record {
                dir: root.join("recorded"),
            }
        },
        cache: cache.then(|| root.join("cache.sqlite")),
        proxy: Some(biomcp_fetch::Secret::new(format!(
            "http://{}",
            stub.address
        ))),
        network: Network::new(
            None,
            None,
            "",
            Certificates::from_pem(stub.certificate.as_bytes())?,
        )?,
        ..Default::default()
    })?)
}
fn captures() -> Result<(std::path::PathBuf, Vec<Value>), Error> {
    let root = std::path::PathBuf::from(std::env::var("BIOMCP_MYDISEASE_NATIVE")?);
    let manifest = std::fs::read(root.join("requests.json"))?;
    assert_eq!(
        digest(&manifest),
        "7cd1db0335a1cb96203260f4ea7887b54ebdc9e94df9f3ee8c3014f063f106ba"
    );
    Ok((root, serde_json::from_slice(&manifest)?))
}
fn native_reply(request: &str) -> Result<String, String> {
    native_body(request).map_err(|e| e.to_string())
}
fn native_body(request: &str) -> Result<String, Error> {
    let target = request
        .lines()
        .next()
        .and_then(|v| v.split_whitespace().nth(1))
        .ok_or("missing request")?;
    let (path, fields) = target.split_once('?').ok_or("missing projection")?;
    assert_eq!(fields, "fields=mondo%2Cdisease_ontology");
    let id = path
        .strip_prefix("/v1/disease/")
        .ok_or("wrong endpoint")?
        .replace("%3A", ":");
    let (root, requests) = captures()?;
    let source = requests
        .iter()
        .find(|r| r["source"] == "mydisease" && r["requested_id"] == id)
        .ok_or("undeclared request")?;
    let bytes = std::fs::read(
        root.join(source["directory"].as_str().ok_or("missing directory")?)
            .join("body.raw"),
    )?;
    assert_eq!(
        digest(&bytes),
        source["sha256"].as_str().ok_or("missing hash")?
    );
    // Original URL/date/hash remain in requests.json. This is a local retained-body delivery.
    Ok(String::from_utf8(bytes)?)
}
fn digest(bytes: &[u8]) -> String {
    use sha2::Digest;
    sha2::Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn exercise(fetch: &Fetch) -> Result<(), Error> {
    let (_, requests) = captures()?;
    for source in requests.iter().filter(|r| r["source"] == "mydisease") {
        let id: biodata::Curie = source["requested_id"]
            .as_str()
            .ok_or("missing ID")?
            .parse()?;
        let leg = fetch.leg(&mydisease::SPEC, fetch.deadline(Duration::from_secs(20)));
        let record = mydisease::get(&leg, &id)?;
        assert_eq!(record.requested_id, id);
        assert_eq!(
            record.returned_id.to_string(),
            source["returned_id"]
                .as_str()
                .ok_or("missing returned ID")?
        );
        let mondo = record
            .components
            .iter()
            .find(|c| c.name == "mondo")
            .ok_or("missing MONDO")?;
        assert_eq!(mondo.id, record.returned_id);
        if id.to_string() == "MONDO:0005105" {
            assert_eq!(mondo.synonyms.len(), 4);
        }
        if id.to_string() == "MONDO:0800444" {
            assert!(mondo.definition.is_none());
            assert_eq!(mondo.parents.len(), 2);
        }
        if id.to_string() == "MONDO:0018875" {
            let doids: Vec<_> = mondo
                .xrefs
                .iter()
                .filter(|v| v.field == "doid")
                .map(|v| v.value.as_str())
                .collect();
            assert_eq!(doids, ["DOID:3012", "DOID:0111503"]);
        }
        if id.to_string() == "DOID:3910" {
            assert_eq!(mondo.parents.len(), 2);
            assert_eq!(
                record
                    .components
                    .iter()
                    .find(|c| c.name == "doid")
                    .ok_or("missing DOID component")?
                    .parents
                    .len(),
                2
            );
        }
        if id.to_string() == "MONDO:0007607" {
            assert_eq!(mondo.obsolete, Some(true));
            assert!(mondo.parents.is_empty());
            assert_eq!(
                mondo.consider.as_ref().ok_or("missing alternatives")?.len(),
                2
            );
        }
    }
    Ok(())
}
