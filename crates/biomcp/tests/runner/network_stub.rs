//! A local CONNECT peer terminates TLS with a synthetic private trust anchor.
use serde_json::Value;
use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration};
use rustls::{ServerConfig, ServerConnection, StreamOwned, pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject}};

pub(super) const CERT: &str = include_str!("network-fixtures/ca.pem");
pub(super) struct Stub {
    pub(super) address: std::net::SocketAddr,
    stop: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<Result<Vec<(String, String)>, String>>>,
}
impl Stub {
    pub(super) fn new() -> Result<Self, String> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        let address = listener.local_addr().map_err(|e| e.to_string())?;
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let worker = thread::spawn(move || {
            let certs = CertificateDer::pem_slice_iter(CERT.as_bytes()).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
            let key = PrivateKeyDer::from_pem_slice(include_bytes!("network-fixtures/key.pem")).map_err(|e| e.to_string())?;
            let config = Arc::new(ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
                .with_safe_default_protocol_versions().map_err(|e| e.to_string())?
                .with_no_client_auth().with_single_cert(certs, key).map_err(|e| e.to_string())?);
            let mut requests = Vec::new();
            for stream in listener.incoming() {
                if flag.load(Ordering::Acquire) { break; }
                let mut socket = stream.map_err(|e| e.to_string())?;
                socket.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
                socket.set_write_timeout(Some(Duration::from_secs(10))).map_err(|e| e.to_string())?;
                let connect = header(&mut socket)?;
                if !connect.starts_with("CONNECT storage.googleapis.com:443 ") { return Err("wrong CONNECT destination".into()); }
                socket.write_all(b"HTTP/1.1 200 Connection established\r\n\r\n").map_err(|e| e.to_string())?;
                let connection = ServerConnection::new(config.clone()).map_err(|e| e.to_string())?;
                let mut tls = StreamOwned::new(connection, socket);
                // An untrusted client closes its handshake before sending HTTP.
                if let Ok(request) = header(&mut tls) {
                    let body = body(request.contains("withdrawn.txt"))?;
                    write!(tls, "HTTP/1.1 200 OK\r\nContent-Type: text/tab-separated-values\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).map_err(|e| e.to_string())?;
                    tls.flush().map_err(|e| e.to_string())?;
                    requests.push((connect, request));
                }
            }
            Ok(requests)
        });
        Ok(Self { address, stop, worker: Some(worker) })
    }
    pub(super) fn finish(mut self) -> Result<Vec<(String, String)>, String> {
        self.stop.store(true, Ordering::Release);
        let _ = TcpStream::connect(self.address);
        self.worker.take().ok_or("missing worker")?.join().map_err(|_| "TLS worker panicked")?
    }
}
impl Drop for Stub {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        let _ = TcpStream::connect(self.address);
        if let Some(worker) = self.worker.take() { let _ = worker.join(); }
    }
}
fn header(input: &mut impl Read) -> Result<String, String> {
    let mut bytes = Vec::new();
    while !bytes.ends_with(b"\r\n\r\n") && bytes.len() < 8192 {
        let mut byte = [0];
        input.read_exact(&mut byte).map_err(|e| e.to_string())?;
        bytes.extend(byte);
    }
    String::from_utf8(bytes).map_err(|e| e.to_string())
}
fn body(withdrawn: bool) -> Result<String, String> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/main/hgnc");
    for file in fs::read_dir(root).map_err(|e| e.to_string())? {
        let value: Value = serde_json::from_slice(&fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        if value.get("url").and_then(Value::as_str).is_some_and(|url| url.ends_with("withdrawn.txt") == withdrawn) {
            return value.pointer("/exchanges/0/body/value").and_then(Value::as_str).map(str::to_owned).ok_or("missing fixture body".into());
        }
    }
    Err("missing HGNC fixture".into())
}
