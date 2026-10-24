//! Bundle contracts through the public transport boundary.
#![expect(clippy::unwrap_used, reason = "Fixture failures fail the contract")]
mod support;
use biomcp_fetch::{Bundle, Fetch, FetchConfig, Find, Freshness, KeyKind, KeyUse, Keys, Quota, Rate, Row, RowFormat, SourceKey};
use serde::Deserialize;
use std::{marker::PhantomData, time::Duration};
#[derive(Debug, Deserialize)]
struct Gene { symbol: String, band: String, name: String }
impl Row for Gene {
    fn keys(&self) -> Keys { Keys { exact: vec![(KeyKind::Symbol,self.symbol.clone().into()),(KeyKind::Band,self.band.clone().into())], words:vec![self.name.clone().into()] } }
}
const QUOTA: Quota = Quota::new(SourceKey::declare("bundle-contract"),Rate::per_second(1000),None,4);
#[test]
fn indexed_tsv_rows_are_literal_and_offline() {
    let server = support::Server::<()>::new();
    let spec=server.spec(&QUOTA,KeyUse::None);
    let bundle=Bundle::<Gene>{spec,name:"genes",path:"/genes",format:RowFormat::Tsv,fresh:Freshness::Days(1),row:PhantomData};
    let dir=tempfile::tempdir().unwrap();
    let path=dir.path().join("cache.sqlite");
    let fetch=Fetch::open(FetchConfig{cache:Some(path.clone()),..Default::default()}).unwrap();
    std::thread::scope(|scope| {
        let run=scope.spawn(|| fetch.sync(&bundle).unwrap());
        server.next().respond(200,"ETag: \"one\"\r\n","symbol\tband\tname\nBRAF\t9p21.1\t\"B raf kinase\"\nOTHER\t19p21\tOther\n");
        assert_eq!(run.join().unwrap().rows,2);
    });
    let leg=fetch.leg(spec,fetch.deadline(Duration::from_secs(2)));
    assert_eq!(leg.rows(&bundle,Find::Exact(KeyKind::Symbol,"braf")).unwrap().rows[0].symbol,"BRAF");
    assert_eq!(leg.rows(&bundle,Find::Prefix(KeyKind::Band,"9p21.")).unwrap().rows.len(),1);
    assert!(leg.rows(&bundle,Find::Prefix(KeyKind::Band,"%")).unwrap().rows.is_empty());
    for term in ["\"", "*", "AND", "NEAR("] { let _=leg.rows(&bundle,Find::Words(term)).unwrap(); }
    assert_eq!(leg.rows(&bundle,Find::Words("kinase")).unwrap().rows.len(),1);
    let offline=Fetch::open(FetchConfig{cache:Some(path),mode:biomcp_fetch::Mode::Offline,..Default::default()}).unwrap();
    assert_eq!(offline.leg(spec,offline.deadline(Duration::from_secs(2))).rows(&bundle,Find::Exact(KeyKind::Symbol,"BRAF")).unwrap().rows.len(),1);
}
