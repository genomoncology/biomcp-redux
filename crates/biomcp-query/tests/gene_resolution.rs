//! Temporary public resolution contracts until command cases consume this surface.
#![expect(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic, reason = "Fixture setup must fail on invalid checked inputs")]
use biodata::{GeneInput,GeneResolution,HgncStatus,MatchKind,LocusGroup};
use biomcp_query::gene;
use biomcp_fetch::{Fetch,FetchConfig,Mode};
use biomcp_adapters::hgnc;
use std::{path::PathBuf,time::Duration};
fn captured()->PathBuf{PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/hgnc")}
fn replay(dir:PathBuf)->Fetch{Fetch::open(FetchConfig{mode:Mode::Replay{dir},..Default::default()}).unwrap()}
fn resolve(fetch:&Fetch,text:&str)->GeneResolution{
 let leg=fetch.leg(&hgnc::SPEC,fetch.deadline(Duration::from_secs(120)));
 let mapped=gene::resolve(&leg,&GeneInput::new(text).unwrap()).unwrap();
 assert!(mapped.problems.is_empty(),"{text}: {:?}",mapped.problems);
 mapped.value
}
#[test]
fn captured_symbols_aliases_previous_symbols_and_namespaced_ids_resolve(){
 let fetch=replay(captured());
 for (text,id,kind,source) in [
  ("BRAF",1097,MatchKind::Symbol,"BRAF"),("braf",1097,MatchKind::Symbol,"BRAF"),
  ("HER2",3430,MatchKind::Alias,"HER2"),("MLL",7132,MatchKind::PreviousSymbol,"MLL"),
  ("FAM46C",24712,MatchKind::PreviousSymbol,"FAM46C"),
  ("HGNC:1097",1097,MatchKind::HgncId,"HGNC:1097"),("673",1097,MatchKind::EntrezId,"673"),
  ("ENSG00000157764",1097,MatchKind::EnsemblId,"ENSG00000157764"),
  ("ENSG00000157764.12",1097,MatchKind::EnsemblId,"ENSG00000157764"),
 ] {
  let GeneResolution::Resolved{gene,r#match}=resolve(&fetch,text) else {panic!("expected resolved {text}")};
  assert_eq!(gene.hgnc_id.get(),id);assert_eq!(r#match.kind,kind);assert_eq!(r#match.text.as_str(),source);
 }
}
#[test]
fn captured_ambiguity_orders_every_distinct_identity_without_selection(){
 let fetch=replay(captured());
 for (text,want) in [
  ("MLL2",vec![(7133,MatchKind::PreviousSymbol),(15840,MatchKind::Alias)]),
  ("p16",vec![(1787,MatchKind::Alias),(43802,MatchKind::Alias)]),
  ("100874204",vec![(40244,MatchKind::EntrezId),(55559,MatchKind::EntrezId)]),
 ] {
  let GeneResolution::Ambiguous{candidates}=resolve(&fetch,text) else{panic!("expected ambiguous {text}")};
  let got:Vec<_>=candidates.iter().map(|v|(v.hgnc_id.get(),v.r#match.as_ref().unwrap().kind)).collect();
  assert_eq!(got,want);
 }
}
#[test]
fn captured_withdrawn_identity_preserves_all_stated_successors(){
 let fetch=replay(captured());
 for (text,id,status,targets) in [
  ("BRCA3",18617,HgncStatus::EntryWithdrawn,vec![]),
  ("A2MR",9,HgncStatus::MergedSplit,vec![6692]),
  ("HGNC:9",9,HgncStatus::MergedSplit,vec![6692]),
  ("CBBM",1531,HgncStatus::MergedSplit,vec![4206,9936]),
  ("ADCR",231,HgncStatus::MergedSplit,vec![12797]),
 ]{
  let GeneResolution::Withdrawn{hgnc_id,hgnc_status,merged_into,..}=resolve(&fetch,text) else{panic!("expected withdrawn {text}")};
  assert_eq!(hgnc_id.get(),id);assert_eq!(hgnc_status,status);
  assert_eq!(merged_into.iter().map(|id|id.get()).collect::<Vec<_>>(),targets);
 }
 assert!(matches!(resolve(&fetch,"AK6"),GeneResolution::Resolved{gene,..} if gene.symbol.as_str()=="AK6"));
 for text in ["NOTAGENE","999999999","ENSG99999999999"] {
  assert!(matches!(resolve(&fetch,text),GeneResolution::NotFound{}));
 }
}
#[test]
fn captured_gene_fields_keep_source_precision_and_optional_absence(){
 let fetch=replay(captured());
 let GeneResolution::Resolved{gene,..}=resolve(&fetch,"KRAS") else{panic!("KRAS")};
 let mane=gene.mane_select.unwrap();
 assert_eq!(mane.refseq().to_string(),"NM_004985.5");assert_eq!(mane.ensembl().to_string(),"ENST00000311936.8");
 let GeneResolution::Resolved{gene,..}=resolve(&fetch,"MIR21") else{panic!("MIR21")};
 assert_eq!(gene.locus_group,LocusGroup::NonCodingRna);assert!(gene.uniprot.is_empty());
 let GeneResolution::Resolved{gene,..}=resolve(&fetch,"DMD") else{panic!("DMD")};
 assert!(gene.location.is_none());
}
const COMPLETE:&str="0887a33cc809657a2a564d65e8cbfcddf45adc3810a8bfb0653e03b418cf2953.json";
#[expect(clippy::disallowed_methods,reason="Mutate captured public text only in an isolated synthetic fixture")]
fn synthetic(mut change:impl FnMut(&mut Vec<String>,&[&str]))->tempfile::TempDir{
 let mut recording:serde_json::Value=serde_json::from_str(&std::fs::read_to_string(captured().join("hgnc").join(COMPLETE)).unwrap()).unwrap();
 let body=recording["exchanges"][0]["body"]["value"].as_str().unwrap();
 let mut lines=body.lines();let header=lines.next().unwrap();let fields:Vec<_>=header.split('\t').collect();
 let mut edited=vec![header.to_owned()];
 for line in lines{
  let mut cells:Vec<String>=line.split('\t').map(str::to_owned).collect();change(&mut cells,&fields);edited.push(cells.join("\t"));
 }
 recording["exchanges"][0]["body"]["value"]=format!("{}\n",edited.join("\n")).into();
 let dir=tempfile::tempdir().unwrap();std::fs::create_dir(dir.path().join("hgnc")).unwrap();
 std::fs::write(dir.path().join("hgnc").join(COMPLETE),serde_json::to_vec(&recording).unwrap()).unwrap();dir
}
#[test]
fn synthetic_multiple_index_matches_keep_one_best_identity(){
 let dir=synthetic(|cells,fields|{
  let symbol=fields.iter().position(|f|*f=="symbol").unwrap();
  if cells[symbol]=="KMT2D" {let alias=fields.iter().position(|f|*f=="alias_symbol").unwrap();cells[alias]="MLL2".into();}
  // Make a third record share the alias to exercise locus-group ordering.
  if cells[symbol]=="H3P10"{let alias=fields.iter().position(|f|*f=="alias_symbol").unwrap();cells[alias]="MLL2".into();}
 });
 let fetch=replay(dir.path().into());
 let GeneResolution::Ambiguous{candidates}=resolve(&fetch,"MLL2") else{panic!("MLL2")};
 assert_eq!(candidates.iter().map(|v|v.hgnc_id.get()).collect::<Vec<_>>(),[7133,15840,43802]);
 assert_eq!(candidates[0].r#match.as_ref().unwrap().kind,MatchKind::PreviousSymbol);
}
