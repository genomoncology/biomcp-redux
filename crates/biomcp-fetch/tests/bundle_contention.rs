//! A bundle publisher does not borrow the process-wide reply connection.
#![expect(
    clippy::unwrap_used,
    clippy::disallowed_methods,
    reason = "Public transport contract fixture"
)]
mod support;
use biomcp_fetch::{
    Bundle, Fetch, FetchConfig, Freshness, KeyUse, Keys, Quota, Rate, Row, RowFormat, SourceKey,
};
use serde::Deserialize;
use std::{marker::PhantomData, time::Duration};

#[derive(Deserialize)]
struct Entry {
    symbol: String,
}
impl Row for Entry {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![],
            words: vec![self.symbol.clone().into()],
        }
    }
}
const QUOTA: Quota = Quota::new(
    SourceKey::declare("bundle-contention"),
    Rate::per_second(1000),
    None,
    4,
);

#[test]
fn bundle_publication_finishes_while_another_owner_holds_the_reply_connection() {
    let directory = tempfile::tempdir().unwrap();
    let server = support::Server::<()>::new();
    let bundle = Bundle::<Entry> {
        spec: server.spec(&QUOTA, KeyUse::None),
        name: "genes",
        path: "/genes",
        format: RowFormat::Tsv,
        fresh: Freshness::Days(1),
        row: PhantomData,
    };
    let fetch = Fetch::open(FetchConfig {
        cache: Some(directory.path().join("cache.sqlite")),
        sync_budget: Duration::from_secs(5),
        ..Default::default()
    })
    .unwrap();
    let access = fetch.cache_access().unwrap();
    std::thread::scope(|scope| {
        let sync = scope.spawn(|| fetch.sync(&bundle));
        let request = server.next();
        access
            .admit(fetch.deadline(Duration::from_secs(10)), || {
                // The request proves claim has finished. Keep the ordinary SQL
                // connection held while a multi-batch publication completes.
                let body = format!("symbol\n{}", "BRAF\n".repeat(600));
                request.respond(200, "", &body);
                let answer = scope.spawn(|| {
                    let leg = fetch.leg(bundle.spec, fetch.deadline(Duration::from_secs(5)));
                    let endpoint = bundle.spec.get("/answer").fresh(Freshness::Hours(1));
                    assert!(leg.json::<support::Answer>(&endpoint, &[]).unwrap().answer);
                    leg.finish()
                });
                server.next().ok();
                assert!(answer.join().unwrap().failures.is_empty());
                assert_eq!(sync.join().unwrap().unwrap().rows, 600);
                Ok(())
            })
            .unwrap();
    });
}
