//! Temporary public resolution contracts until command cases consume this surface.
#![expect(clippy::unwrap_used, reason = "Fixture setup must fail on invalid checked inputs")]
use biodata::{GeneInput,GeneResolution};
use biomcp_query::gene;
use biomcp_fetch::{Fetch,FetchConfig,Mode};
use biomcp_adapters::hgnc;
use std::{path::PathBuf,time::Duration};
#[test]
fn captured_gene_resolution_preserves_identity_and_ambiguity() {
 let fetch=Fetch::open(FetchConfig{mode:Mode::Replay{dir:PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/hgnc")},..Default::default()}).unwrap();
 let leg=fetch.leg(&hgnc::SPEC,fetch.deadline(Duration::from_secs(120)));
 for text in ["BRAF","braf","HGNC:1097","673","ENSG00000157764"] {
  let mapped=gene::resolve(&leg,&GeneInput::new(text).unwrap()).unwrap();
  assert!(mapped.problems.is_empty());
  assert!(matches!(mapped.value,GeneResolution::Resolved{gene,..} if gene.hgnc_id.get()==1097));
 }
}
