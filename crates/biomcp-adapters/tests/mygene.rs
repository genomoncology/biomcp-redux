//! Temporary adapter contracts until gene command cases own these behaviors.
use biodata::{Build, EntrezGeneId, HgncId, Strand};
use biomcp_adapters::mygene;
use biomcp_fetch::{Fetch, FetchConfig, Mode};
use std::{path::PathBuf, time::Duration};

fn captured() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/mygene")
}
fn replay(dir: PathBuf) -> Fetch {
    Fetch::open(FetchConfig { mode: Mode::Replay { dir }, ..Default::default() }).unwrap()
}
fn id(value: u32) -> EntrezGeneId { EntrezGeneId::new(value).unwrap() }

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
    assert_eq!((intervals[0].span.start(), intervals[0].span.end()), (140719326, 140925199));
    assert_eq!(intervals[1].build, Build::Grch37);
    assert_eq!((intervals[1].span.start(), intervals[1].span.end()), (140419126, 140624564));
    assert!(intervals.iter().all(|span| span.strand == Some(Strand::Reverse)));
    let csf2ra = &values[1].value;
    let chroms: Vec<_> = csf2ra.spans.iter().filter(|span| span.build == Build::Grch38).map(|span| span.contig.as_str()).collect();
    assert_eq!(chroms, ["X", "Y"]);
}
