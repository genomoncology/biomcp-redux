//! Separate process proof for a shared SQLite cache; control sockets name events.
#![expect(
    clippy::unwrap_used,
    clippy::panic,
    reason = "Fixture setup, child processes and failed source assertions must abort the contract test"
)]
use super::{Server, config, spec, support::Answer};
use biomcp_fetch::{Arg, Fetch, Freshness, KeyUse, SourceSpec};
use rusqlite::Connection;
#[expect(
    clippy::disallowed_types,
    reason = "Separate test processes must share the same SQLite file"
)]
use std::process::Command;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::{Child, Stdio},
    thread,
    time::{Duration, Instant},
};

struct Worker {
    process: Child,
    control: TcpStream,
}
impl Worker {
    #[expect(
        clippy::disallowed_methods,
        reason = "Launch this integration-test executable as an isolated cache client"
    )]
    #[expect(
        clippy::disallowed_types,
        reason = "A separate OS process is necessary to prove SQLite sharing"
    )]
    fn start(path: &Path, base: &str, key: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut process = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "cache_process::child", "--nocapture"])
            .env("CACHE_CHILD", key)
            .env("CACHE_PATH", path)
            .env("CACHE_BASE", base)
            .env("CACHE_CONTROL", listener.local_addr().unwrap().to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let end = Instant::now() + Duration::from_secs(15);
        let control = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < end =>
                {
                    if let Some(status) = process.try_wait().unwrap() {
                        panic!("child exited before connecting: {status}");
                    }
                    thread::park_timeout(Duration::from_millis(1));
                }
                Err(error) => {
                    let _ = process.kill();
                    let _ = process.wait();
                    panic!("control connection failed: {error}");
                }
            }
        };
        control.set_nonblocking(false).unwrap();
        control
            .set_read_timeout(Some(Duration::from_secs(15)))
            .unwrap();
        control
            .set_write_timeout(Some(Duration::from_secs(15)))
            .unwrap();
        Self { process, control }
    }
    fn send(&mut self, event: u8) {
        self.control.write_all(&[event]).unwrap();
    }
    fn event(&mut self, expected: u8) {
        event(&mut self.control, expected);
    }
    fn finish(mut self) {
        self.send(b'E');
        let status = self.process.wait().unwrap();
        let mut output = String::new();
        self.process
            .stdout
            .take()
            .unwrap()
            .read_to_string(&mut output)
            .unwrap();
        self.process
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut output)
            .unwrap();
        assert!(status.success(), "cache child failed: {output}");
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}
fn event(control: &mut TcpStream, expected: u8) {
    let mut received = [0];
    control.read_exact(&mut received).unwrap();
    assert_eq!(
        received,
        [expected],
        "The cache process must publish its synchronization event"
    );
}

#[test]
fn concurrent_first_opens_and_distinct_writes_survive_into_a_new_process() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("shared.sqlite");
    let server = Server::new();
    let lock = Connection::open(&path).unwrap();
    // Both child first opens encounter an active writer, then proceed together.
    lock.execute_batch("CREATE TABLE unrelated(value TEXT); BEGIN IMMEDIATE")
        .unwrap();
    let mut first = Worker::start(&path, server.base, "first");
    let mut second = Worker::start(&path, server.base, "second");
    for worker in [&mut first, &mut second] {
        worker.send(b'S');
        worker.event(b'O');
    }
    lock.execute_batch("COMMIT").unwrap();
    for worker in [&mut first, &mut second] {
        worker.event(b'A');
    }
    lock.execute_batch("BEGIN IMMEDIATE").unwrap();
    for worker in [&mut first, &mut second] {
        worker.send(b'G');
    }
    // Requests and writes overlap while the shared database writer is held.
    let request_a = server.next();
    let request_b = server.next();
    assert_ne!(request_a.text, request_b.text);
    request_a.ok();
    request_b.ok();
    lock.execute_batch("COMMIT").unwrap();
    for worker in [&mut first, &mut second] {
        worker.event(b'D');
    }
    first.finish();
    second.finish();
    let mut reader = Worker::start(&path, server.base, "read");
    reader.send(b'S');
    reader.event(b'O');
    reader.event(b'A');
    reader.send(b'G');
    reader.event(b'D');
    reader.finish();
    assert!(server.requests.try_recv().is_err());
    assert_eq!(
        lock.query_row("SELECT count(*) FROM reply", [], |r| r.get::<_, u32>(0))
            .unwrap(),
        2
    );
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Only child test processes read their explicitly supplied fixture settings"
)]
fn child() {
    let Ok(key) = std::env::var("CACHE_CHILD") else {
        return;
    };
    let path = std::env::var("CACHE_PATH").unwrap();
    let base = Box::leak(std::env::var("CACHE_BASE").unwrap().into_boxed_str());
    let mut control = TcpStream::connect(std::env::var("CACHE_CONTROL").unwrap()).unwrap();
    control
        .set_read_timeout(Some(Duration::from_secs(15)))
        .unwrap();
    event(&mut control, b'S');
    control.write_all(b"O").unwrap();
    let fetch = Fetch::open(config(Path::new(&path))).unwrap();
    let quota_server = Server::new();
    let template = spec(&quota_server, KeyUse::None);
    let spec = Box::leak(Box::new(SourceSpec { base, ..*template }));
    let endpoint = spec.get("/answer").fresh(Freshness::Hours(1));
    control.write_all(b"A").unwrap();
    event(&mut control, b'G');
    let keys = if key == "read" {
        vec!["first", "second"]
    } else {
        vec![key.as_str()]
    };
    for key in keys {
        assert!(
            fetch
                .leg(spec, fetch.deadline(Duration::from_secs(10)))
                .json::<Answer>(&endpoint, &[("key", Arg::Text(key))])
                .unwrap()
                .answer
        );
    }
    control.write_all(b"D").unwrap();
    event(&mut control, b'E');
}
