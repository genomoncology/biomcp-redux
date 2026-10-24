//! Public bundle mapping boundary: incomplete or contradictory rows are never absence.
#![expect(
    clippy::disallowed_methods,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Isolate synthetic boundary mutations of retained public source records"
)]
use biodata::{ClinGenGene, Reason};
use biomcp_adapters::clingen;
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use std::{fs, path::PathBuf, time::Duration};
fn call(case: &str) -> Result<Option<ClinGenGene>, RequestError> {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir(directory.path().join("clingen")).unwrap();
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/clingen/clingen");
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let dosage = value["url"].as_str().unwrap().contains("dosage");
        let body = value["exchanges"][0]["body"]["value"].as_str().unwrap();
        let mut lines: Vec<_> = body.lines().map(str::to_owned).collect();
        let index = lines
            .iter()
            .position(|v| v.starts_with("\"TP53\""))
            .unwrap();
        match (case, dosage) {
            ("duplicate", true) => lines.push(lines[index].clone()),
            ("missing classification", false) => {
                lines[index] = lines[index].replace("\"Definitive\"", "\"\"")
            }
            ("validity only", true) | ("dosage only", false) => {
                lines.remove(index);
            }
            _ => {}
        }
        value["exchanges"][0]["body"]["value"] = format!("{}\n", lines.join("\n")).into();
        if case == "failed dosage" && dosage {
            value["exchanges"][0]["status"] = 500.into();
        }
        if case == "missing dosage" && dosage {
            continue;
        }
        fs::write(
            directory
                .path()
                .join("clingen")
                .join(path.file_name().unwrap()),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
    }
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..Default::default()
    })
    .unwrap();
    let leg = fetch.leg(&clingen::SPEC, fetch.deadline(Duration::from_secs(20)));
    clingen::gene(&leg, "HGNC:11998".parse().unwrap())
}
#[test]
fn contradictory_incomplete_and_failed_dosage_never_become_empty() {
    for case in ["duplicate", "missing classification", "failed dosage"] {
        assert!(
            matches!(call(case), Err(RequestError::Source(e)) if e.reason == Reason::BadReply),
            "{case}"
        );
    }
    assert!(matches!(
        call("missing dosage"),
        Err(RequestError::Recording(_))
    ));
}
#[test]
fn a_single_curated_file_still_requires_both_complete_imports() {
    let validity = call("validity only").unwrap().unwrap();
    assert_eq!(validity.validity.len(), 1);
    assert!(validity.haploinsufficiency.is_none());
    let dosage = call("dosage only").unwrap().unwrap();
    assert!(dosage.validity.is_empty());
    assert_eq!(
        dosage.haploinsufficiency.unwrap().as_str(),
        "Sufficient Evidence for Haploinsufficiency"
    );
}
