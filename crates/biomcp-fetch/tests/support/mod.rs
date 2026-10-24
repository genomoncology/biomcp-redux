//! Event-driven local HTTP server; each observed request owns its response release.
#![expect(
    clippy::unwrap_used,
    reason = "Fixture setup and worker failures must fail the contract test"
)]
use biomcp_fetch::{BadReply, KeyUse, Quota, Reply, SourceSpec};
use serde::Deserialize;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Deserialize)]
pub(super) struct Answer {
    pub(super) answer: bool,
}
impl Reply for Answer {
    fn check(&self) -> Result<(), BadReply> {
        if self.answer { Ok(()) } else { Err(BadReply) }
    }
}

pub(super) struct Request {
    pub(super) text: String,
    #[allow(dead_code, reason = "Only quota and retry contracts inspect request timing")]
    pub(super) started: Instant,
    reply: mpsc::Sender<String>,
}
impl Request {
    pub(super) fn respond(self, status: u16, headers: &str, body: &str) {
        let response = format!(
            "HTTP/1.1 {status} Stub\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",
            body.len()
        );
        let _ = self.reply.send(response);
    }
    #[allow(
        dead_code,
        reason = "Only retry contracts send malformed raw HTTP replies"
    )]
    pub(super) fn raw(self, response: &str) {
        let _ = self.reply.send(response.to_owned());
    }
    pub(super) fn ok(self) {
        self.respond(200, "", r#"{"answer":true}"#);
    }
}

pub(super) struct Server {
    pub(super) base: &'static str,
    pub(super) requests: mpsc::Receiver<Request>,
    worker: Option<thread::JoinHandle<()>>,
}
impl Server {
    pub(super) fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = Box::leak(format!("http://{}", listener.local_addr().unwrap()).into_boxed_str());
        let (sent, requests) = mpsc::channel();
        let worker = thread::spawn(move || {
            let mut handlers = Vec::new();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut byte = [0];
                while stream.read(&mut byte).unwrap_or(0) == 1 {
                    bytes.push(byte[0]);
                    if bytes.ends_with(b"\r\n\r\n") {
                        break;
                    }
                }
                if bytes.is_empty() {
                    break;
                }
                let (reply, receive) = mpsc::channel();
                if sent
                    .send(Request {
                        text: String::from_utf8(bytes).unwrap(),
                        started: Instant::now(),
                        reply,
                    })
                    .is_err()
                {
                    break;
                }
                handlers.push(thread::spawn(move || {
                    if let Ok(response) = receive.recv_timeout(Duration::from_secs(10)) {
                        let _ = stream.write_all(response.as_bytes());
                    }
                }));
            }
            for handler in handlers {
                handler.join().unwrap();
            }
        });
        Self {
            base,
            requests,
            worker: Some(worker),
        }
    }
    pub(super) fn next(&self) -> Request {
        self.requests.recv_timeout(Duration::from_secs(10)).unwrap()
    }
    pub(super) fn spec(&self, quota: &'static Quota, key: KeyUse) -> &'static SourceSpec {
        Box::leak(Box::new(SourceSpec {
            source: biodata::Source::PubMed,
            base: self.base,
            key,
            quota,
            deadline: Duration::from_secs(10),
        }))
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        // Drop any unclaimed replies before joining their handlers.
        while self.requests.try_recv().is_ok() {}
        let _ = TcpStream::connect(self.base.trim_start_matches("http://"));
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }
}
