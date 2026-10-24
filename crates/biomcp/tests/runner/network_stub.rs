//! A local CONNECT peer terminates TLS with a synthetic private trust anchor.
use rustls::{ServerConfig, ServerConnection, StreamOwned, pki_types::PrivatePkcs8KeyDer};
use serde_json::Value;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

type Requests = Vec<(String, String)>;

pub(super) struct Stub {
    pub(super) address: std::net::SocketAddr,
    pub(super) certificate: String,
    stop: mpsc::Sender<()>,
    worker: Option<thread::JoinHandle<Result<Requests, String>>>,
}
impl Stub {
    pub(super) fn new() -> Result<Self, String> {
        Self::for_source(
            "storage.googleapis.com",
            "text/tab-separated-values",
            |request| body(request.contains("withdrawn.txt")),
        )
    }
    pub(super) fn for_source(
        host: &'static str,
        content_type: &'static str,
        reply: fn(&str) -> Result<String, String>,
    ) -> Result<Self, String> {
        Self::with_length(host, move |_| content_type, reply, |_, bytes| bytes)
    }
    pub(super) fn with_length(
        host: &'static str,
        content_type: impl Fn(&str) -> &'static str + Send + 'static,
        reply: fn(&str) -> Result<String, String>,
        length: fn(&str, usize) -> usize,
    ) -> Result<Self, String> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
        let address = listener.local_addr().map_err(|e| e.to_string())?;
        let (stop, stopping) = mpsc::channel();
        let (certificate, config) = identity(host)?;
        let worker = thread::spawn(move || {
            let mut requests = Vec::new();
            for stream in listener.incoming() {
                if stopping.try_recv().is_ok() {
                    break;
                }
                let mut socket = stream.map_err(|e| e.to_string())?;
                socket
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .map_err(|e| e.to_string())?;
                socket
                    .set_write_timeout(Some(Duration::from_secs(10)))
                    .map_err(|e| e.to_string())?;
                let connect = header(&mut socket)?;
                if !connect.starts_with(&format!("CONNECT {host}:443 ")) {
                    return Err("wrong CONNECT destination".into());
                }
                socket
                    .write_all(b"HTTP/1.1 200 Connection established\r\n\r\n")
                    .map_err(|e| e.to_string())?;
                let connection =
                    ServerConnection::new(config.clone()).map_err(|e| e.to_string())?;
                let mut tls = StreamOwned::new(connection, socket);
                // An untrusted client closes its handshake before sending HTTP.
                if let Ok(request) = header(&mut tls) {
                    let body = reply(&request)?;
                    let content_type = content_type(&request);
                    write!(tls, "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", length(&request, body.len())).map_err(|e| e.to_string())?;
                    tls.flush().map_err(|e| e.to_string())?;
                    requests.push((connect, request));
                }
            }
            Ok(requests)
        });
        Ok(Self {
            address,
            certificate,
            stop,
            worker: Some(worker),
        })
    }
    pub(super) fn finish(mut self) -> Result<Vec<(String, String)>, String> {
        let _ = self.stop.send(());
        let _ = TcpStream::connect(self.address);
        self.worker
            .take()
            .ok_or("missing worker")?
            .join()
            .map_err(|_| "TLS worker panicked")?
    }
}
impl Drop for Stub {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        let _ = TcpStream::connect(self.address);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
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
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/main/hgnc");
    for file in fs::read_dir(root).map_err(|e| e.to_string())? {
        let value: Value = serde_json::from_slice(
            &fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        if value
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| url.ends_with("withdrawn.txt") == withdrawn)
        {
            return value
                .pointer("/exchanges/0/body/value")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or("missing fixture body".into());
        }
    }
    Err("missing HGNC fixture".into())
}

fn identity(host: &str) -> Result<(String, Arc<ServerConfig>), String> {
    use rcgen::{
        BasicConstraints, CertificateParams, ExtendedKeyUsagePurpose, IsCa, KeyPair,
        KeyUsagePurpose,
    };
    let now = time::OffsetDateTime::now_utc();
    let mut ca = CertificateParams::default();
    // Windows must distinguish the signer from the leaf's default subject name.
    ca.distinguished_name
        .push(rcgen::DnType::CommonName, "Synthetic local test authority");
    ca.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];
    ca.not_before = now - time::Duration::days(1);
    ca.not_after = now + time::Duration::days(2);
    let ca_key = KeyPair::generate().map_err(|e| e.to_string())?;
    let ca = ca.self_signed(&ca_key).map_err(|e| e.to_string())?;
    let mut leaf = CertificateParams::new(vec![host.into()]).map_err(|e| e.to_string())?;
    leaf.not_before = now - time::Duration::days(1);
    leaf.not_after = now + time::Duration::days(1);
    leaf.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    let key = KeyPair::generate().map_err(|e| e.to_string())?;
    let cert = leaf
        .signed_by(&key, &ca, &ca_key)
        .map_err(|e| e.to_string())?;
    let config =
        ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
            .with_safe_default_protocol_versions()
            .map_err(|e| e.to_string())?
            .with_no_client_auth()
            .with_single_cert(
                vec![cert.der().clone()],
                PrivatePkcs8KeyDer::from(key.serialize_der()).into(),
            )
            .map_err(|e| e.to_string())?;
    Ok((ca.pem(), Arc::new(config)))
}
