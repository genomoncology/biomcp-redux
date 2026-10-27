//! Feature stream-worker: blocking callers use the shared admission policy before network work.
use biomcp_fetch::{Fetch, FetchConfig, KeyUse, Mode, Quota, Rate, SourceKey, SourceSpec, Terms};
use biomcp_types::{Reason, Source, Use};
use std::time::Duration;

static QUOTA: Quota = Quota::new(
    SourceKey::declare("grpc-synthetic"),
    Rate::per_second(100),
    None,
    1,
);
static SOURCE: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::Mygene,
    terms: Terms {
        name: "Synthetic fixture",
        url: "https://example.test/terms",
        usage: Use::Open,
    },
    base: "https://localhost:1",
    key: KeyUse::None,
    quota: &QUOTA,
    deadline: Duration::from_secs(2),
};

#[test]
fn stream_worker_offline_never_opens_a_connection() {
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        ..FetchConfig::default()
    })
    .unwrap();
    let leg = fetch.leg(&SOURCE, fetch.deadline(Duration::from_secs(1)));
    let error = leg
        .grpc(&SOURCE.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
        .unwrap_err();
    assert!(
        matches!(error, biomcp_fetch::RequestError::Source(error) if error.reason == Reason::Offline)
    );
}
