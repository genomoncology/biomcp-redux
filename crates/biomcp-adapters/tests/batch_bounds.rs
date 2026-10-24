//! Adapter call shapes that command validation cannot construct.

use biomcp_adapters::{civic, gnomad, mygene};
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use std::time::Duration;
#[test]
fn public_batches_reject_empty_and_oversized_inputs_before_io() {
    let directory = tempfile::tempdir().unwrap();
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..Default::default()
    })
    .unwrap();
    for count in [0, 21] {
        let civic_leg = fetch.leg(&civic::SPEC, fetch.deadline(Duration::from_secs(20)));
        let civic_ids: Vec<_> = (1..=count)
            .map(|id| {
                (
                    format!("GENE{id}").parse().unwrap(),
                    id.to_string().parse().unwrap(),
                )
            })
            .collect();
        assert!(
            matches!(civic::genes(&civic_leg,&civic_ids), Err(RequestError::Source(e)) if e.reason == biodata::Reason::BadReply)
        );
        let gnomad_leg = fetch.leg(&gnomad::SPEC, fetch.deadline(Duration::from_secs(20)));
        let ids: Vec<_> = (0..count)
            .map(|id| format!("ENSG{id:011}").parse().unwrap())
            .collect();
        assert!(
            matches!(gnomad::constraints(&gnomad_leg,&ids), Err(RequestError::Source(e)) if e.reason == biodata::Reason::BadReply)
        );
    }
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
    assert!(mygene::genes(&leg, &[]).is_empty());
}
#[test]
fn synthetic_chunk_failure_does_not_hide_a_successful_chunk() {
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/mygene/synthetic-twochunks"),
        },
        ..Default::default()
    })
    .unwrap();
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
    assert!(mygene::genes(&leg, &[]).is_empty());
    let ids: Vec<_> = (1..=201)
        .map(|id| biodata::NcbiGeneId::new(id).unwrap())
        .collect();
    let batches = mygene::genes(&leg, &ids);
    assert_eq!(batches.len(), 2);
    assert_eq!(batches[0].ids.as_ref(), &ids[..200]);
    assert!(batches[0].result.is_err());
    assert_eq!(
        batches[1].ids.as_ref(),
        &[biodata::NcbiGeneId::new(201).unwrap()]
    );
    let values = batches[1].result.as_ref().unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(
        values[0].value.query_id,
        biodata::NcbiGeneId::new(201).unwrap()
    );
}

#[test]
fn indexed_lookups_keep_literal_band_and_xref_namespaces() {
    use biomcp_adapters::hgnc;
    use biomcp_fetch::{Find, KeyKind};
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/replies/hgnc"),
        },
        ..Default::default()
    })
    .unwrap();
    let leg = fetch.leg(&hgnc::SPEC, fetch.deadline(Duration::from_secs(120)));
    for key in ["673", "ensembl:673", "entrez:ENSG00000157764"] {
        assert!(
            hgnc::lookup(&leg, Find::Exact(KeyKind::Xref, key))
                .unwrap()
                .value
                .is_empty()
        );
    }
    assert!(hgnc::in_band(&leg, "%_").unwrap().value.is_empty());
}
