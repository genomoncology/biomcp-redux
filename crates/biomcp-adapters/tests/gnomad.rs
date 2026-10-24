//! Public GraphQL decoder boundary tables beyond the command stories.
#![expect(
    clippy::disallowed_methods,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Isolate synthetic mutations of a public source capture and assert checked results"
)]
use biodata::Reason;
use biomcp_adapters::gnomad;
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, time::Duration};
const NAME: &str = "1f31a0cea7891ff58278d430757574d3fea0a23545c0f19f562fcf0e137403da.json";
fn captured() -> Value {
    serde_json::from_str(
        &fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/gene/constraint/gnomad")
                .join(NAME),
        )
        .unwrap(),
    )
    .unwrap()
}
fn call(mut recording: Value, body: Value) -> Result<Vec<gnomad::GeneConstraint>, RequestError> {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir(directory.path().join("gnomad")).unwrap();
    if recording["exchanges"][0]["body"]["kind"] == "json" {
        recording["exchanges"][0]["body"]["value"] = body;
    }
    fs::write(
        directory.path().join("gnomad").join(NAME),
        serde_json::to_vec(&recording).unwrap(),
    )
    .unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..Default::default()
    })
    .unwrap();
    let leg = fetch.leg(&gnomad::SPEC, fetch.deadline(Duration::from_secs(20)));
    gnomad::constraints(&leg, &["ENSG00000141510".parse().unwrap()])
}
#[test]
fn malformed_aliases_errors_and_required_fields_never_become_confirmed_absence() {
    let recording = captured();
    let good = recording["exchanges"][0]["body"]["value"].clone();
    let mut missing_field = good.clone();
    missing_field["data"]["g0"]
        .as_object_mut()
        .unwrap()
        .remove("gnomad_constraint");
    let mut wrong_type = good.clone();
    wrong_type["data"]["g0"]["gnomad_constraint"]["pli"] = json!("NaN");
    let mut bad_transcript = good.clone();
    bad_transcript["data"]["g0"]["mane_select_transcript"]["ensembl_id"] = json!("bad");
    for (name, body) in [
        ("missing alias", json!({"data":{}})),
        ("wrong data type", json!({"data":17})),
        ("unexpected alias", json!({"data":{"extra":null}})),
        ("wrong type alias", json!({"data":{"g0":7}})),
        ("unexplained null", json!({"data":{"g0":null}})),
        (
            "error count mismatch",
            json!({"data":{"g0":null},"errors":[]}),
        ),
        (
            "mixed messages",
            json!({"data":{"g0":null},"errors":[{"message":"Gene not found"},{"message":"bad query"}]}),
        ),
        ("malformed error", json!({"data":{"g0":null},"errors":[{}]})),
        (
            "wrong error message",
            json!({"data":{"g0":null},"errors":[{"message":"gene not found"}]}),
        ),
        ("missing requested field", missing_field),
        ("wrong numeric type", wrong_type),
        ("invalid preferred transcript", bad_transcript),
    ] {
        assert!(
            matches!(call(recording.clone(), body), Err(RequestError::Source(error)) if error.reason == Reason::BadReply),
            "{name}"
        );
    }
    let mut duplicate = recording.clone();
    duplicate["exchanges"][0]["body"] = json!({"kind":"text","value":"{\"data\":{\"g0\":null,\"g0\":null},\"errors\":[{\"message\":\"Gene not found\"},{\"message\":\"Gene not found\"}]}"});
    assert!(
        matches!(call(duplicate, Value::Null), Err(RequestError::Source(error)) if error.reason == Reason::BadReply)
    );
}
#[test]
fn source_preferred_transcript_fallback_and_mapping_problems_remain_independent() {
    let recording = captured();
    let mut body = recording["exchanges"][0]["body"]["value"].clone();
    body["data"]["g0"]["mane_select_transcript"]["ensembl_id"] = json!("ENST00000311936.8");
    body["data"]["g0"]["gnomad_constraint"]["flags"] = json!(["source flag", 17, ""]);
    let values = call(recording.clone(), body.clone()).unwrap();
    let value = &values[0].constraint;
    assert_eq!(
        value.value.as_ref().unwrap().transcript.to_string(),
        "ENST00000311936"
    );
    assert_eq!(
        value.value.as_ref().unwrap().flags[0].as_str(),
        "source flag"
    );
    assert_eq!(value.problems.len(), 2);
    body["data"]["g0"]["mane_select_transcript"] = Value::Null;
    let values = call(recording.clone(), body.clone()).unwrap();
    assert_eq!(
        values[0]
            .constraint
            .value
            .as_ref()
            .unwrap()
            .transcript
            .to_string(),
        "ENST00000269305"
    );
    for flags in [Value::Null, json!({"unexpected":"value"})] {
        let mut changed = body.clone(); changed["data"]["g0"]["gnomad_constraint"]["flags"] = flags.clone();
        let values = call(recording.clone(), changed).unwrap();
        assert!(values[0].constraint.value.as_ref().unwrap().flags.is_empty());
        assert_eq!(values[0].constraint.problems.is_empty(), flags.is_null());
    }
    body["data"]["g0"]["gnomad_constraint"] = Value::Null;
    assert!(
        call(recording.clone(), body).unwrap()[0]
            .constraint
            .value
            .is_none()
    );
    let empty = json!({"data":{"g0":null},"errors":[{"message":"Gene not found"}]});
    assert!(
        call(recording.clone(), empty).unwrap()[0]
            .constraint
            .value
            .is_none()
    );
    let mut wrong_identity = recording["exchanges"][0]["body"]["value"].clone();
    wrong_identity["data"]["g0"]["gene_id"] = json!("ENSG00000157764");
    let values = call(recording, wrong_identity).unwrap();
    assert_eq!(values[0].constraint.problems[0].field.as_ref(), "gene_id");
}

#[test]
fn the_public_constraint_batch_refuses_empty_and_oversized_distinct_queries_before_io() {
    let directory = tempfile::tempdir().unwrap();
    let fetch = Fetch::open(FetchConfig { mode: Mode::Replay { dir: directory.path().into() }, ..Default::default() }).unwrap();
    let leg = fetch.leg(&gnomad::SPEC, fetch.deadline(Duration::from_secs(20)));
    for count in [0, 21] {
        let ids: Vec<_> = (0..count).map(|id| format!("ENSG{id:011}").parse().unwrap()).collect();
        assert!(matches!(gnomad::constraints(&leg, &ids), Err(RequestError::Source(error)) if error.reason == Reason::BadReply));
    }
}
