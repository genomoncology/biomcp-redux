//! Temporary adapter contracts until gene command cases own these behaviors.
use biodata::{Build, EntrezGeneId, HgncId, Strand};
use biomcp_adapters::mygene;
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::{path::PathBuf, time::Duration};

fn captured() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/mygene")
}
fn replay(dir: PathBuf) -> Fetch {
    Fetch::open(FetchConfig {
        mode: Mode::Replay { dir },
        ..Default::default()
    })
    .unwrap()
}
fn id(value: u32) -> EntrezGeneId {
    EntrezGeneId::new(value).unwrap()
}

#[test]
fn recorded_gene_extras_preserve_whole_summary_and_source_coordinates() {
    let fetch = replay(captured());
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
    let batches = mygene::genes(&leg, &[id(999999999), id(673), id(1438), id(673)]);
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].ids.as_ref(), &[id(673), id(1438), id(999999999)]);
    let values = batches[0].result.as_ref().unwrap();
    assert_eq!(values.len(), 2);
    let braf = &values[0];
    assert!(braf.problems.is_empty());
    assert_eq!(braf.value.hgnc_id, Some(HgncId::new(1097).unwrap()));
    let summary = braf.value.summary.as_ref().unwrap();
    assert!(summary.text.as_str().contains("V600E"));
    assert!(!summary.text.as_str().contains("[provided by"));
    assert_eq!(summary.dated.unwrap().to_string(), "2017-08");
    let intervals = &braf.value.spans;
    assert_eq!(intervals.len(), 2);
    assert_eq!(intervals[0].build, Build::Grch38);
    assert_eq!(
        (intervals[0].span.start(), intervals[0].span.end()),
        (140719326, 140925199)
    );
    assert_eq!(intervals[1].build, Build::Grch37);
    assert_eq!(
        (intervals[1].span.start(), intervals[1].span.end()),
        (140419126, 140624564)
    );
    assert!(
        intervals
            .iter()
            .all(|span| span.strand == Some(Strand::Reverse))
    );
    let csf2ra = &values[1].value;
    let chroms: Vec<_> = csf2ra
        .spans
        .iter()
        .filter(|span| span.build == Build::Grch38)
        .map(|span| span.contig.as_str())
        .collect();
    assert_eq!(chroms, ["X", "Y"]);
}

use biodata::Reason;
use biomcp_fetch::RequestError;
use serde_json::{Value, json};

const CAPTURE_NAME: &str = "98c9104ac96220cdc31271fc9435b253a46b6290e7b0a9c0dfc8c8f8de372773.json";
#[expect(clippy::disallowed_methods, reason = "Read the captured public source reply and isolate each synthetic mutation")]
fn recording() -> Value {
    serde_json::from_str(&std::fs::read_to_string(captured().join("mygene").join(CAPTURE_NAME)).unwrap()).unwrap()
}
#[expect(clippy::disallowed_methods, reason = "Write explicitly synthetic failure fixtures only into a case-owned temporary directory")]
fn synthetic(mut change: impl FnMut(&mut Value)) -> tempfile::TempDir {
    let mut value = recording();
    change(&mut value["exchanges"][0]["body"]["value"]);
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir(temp.path().join("mygene")).unwrap();
    std::fs::write(temp.path().join("mygene").join(CAPTURE_NAME), serde_json::to_vec(&value).unwrap()).unwrap();
    temp
}
fn run_synthetic(mut change: impl FnMut(&mut Value)) -> Vec<mygene::GeneBatch> {
    let dir = synthetic(|body| change(body));
    let fetch = replay(dir.path().into());
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
    mygene::genes(&leg, &[id(673), id(1438), id(999999999)])
}

#[test]
fn synthetic_bad_fields_drop_only_the_affected_annotation() {
    for pos in [json!({"chr":"7","start":10,"end":20,"strand":0}),
                json!({"chr":"7","start":0,"end":20}),
                json!({"chr":"7","start":20,"end":10}),
                json!({"chr":"7","start":10,"end":20,"strand":"bad"}),
                json!({"chr":"7","start":10,"end":20,"strand":null}),
                json!({"chr":"7","start":"bad","end":20}),
                json!({"chr":"","start":10,"end":20}), json!(42)] {
        let batches = run_synthetic(|body| body[1]["genomic_pos"] = pos.clone());
        let braf = &batches[0].result.as_ref().unwrap()[0];
        // Null is the same as omitted strand in serde's optional source field.
        if pos["strand"].is_null() && pos["start"] == 10 && pos["chr"] == "7" {
            assert_eq!(braf.value.spans.len(), 2);
            assert!(braf.value.spans[0].strand.is_none());
        } else {
            assert_eq!(braf.value.spans.len(), 1);
            assert_eq!(braf.problems[0].field.as_ref(), "genomic_pos");
        }
    }
    for pos in [json!({"chr":"7","start":20,"end":20}),
                json!({"chr":"7","start":1,"end":2,"strand":1})] {
        let batches = run_synthetic(|body| body[1]["genomic_pos"] = pos.clone());
        let braf = &batches[0].result.as_ref().unwrap()[0];
        assert!(braf.problems.is_empty());
        assert_eq!(braf.value.spans[0].span.start(), pos["start"].as_u64().unwrap() as u32 - 1);
        assert_eq!(braf.value.spans[0].span.end(), pos["end"].as_u64().unwrap() as u32);
    }
}

#[test]
fn synthetic_untrusted_identities_cannot_attach_extras_to_another_query() {
    let mutations: Vec<Box<dyn Fn(&mut Value)>> = vec![
        Box::new(|body| { let hit = body[1].clone(); body.as_array_mut().unwrap().push(hit); }),
        Box::new(|body| body[1]["entrezgene"] = json!("1438")),
        Box::new(|body| { body[1].as_object_mut().unwrap().remove("entrezgene"); }),
        Box::new(|body| body[1]["HGNC"] = json!("not-an-id")),
        Box::new(|body| { body[1].as_object_mut().unwrap().remove("HGNC"); }),
        Box::new(|body| body[1]["query"] = json!("12345")),
        Box::new(|body| body[1]["query"] = json!("invalid")),
    ];
    for mutate in mutations {
        let batches = run_synthetic(mutate);
        let batch = &batches[0];
        assert_eq!(batch.result.as_ref().unwrap().len(), 1);
        assert_eq!(batch.result.as_ref().unwrap()[0].value.query_id, id(1438));
        assert_eq!(batch.problems.len(), 1);
    }
    // An HGNC identity mismatch is decided by the authoritative join consumer.
    let batches = run_synthetic(|body| body[1]["HGNC"] = json!("2435"));
    assert_eq!(batches[0].result.as_ref().unwrap()[0].value.hgnc_id, Some(HgncId::new(2435).unwrap()));
}

#[test]
fn synthetic_malformed_envelopes_fail_as_bad_reply() {
    for mode in 0..3 {
        let batches = run_synthetic(|body| match mode {
            0 => { body[1].as_object_mut().unwrap().remove("query"); },
            1 => body[1]["query"] = json!(""),
            // A 200 error envelope is synthetic, not an observed provider status.
            _ => *body = json!({"error":"synthetic invalid input"}),
        });
        assert!(matches!(batches[0].result, Err(RequestError::Source(error)) if error.reason == Reason::BadReply));
    }
}

#[test]
fn synthetic_summary_markers_preserve_unrecognized_text_and_reject_bad_fields() {
    for text in ["Whole text without a marker", "Whole text [provided by RefSeq, Autumn 2017].",
                 "Whole text [provided by RefSeq, Aug 0000].", "Whole text [provided by RefSeq, Aug 2017]. trailing"] {
        let batches = run_synthetic(|body| body[1]["summary"] = json!(text));
        let summary = batches[0].result.as_ref().unwrap()[0].value.summary.as_ref().unwrap();
        assert_eq!(summary.text.as_str(), text);
        assert!(summary.dated.is_none());
    }
    for bad in [json!(42), json!(" ")] {
        let batches = run_synthetic(|body| body[1]["summary"] = bad.clone());
        let mapped = &batches[0].result.as_ref().unwrap()[0];
        assert!(mapped.value.summary.is_none());
        assert_eq!(mapped.problems[0].field.as_ref(), "summary");
    }
    let batches = run_synthetic(|body| { body[1].as_object_mut().unwrap().remove("summary"); body[1].as_object_mut().unwrap().remove("genomic_pos"); });
    assert!(batches[0].result.as_ref().unwrap()[0].value.summary.is_none());
}

#[test]
fn synthetic_chunk_failure_does_not_hide_a_successful_chunk() {
    let fetch = replay(captured().join("synthetic-twochunks"));
    let leg = fetch.leg(&mygene::SPEC, fetch.deadline(Duration::from_secs(20)));
    assert!(mygene::genes(&leg, &[]).is_empty());
    let ids: Vec<_> = (1..=201).map(id).collect();
    let batches = mygene::genes(&leg, &ids);
    assert_eq!(batches.len(), 2);
    assert_eq!(batches[0].ids.as_ref(), &ids[..200]);
    assert!(batches[0].result.is_err());
    assert_eq!(batches[1].ids.as_ref(), &[id(201)]);
    let values = batches[1].result.as_ref().unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].value.query_id, id(201));
}
