//! Prove joined shutdown without exporting a test-only transport control.
use crate::{Fetch, FetchConfig, KeyUse, Quota, Rate, SourceKey};
use biomcp_test_support::grpc as support;
use std::time::Duration;
use support::{Call, Check, Fixture, Hold};
static QUOTA: Quota = Quota::new(
    SourceKey::declare("grpc-shutdown"),
    Rate::per_second(100),
    None,
    1,
);
#[test]
fn stream_worker_shutdown_cancels_and_joins_active_network_work() -> Check {
    let fixture = Fixture::new(Hold::Headers, 0, false, false)?;
    let fetch = Fetch::open(FetchConfig {
        network: fixture.network(None, "")?,
        ..Default::default()
    })?;
    let source = fixture.with_quota(false, KeyUse::None, &QUOTA);
    let call = Call::start(fetch.clone(), source, Duration::from_secs(5))?;
    while !matches!(fixture.received()?, support::Seen::Request(_, _)) {}
    fetch.grpc_worker.shutdown();
    assert!(call.finish()?.is_err());
    assert!(
        fetch
            .leg(source, fetch.deadline(Duration::from_secs(1)))
            .grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
            .is_err()
    );
    assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
    fixture.finish()
}
