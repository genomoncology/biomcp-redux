//! Prove joined shutdown without exporting a test-only transport control.
use crate::{Fetch, FetchConfig, KeyUse, Quota, Rate, SourceKey};
use std::time::Duration;
#[path = "support.rs"]
#[expect(
    dead_code,
    reason = "Reuse the existing local TLS fixture for one private shutdown boundary"
)]
#[expect(
    clippy::disallowed_types,
    reason = "The local TLS fixture counts sockets across its owned worker"
)]
mod support;
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
    let source = fixture.source(false, KeyUse::None);
    let call = Call::start(fetch.clone(), source, Duration::from_secs(5))?;
    fixture.wait_request()?;
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
