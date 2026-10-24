//! CIViC's public GraphQL boundary refuses malformed aliases before caching.
#![expect(
    clippy::disallowed_methods,
    clippy::unwrap_used,
    reason = "Isolate synthetic raw-envelope mutations at the public transport boundary"
)]
use biodata::Reason;
use biomcp_adapters::civic;
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, time::Duration};
const NAME: &str = "9364346f41fe0b3bcb90f63403b5221191426e9eff9fce9b22b7cd0cbee0b29a.json";
fn captured() -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/gene/civic/civic")
                .join(NAME),
        )
        .unwrap(),
    )
    .unwrap()
}
fn call(recording: &Value) -> Result<Vec<civic::GeneCivic>, RequestError> {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir(directory.path().join("civic")).unwrap();
    fs::write(
        directory.path().join("civic").join(NAME),
        serde_json::to_vec(recording).unwrap(),
    )
    .unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..Default::default()
    })
    .unwrap();
    let leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(20)));
    civic::genes(&leg, &[("ERBB2".parse().unwrap(), "2064".parse().unwrap())])
}
#[test]
fn errors_members_and_malformed_aliases_cannot_be_cached_as_absence() {
    let original = captured();
    let good = original["exchanges"][0]["body"]["value"].clone();
    let mut missing = good.clone();
    missing["data"]["g0"]
        .as_object_mut()
        .unwrap()
        .remove("description");
    let mut invalid = good.clone();
    invalid["data"]["g0"]["entrezId"] = json!(0);
    let mut bad_link = good.clone();
    bad_link["data"]["g0"]["link"] = json!("https://other.example/features/20");
    let mut bad_id = good.clone();
    bad_id["data"]["g0"]["id"] = json!(0);
    for body in [
        json!({"data":{}}),
        json!({"data":{"other":null}}),
        json!({"data":{"g0":7}}),
        json!({"data":{"g0":null},"errors":null}),
        json!({"data":{"g0":null},"errors":[]}),
        json!({"data":{"g0":null},"errors":[{"message":"missing"}]}),
        missing,
        invalid,
        bad_link,
        bad_id,
    ] {
        let mut recording = original.clone();
        recording["exchanges"][0]["body"]["value"] = body;
        assert!(
            matches!(call(&recording),Err(RequestError::Source(error)) if error.reason==Reason::BadReply)
        );
    }
    let mut duplicate = original;
    duplicate["exchanges"][0]["body"] =
        json!({"kind":"text","value":"{\"data\":{\"g0\":null,\"g0\":null}}"});
    assert!(
        matches!(call(&duplicate),Err(RequestError::Source(error)) if error.reason==Reason::BadReply)
    );
}
#[test]
fn civic_refuses_empty_and_oversized_batches_before_io() {
    let directory = tempfile::tempdir().unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..Default::default()
    })
    .unwrap();
    let leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(20)));
    for count in [0, 21] {
        let ids: Vec<_> = (1..=count)
            .map(|id| {
                (
                    format!("GENE{id}").parse().unwrap(),
                    id.to_string().parse().unwrap(),
                )
            })
            .collect();
        assert!(
            matches!(civic::genes(&leg,&ids),Err(RequestError::Source(error)) if error.reason==Reason::BadReply)
        );
    }
}

#[test]
fn malformed_optional_description_retains_the_civic_record() {
    let mut recording = captured();
    recording["exchanges"][0]["body"]["value"]["data"]["g0"]["description"] = json!(7);
    let values = call(&recording).unwrap();
    assert!(matches!(&values[0].civic.value, biodata::Section::Data { data, .. } if data.id == 20 && data.description.is_none()));
    assert_eq!(values[0].civic.problems.len(), 1);
    assert_eq!(values[0].civic.problems[0].field, "description");
}
