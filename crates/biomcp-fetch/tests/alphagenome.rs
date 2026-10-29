//! Feature alphagenome-protocol: required credential routing uses a synthetic TLS peer.
#[path = "alphagenome/routing.rs"]
mod routing;
#[path = "grpc_worker/support.rs"]
#[expect(
    dead_code,
    reason = "Reuse the local TLS fixture without provider traffic"
)]
mod support;
static QUOTA: biomcp_fetch::Quota = biomcp_fetch::Quota::new(
    biomcp_fetch::SourceKey::declare("key-routing-synthetic"),
    biomcp_fetch::Rate::per_second(1000),
    None,
    1,
);
type Check = Result<(), Box<dyn std::error::Error + Send + Sync>>;
