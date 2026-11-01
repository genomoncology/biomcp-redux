//! Hosted calls retain TLS sessions while their connections remain independent.
use biomcp_fetch::{
    Certificates, Destinations, Fetch, FetchConfig, Input, KeyUse, Network, Quota, Rate, Read,
    Secret, Secrets, SourceKey, SourceSpec, Terms,
};
use biomcp_test_support::network_stub::Stub;
use biomcp_types::{Source, Use};
use std::time::Duration;

type Check = Result<(), Box<dyn std::error::Error>>;

#[test]
fn second_hosted_call_resumes_tls_after_first_calls_connections_close() -> Check {
    let stub = Stub::for_source("example.test", "application/octet-stream", |_| {
        Ok("synthetic".into())
    })?;
    let network = Network::new(
        None,
        Some(Secret::new(format!("http://{}", stub.address))),
        "",
        Certificates::from_pem(stub.certificate.as_bytes())?,
    )?;
    let shared = Fetch::open(FetchConfig {
        network,
        ..Default::default()
    })?
    .with_hosted_secrets(Secrets::default());
    static SPEC: SourceSpec = SourceSpec {
        source: Source::PubMed,
        base: "https://example.test",
        destinations: Destinations::Primary,
        user_agent: None,
        terms: Terms {
            name: "Synthetic test data",
            url: "https://example.test/terms",
            usage: Use::Open,
        },
        key: KeyUse::None,
        quota: &Quota::new(
            SourceKey::declare("shared-tls"),
            Rate::per_second(1000),
            None,
            8,
        ),
        deadline: Duration::from_secs(10),
    };
    let spec = &SPEC;
    for path in ["/first", "/second"] {
        let (call, cancellation) = shared.cancellable()?;
        let reply = call.leg(spec, call.deadline(spec.deadline)).call(
            &spec.get(path).bytes(1024),
            Input::Params(&[]),
            Read::bytes(),
        )?;
        assert_eq!(
            reply.value.0, b"synthetic",
            "Each call must receive its reply"
        );
        cancellation.cancel();
    }
    assert_eq!(
        stub.handshakes(),
        [rustls::HandshakeKind::Full, rustls::HandshakeKind::Resumed],
        "The second call must resume the first call's TLS session"
    );
    assert_eq!(
        stub.finish()?.len(),
        2,
        "Both calls must reach the TLS peer"
    );
    Ok(())
}
