//! Checked reply cache through the public transport API and documented SQLite schema.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    reason = "Fixture setup, child processes and failed source assertions must abort the contract test"
)]
#![expect(
    clippy::disallowed_methods,
    reason = "Scoped callers exercise public cache behavior against controlled local peers"
)]
#[path = "support/cache_process.rs"]
mod cache_process;
mod support;
use biomcp_fetch::{
    Arg, BadReply, CacheError, Credential, Endpoint, Fetch, FetchConfig, FetchError, Freshness,
    KeyPlace, KeyUse, Mode, Quota, Rate, Reply, RequestError, Secret, SecretName, SourceKey,
    SourceSpec,
};
use biomcp_types::Reason;
use rusqlite::Connection;
use std::{path::Path, thread, time::Duration};
use support::Answer;
type Server = support::Server<()>;

fn spec(server: &Server, key: KeyUse) -> &'static SourceSpec {
    server.spec(
        Box::leak(Box::new(Quota::new(
            SourceKey::declare("cache"),
            Rate::per_second(1000),
            None,
            8,
        ))),
        key,
    )
}
fn config(path: &Path) -> FetchConfig {
    FetchConfig {
        cache: Some(path.to_owned()),
        ..FetchConfig::default()
    }
}
fn get(
    fetch: &Fetch,
    spec: &'static SourceSpec,
    endpoint: &Endpoint,
    params: &[(&str, Arg<'_>)],
) -> Result<Answer, RequestError> {
    fetch
        .leg(spec, fetch.deadline(Duration::from_secs(10)))
        .json(endpoint, params)
}
fn exchange<T: Send>(
    server: &Server,
    status: u16,
    body: &str,
    call: impl FnOnce() -> T + Send,
) -> T {
    thread::scope(|scope| {
        let worker = scope.spawn(call);
        let request = server.next();
        assert!(request.text.starts_with("GET /answer"));
        request.respond(status, "", body);
        worker.join().unwrap()
    })
}
fn expire(path: &Path) {
    Connection::open(path)
        .unwrap()
        .execute("UPDATE reply SET fresh_until=0", [])
        .unwrap();
}
fn source(error: RequestError) -> biomcp_fetch::SourceError {
    let RequestError::Source(error) = error else {
        panic!("expected provider failure: {error}");
    };
    error
}

#[path = "cache/credentials.rs"]
mod credentials;
#[path = "cache/freshness.rs"]
mod freshness;
#[path = "cache/storage.rs"]
mod storage;

#[path = "cache/contention.rs"]
mod contention;
