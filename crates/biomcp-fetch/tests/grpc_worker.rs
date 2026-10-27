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
    source: Source::MyGene,
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

#[path = "grpc_worker/support.rs"]
mod support;
use support::{Check, Fixture, Hold, Seen};

#[test]
fn stream_worker_uses_added_trust_and_filters_provider_metadata() -> Check {
    let fixture = Fixture::new(Hold::None, 0, false, false)?;
    let source = fixture.source(false, KeyUse::None);
    let fetch = Fetch::open(FetchConfig {
        network: fixture.network(None, "")?,
        ..FetchConfig::default()
    })?;
    let leg = fetch.leg(source, fetch.deadline(Duration::from_secs(5)));
    let mut messages = Vec::new();
    leg.grpc(
        &source.grpc("/synthetic.Tensor/Exchange"),
        &[8, 7],
        |bytes| {
            messages.push(bytes.to_vec());
            Ok(())
        },
    )?;
    assert_eq!(messages, [vec![8, 42]]);
    let mut request = None;
    for _ in 0..3 {
        if let Seen::Request(headers, bytes) = fixture.received()? {
            request = Some((headers, bytes));
        }
    }
    let (headers, bytes) = request.unwrap();
    assert!(headers.contains_key("grpc-timeout"));
    assert_eq!(bytes, [0, 0, 0, 0, 2, 8, 7]);
    assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
    fixture.finish()
}

#[test]
fn stream_worker_refuses_untrusted_certificates_and_missing_h2() -> Check {
    for (hold, trusted) in [(Hold::None, false), (Hold::NoAlpn, true)] {
        let fixture = Fixture::new(hold, 0, false, false)?;
        let network = if trusted {
            fixture.network(None, "")?
        } else {
            biomcp_fetch::Network::default()
        };
        let fetch = Fetch::open(FetchConfig {
            network,
            ..FetchConfig::default()
        })?;
        let source = fixture.source(false, KeyUse::None);
        let leg = fetch.leg(source, fetch.deadline(Duration::from_secs(5)));
        let error = leg
            .grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
            .unwrap_err();
        assert!(
            matches!(error, biomcp_fetch::RequestError::Source(error) if error.reason == Reason::Tls)
        );
        assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
        fixture.finish()?;
    }
    Ok(())
}

#[test]
fn stream_worker_routes_explicit_environment_and_bypass_proxies() -> Check {
    use biomcp_fetch::Secret;
    for (explicit, proxy_tls, bypass) in [
        (true, false, false),
        (false, false, false),
        (false, true, false),
        (false, false, true),
    ] {
        let fixture = Fixture::new(Hold::None, 0, !bypass, proxy_tls)?;
        let source = fixture.source(!bypass, KeyUse::None);
        let proxy = format!(
            "{}://user:synthetic-proxy@{}",
            if proxy_tls { "https" } else { "http" },
            fixture.address
        );
        let network = fixture.network(
            if explicit {
                None
            } else {
                Some(Secret::new(if bypass {
                    "http://127.0.0.1:1".to_owned()
                } else {
                    proxy.clone()
                }))
            },
            if bypass { "127.0.0.1" } else { "" },
        )?;
        let fetch = Fetch::open(FetchConfig {
            network,
            proxy: explicit.then(|| Secret::new(proxy)),
            ..FetchConfig::default()
        })?;
        let leg = fetch.leg(source, fetch.deadline(Duration::from_secs(5)));
        leg.grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))?;
        let mut connect = None;
        for _ in 0..if bypass { 3 } else { 4 } {
            match fixture.received()? {
                Seen::Connect(header) => connect = Some(header),
                Seen::Request(headers, _) => assert!(!headers.contains_key("proxy-authorization")),
                _ => {}
            }
        }
        if bypass {
            assert!(connect.is_none());
        } else {
            let connect = connect.unwrap();
            assert!(connect.starts_with("CONNECT synthetic.test:443 HTTP/1.1"));
            assert!(connect.contains("Proxy-Authorization: Basic "));
        }
        fixture.finish()?;
    }
    Ok(())
}

#[test]
fn stream_worker_maps_status_without_retrying_or_reflecting_provider_text() -> Check {
    use biomcp_fetch::{Credential, KeyPlace, Secret, SecretName, Secrets};
    for (status, reason) in [
        (8, Reason::RateLimited),
        (7, Reason::Rejected),
        (16, Reason::Rejected),
        (14, Reason::Unavailable),
        (4, Reason::DeadlineExceeded),
        (5, Reason::NotFound),
        (13, Reason::BadReply),
    ] {
        let fixture = Fixture::new(Hold::None, status, false, false)?;
        let key = KeyUse::Required(Credential {
            secret: SecretName::OncokbApiKey,
            place: KeyPlace::Header("x-fixture-key"),
        });
        let source = fixture.source(false, key);
        let mut secrets = Secrets::default();
        secrets.insert(
            SecretName::OncokbApiKey,
            Secret::new("echo-synthetic-secret"),
        );
        let fetch = Fetch::open(FetchConfig {
            secrets,
            network: fixture.network(None, "")?,
            ..FetchConfig::default()
        })?;
        let leg = fetch.leg(source, fetch.deadline(Duration::from_secs(5)));
        let error = leg
            .grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
            .unwrap_err();
        assert!(
            matches!(error, biomcp_fetch::RequestError::Source(error) if error.reason == reason)
        );
        assert!(!format!("{error:?} {fetch:?}").contains("echo-synthetic-secret"));
        for _ in 0..3 {
            if let Seen::Request(headers, _) = fixture.received()? {
                assert_eq!(
                    headers.get("x-fixture-key").unwrap(),
                    "echo-synthetic-secret"
                );
            }
        }
        assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
        fixture.finish()?;
    }
    Ok(())
}

#[test]
fn stream_worker_uses_one_absolute_deadline_in_each_response_phase() -> Check {
    for hold in [
        Hold::Tls,
        Hold::Connect,
        Hold::Upload,
        Hold::Headers,
        Hold::Message,
        Hold::Trailers,
    ] {
        let fixture = Fixture::new(hold, 0, hold == Hold::Connect, false)?;
        let source = fixture.source(hold == Hold::Connect, KeyUse::None);
        let fetch = Fetch::open(FetchConfig {
            network: fixture.network(
                (hold == Hold::Connect)
                    .then(|| biomcp_fetch::Secret::new(format!("http://{}", fixture.address))),
                "",
            )?,
            grpc_limits: biomcp_fetch::grpc::Limits {
                request_bytes: 256 * 1024,
                ..Default::default()
            },
            ..FetchConfig::default()
        })?;
        let leg = fetch.leg(source, fetch.deadline(Duration::from_secs(1)));
        let body = if hold == Hold::Upload {
            vec![0; 128 * 1024]
        } else {
            Vec::new()
        };
        let error = leg
            .grpc(
                &source.grpc("/synthetic.Tensor/Exchange"),
                &body,
                |_| Ok(()),
            )
            .unwrap_err();
        assert!(
            matches!(error, biomcp_fetch::RequestError::Source(error) if error.reason == Reason::DeadlineExceeded)
        );
        let seen = fixture.events.try_iter().collect::<Vec<_>>();
        assert!(seen.iter().any(|event| matches!(event, Seen::Tcp)));
        if !matches!(hold, Hold::Tls | Hold::Connect) {
            assert!(
                seen.iter()
                    .any(|event| matches!(event, Seen::Request(_, _)))
            );
        }
        if hold == Hold::Trailers {
            assert!(seen.iter().any(|event| matches!(event, Seen::Message)));
        }
        assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
        fixture.finish()?;
    }
    Ok(())
}

#[test]
fn stream_worker_refuses_interrupted_stream_without_resending() -> Check {
    let fixture = Fixture::new(Hold::Interrupt, 0, false, false)?;
    let source = fixture.source(false, KeyUse::None);
    let fetch = Fetch::open(FetchConfig {
        network: fixture.network(None, "")?,
        ..FetchConfig::default()
    })?;
    let leg = fetch.leg(source, fetch.deadline(Duration::from_secs(5)));
    assert!(
        leg.grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
            .is_err()
    );
    assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
    fixture.finish()
}

#[test]
fn stream_worker_rejects_unrepresentable_bounds_without_allocating_payloads() {
    use biomcp_fetch::grpc::Limits;
    let request_max = (u32::MAX as usize).saturating_sub(tonic::codec::HEADER_SIZE);
    for limits in [
        Limits {
            queued_calls: tokio::sync::Semaphore::MAX_PERMITS + 1,
            ..Default::default()
        },
        Limits {
            request_bytes: request_max + 1,
            ..Default::default()
        },
    ] {
        assert!(matches!(
            Fetch::open(FetchConfig {
                grpc_limits: limits,
                ..Default::default()
            }),
            Err(biomcp_fetch::FetchError::StreamingLimits)
        ));
    }
    assert!(
        Fetch::open(FetchConfig {
            grpc_limits: Limits {
                queued_calls: tokio::sync::Semaphore::MAX_PERMITS,
                request_bytes: request_max,
                ..Default::default()
            },
            ..Default::default()
        })
        .is_ok()
    );
}

#[test]
fn stream_worker_holds_the_shared_permit_until_stream_cancellation() -> Check {
    let first = Fixture::new(Hold::Trailers, 0, false, false)?;
    let second = Fixture::new(Hold::None, 0, false, false)?;
    let roots = biomcp_fetch::Certificates::from_pem(
        format!("{}{}", first.certificate, second.certificate).as_bytes(),
    )?;
    let network = biomcp_fetch::Network::new(None, None, "", roots)?;
    let fetch = Fetch::open(FetchConfig {
        network,
        ..Default::default()
    })?;
    let first_source = first.source(false, KeyUse::None);
    let second_source = second.source(false, KeyUse::None);
    let end = fetch.deadline(Duration::from_secs(2));
    std::thread::scope(|scope| -> Check {
        let first_call = scope.spawn(|| {
            fetch.leg(first_source, end).grpc(
                &first_source.grpc("/synthetic.Tensor/Exchange"),
                &[],
                |_| Ok(()),
            )
        });
        first.wait_message()?;
        fetch
            .leg(second_source, fetch.deadline(Duration::from_secs(4)))
            .grpc(
                &second_source.grpc("/synthetic.Tensor/Exchange"),
                &[],
                |_| {
                    assert!(
                        end.expired(),
                        "second stream bypassed the first stream's permit"
                    );
                    Ok(())
                },
            )?;
        let error = first_call.join().unwrap().unwrap_err();
        assert!(
            matches!(error, biomcp_fetch::RequestError::Source(error) if error.reason == Reason::DeadlineExceeded)
        );
        Ok(())
    })?;
    first.finish()?;
    second.finish()
}

#[test]
fn stream_worker_shutdown_cancels_and_joins_active_network_work() -> Check {
    let fixture = Fixture::new(Hold::Headers, 0, false, false)?;
    let fetch = Fetch::open(FetchConfig {
        network: fixture.network(None, "")?,
        ..Default::default()
    })?;
    let source = fixture.source(false, KeyUse::None);
    std::thread::scope(|scope| -> Check {
        let call = scope.spawn(|| {
            fetch
                .leg(source, fetch.deadline(Duration::from_secs(5)))
                .grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
        });
        fixture.wait_request()?;
        fetch.shutdown_grpc();
        assert!(call.join().unwrap().is_err());
        assert!(
            fetch
                .leg(source, fetch.deadline(Duration::from_secs(1)))
                .grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
                .is_err()
        );
        Ok(())
    })?;
    assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
    fixture.finish()
}

static QUEUE_QUOTA: Quota = Quota::new(
    SourceKey::declare("grpc-queue"),
    Rate::per_second(100),
    None,
    1,
);
#[test]
fn stream_worker_queue_deadlines_return_without_waiting_for_another_stream() -> Check {
    let fixture = Fixture::new(Hold::Trailers, 0, false, false)?;
    let fetch = Fetch::open(FetchConfig {
        network: fixture.network(None, "")?,
        grpc_limits: biomcp_fetch::grpc::Limits {
            queued_calls: 1,
            in_flight_streams: 1,
            ..Default::default()
        },
        ..Default::default()
    })?;
    let (first_fetch, cancel) = fetch.cancellable()?;
    let source = fixture.source(false, KeyUse::None);
    let queued = fixture.with_quota(false, KeyUse::None, &QUEUE_QUOTA);
    std::thread::scope(|scope| -> Check {
        let first = scope.spawn(|| {
            first_fetch
                .leg(source, first_fetch.deadline(Duration::from_secs(5)))
                .grpc(&source.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
        });
        fixture.wait_message()?;
        for _ in 0..2 {
            let error = fetch
                .leg(queued, fetch.deadline(Duration::from_millis(100)))
                .grpc(&queued.grpc("/synthetic.Tensor/Exchange"), &[], |_| Ok(()))
                .unwrap_err();
            assert!(
                matches!(error, biomcp_fetch::RequestError::Source(error) if error.reason == Reason::DeadlineExceeded)
            );
            assert!(
                !first.is_finished(),
                "queued caller waited for the unrelated active stream"
            );
        }
        cancel.cancel();
        assert!(first.join().unwrap().is_err());
        Ok(())
    })?;
    assert_eq!(fixture.count.load(std::sync::atomic::Ordering::SeqCst), 1);
    fixture.finish()
}
