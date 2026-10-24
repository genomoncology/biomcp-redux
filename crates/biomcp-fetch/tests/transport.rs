//! Public transport contract against isolated local servers.
use biodata::{Reason, Source};
use biomcp_fetch::{
    Arg, BadReply, Credential, Fetch, FetchConfig, KeyPlace, KeyUse, Reply, Secret, SecretName,
    SourceSpec,
};
use serde::Deserialize;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

#[derive(Debug, Deserialize)]
struct Answer {
    answer: Option<String>,
}
impl Reply for Answer {
    fn check(&self) -> Result<(), BadReply> {
        self.answer
            .as_ref()
            .filter(|v| !v.is_empty())
            .map(|_| ())
            .ok_or(BadReply)
    }
}

struct Stub {
    base: &'static str,
    requests: mpsc::Receiver<String>,
    release: Option<mpsc::Sender<()>>,
    worker: Option<thread::JoinHandle<()>>,
}
#[expect(clippy::unwrap_used, reason = "Fixture setup and worker failures must abort the contract test")]
impl Stub {
    fn new(status: u16, content_type: &str, body: &str, blocked: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = Box::leak(format!("http://{}", listener.local_addr().unwrap()).into_boxed_str());
        let reply = format!(
            "HTTP/1.1 {status} Stub\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let (sent, requests) = mpsc::channel();
        let (release, wait) = mpsc::channel();
        let worker = thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .unwrap();
                let mut request = Vec::new();
                let mut byte = [0];
                while stream.read(&mut byte).unwrap_or(0) == 1 {
                    request.push(byte[0]);
                    if request.ends_with(b"\r\n\r\n") {
                        break;
                    }
                }
                if request.is_empty() {
                    break;
                }
                sent.send(String::from_utf8(request).unwrap()).unwrap();
                if blocked {
                    let _ = wait.recv();
                }
                let _ = stream.write_all(reply.as_bytes());
            }
        });
        Self {
            base,
            requests,
            release: Some(release),
            worker: Some(worker),
        }
    }
    fn spec(&self, key: KeyUse, deadline: Duration) -> &'static SourceSpec {
        Box::leak(Box::new(SourceSpec {
            source: Source::PubMed,
            base: self.base,
            key,
            deadline,
        }))
    }
    fn finish(mut self) -> Vec<String> {
        self.stop();
        self.requests.try_iter().collect()
    }
    fn stop(&mut self) {
        self.release.take();
        let _ = TcpStream::connect(self.base.trim_start_matches("http://"));
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }
}
impl Drop for Stub {
    fn drop(&mut self) {
        self.stop();
    }
}

#[expect(clippy::unwrap_used, reason = "Default fixture configuration must open successfully")]
fn fetch() -> Fetch {
    Fetch::open(FetchConfig::default()).unwrap()
}
fn credential(place: KeyPlace) -> Credential {
    Credential {
        secret: SecretName::NcbiApiKey,
        place,
    }
}

#[test]
fn checked_json_encodes_parameters_and_sorts_identifier_sets() {
    let stub = Stub::new(
        200,
        "application/json; charset=utf-8",
        r#"{"answer":"yes"}"#,
        false,
    );
    let spec = stub.spec(KeyUse::None, Duration::from_secs(10));
    let fetch = fetch();
    let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
    let ids = ["b".into(), "a".into()];
    let reply: Answer = leg
        .json(
            &spec.get("/answer"),
            &[
                ("ids", Arg::Ids(&ids)),
                ("text", Arg::Text("a b&c")),
                ("n", Arg::Num(2)),
            ],
        )
        .unwrap();
    assert_eq!(reply.answer.as_deref(), Some("yes"));
    let requests = stub.finish();
    assert_eq!(requests.len(), 1);
    assert!(
        requests[0].starts_with("GET /answer?ids=a%2Cb&text=a%20b%26c&n=2 HTTP/1.1"),
        "{}",
        requests[0]
    );
}

#[test]
fn http_failures_preserve_status_and_optional_404_is_absent() {
    for (status, reason) in [
        (404, Reason::NotFound),
        (400, Reason::Rejected),
        (401, Reason::Rejected),
        (429, Reason::RateLimited),
        (500, Reason::Unavailable),
        (503, Reason::Unavailable),
        (302, Reason::Rejected),
    ] {
        let stub = Stub::new(status, "application/json", "credential-echo", false);
        let spec = stub.spec(KeyUse::None, Duration::from_secs(10));
        let fetch = fetch();
        let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
        let error = leg.json::<Answer>(&spec.get("/answer"), &[]).unwrap_err();
        assert_eq!((error.reason, error.status), (reason, Some(status)));
        if status == 404 {
            assert!(
                leg.json_opt::<Answer>(&spec.get("/answer"), &[])
                    .unwrap()
                    .is_none()
            );
        }
    }
}

#[test]
fn unusable_success_replies_are_bad_reply() {
    for (content_type, body, limit) in [
        ("text/html", "<html>error</html>", 100),
        ("application/json", "{broken", 100),
        ("application/json", r#"{"ERROR":"failed"}"#, 100),
        ("application/problem+json", r#"{"answer":"too large"}"#, 5),
    ] {
        let stub = Stub::new(200, content_type, body, false);
        let spec = stub.spec(KeyUse::None, Duration::from_secs(10));
        let fetch = Fetch::open(FetchConfig {
            max_body: limit,
            ..FetchConfig::default()
        })
        .unwrap();
        let error = fetch
            .leg(spec, fetch.deadline(Duration::from_secs(10)))
            .json::<Answer>(&spec.get("/answer"), &[])
            .unwrap_err();
        assert_eq!((error.reason, error.status), (Reason::BadReply, Some(200)));
    }
}

#[test]
fn refusals_send_zero_requests() {
    for refusal in ["no_key", "expired", "foreign"] {
        let stub = Stub::new(200, "application/json", r#"{"answer":"yes"}"#, false);
        let key = if refusal == "no_key" {
            KeyUse::Required(credential(KeyPlace::Query("api_key")))
        } else {
            KeyUse::None
        };
        let spec = stub.spec(key, Duration::from_secs(10));
        let foreign = stub.spec(KeyUse::None, Duration::from_secs(10));
        let fetch = fetch();
        let deadline = fetch.deadline(if refusal == "expired" {
            Duration::ZERO
        } else {
            Duration::from_secs(10)
        });
        let endpoint = if refusal == "foreign" {
            foreign.get("/answer")
        } else {
            spec.get("/answer")
        };
        let error = fetch
            .leg(spec, deadline)
            .json::<Answer>(&endpoint, &[])
            .unwrap_err();
        assert_eq!(
            error.reason,
            match refusal {
                "no_key" => Reason::NoKey,
                "expired" => Reason::DeadlineExceeded,
                _ => Reason::Rejected,
            }
        );
        assert!(stub.finish().is_empty());
    }
}

#[test]
fn source_budget_caps_supplied_deadline_while_waiting_for_response() {
    let stub = Stub::new(200, "application/json", r#"{"answer":"late"}"#, true);
    let spec = stub.spec(KeyUse::None, Duration::from_millis(40));
    let fetch = fetch();
    let error = fetch
        .leg(spec, fetch.deadline(Duration::from_secs(30)))
        .json::<Answer>(&spec.get("/answer"), &[])
        .unwrap_err();
    assert_eq!(error.reason, Reason::DeadlineExceeded);
    // The server stays blocked until after the call returns. No elapsed-time assertion.
    assert_eq!(stub.finish().len(), 1);
}

#[test]
fn configured_credentials_are_sent_and_all_public_formatting_redacts_them() {
    let secret = "placeholder-only-abc123";
    for place in [KeyPlace::Query("api_key"), KeyPlace::Header("x-api-key")] {
        let stub = Stub::new(500, "application/json", secret, false);
        let spec = stub.spec(KeyUse::Required(credential(place)), Duration::from_secs(10));
        let mut config = FetchConfig::default();
        config
            .secrets
            .insert(SecretName::NcbiApiKey, Secret::new(secret));
        assert!(!format!("{config:?}").contains(secret));
        let fetch = Fetch::open(config).unwrap();
        let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
        let error = leg.json::<Answer>(&spec.get("/answer"), &[]).unwrap_err();
        for formatted in [
            format!("{fetch:?}"),
            format!("{leg:?}"),
            format!("{error:?}"),
            error.to_string(),
            Secret::new(secret).to_string(),
        ] {
            assert!(!formatted.contains(secret));
        }
        let requests = stub.finish();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].contains(secret));
    }
}
