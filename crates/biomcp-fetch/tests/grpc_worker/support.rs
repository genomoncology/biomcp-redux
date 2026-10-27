//! Own an event-driven local TLS HTTP/2 peer with synthetic protobuf messages.
use biomcp_fetch::{Certificates, KeyUse, Network, SourceSpec, Terms};
use std::{
    net::TcpListener,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tonic::codegen::http;
pub(super) type Check<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Hold {
    None,
    Tls,
    Connect,
    Upload,
    Headers,
    Message,
    Trailers,
    Interrupt,
    NoAlpn,
}
#[derive(Debug)]
#[allow(
    dead_code,
    reason = "Consumers read different events from this shared transport and CLI fixture"
)]
pub(super) enum Seen {
    Tcp,
    Http(String),
    Path(String),
    Connect(String),
    Request(http::HeaderMap, Vec<u8>),
    Message,
    Trailers,
}
pub(super) struct Fixture {
    pub(super) address: std::net::SocketAddr,
    pub(super) certificate: String,
    pub(super) events: mpsc::Receiver<Seen>,
    pub(super) count: Arc<AtomicUsize>,
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    thread: Option<thread::JoinHandle<Check>>,
}
struct Replies {
    messages: Vec<Vec<u8>>,
    http: Option<Vec<(String, Vec<u8>)>>,
    status: u16,
}
#[derive(Debug)]
pub(super) struct HttpReply {
    pub(super) hosts: Vec<String>,
    pub(super) replies: Vec<(String, Vec<u8>)>,
}
trait Io: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T: AsyncRead + AsyncWrite + Send + Unpin> Io for T {}
type Socket = Box<dyn Io>;
impl Fixture {
    pub(super) fn new(hold: Hold, status: u8, proxy: bool, proxy_tls: bool) -> Check<Self> {
        Self::with_messages(hold, status, proxy, proxy_tls, vec![vec![8, 42]])
    }
    pub(super) fn with_messages(
        hold: Hold,
        status: u8,
        proxy: bool,
        proxy_tls: bool,
        messages: Vec<Vec<u8>>,
    ) -> Check<Self> {
        Self::with_http(hold, status, proxy, proxy_tls, messages, None, 200)
    }
    pub(super) fn with_http(
        hold: Hold,
        status: u8,
        proxy: bool,
        proxy_tls: bool,
        messages: Vec<Vec<u8>>,
        http: Option<HttpReply>,
        response_status: u16,
    ) -> Check<Self> {
        let mut hosts = vec![
            "localhost".into(),
            "synthetic.test".into(),
            "127.0.0.1".into(),
        ];
        if let Some(http) = &http {
            hosts.extend(http.hosts.iter().cloned());
        }
        let messages = Arc::new(Replies {
            messages,
            http: http.map(|http| http.replies),
            status: response_status,
        });
        let pair = rcgen::generate_simple_self_signed(hosts)?;
        let certificate = pair.cert.pem();
        let key = rustls::pki_types::PrivatePkcs8KeyDer::from(pair.key_pair.serialize_der());
        let mut tls = rustls::ServerConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(vec![pair.cert.der().clone()], key.into())?;
        if hold != Hold::NoAlpn {
            tls.alpn_protocols = vec![b"h2".to_vec()];
        }
        if messages.http.is_some() {
            tls.alpn_protocols.push(b"http/1.1".to_vec());
        }
        let tls = Arc::new(tls);
        let listener = TcpListener::bind("127.0.0.1:0")?;
        listener.set_nonblocking(true)?;
        let address = listener.local_addr()?;
        let (sent, events) = mpsc::channel();
        let (stop, mut stopping) = tokio::sync::oneshot::channel();
        let count = Arc::new(AtomicUsize::new(0));
        let accepted = count.clone();
        let thread = thread::spawn(move || -> Check {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            runtime.block_on(async move {
                let listener = tokio::net::TcpListener::from_std(listener)?;
                let mut connections = tokio::task::JoinSet::new();
                loop {
                    tokio::select! {
                        _ = &mut stopping => break,
                        stream = listener.accept() => {
                            let (stream, _) = stream?;
                            accepted.fetch_add(1, Ordering::SeqCst);
                            sent.send(Seen::Tcp)?;
                            let (sent, tls, messages) = (sent.clone(), tls.clone(), messages.clone());
                            connections.spawn(async move { serve(Box::new(stream), tls, hold, status, (proxy, proxy_tls), sent, messages).await });
                        }
                    }
                }
                connections.abort_all();
                while connections.join_next().await.is_some() {}
                Ok(())
            })
        });
        Ok(Self {
            address,
            certificate,
            events,
            count,
            stop: Some(stop),
            thread: Some(thread),
        })
    }
    pub(super) fn network(
        &self,
        proxy: Option<biomcp_fetch::Secret>,
        bypass: &str,
    ) -> Check<Network> {
        Ok(Network::new(
            None,
            proxy,
            bypass,
            Certificates::from_pem(self.certificate.as_bytes())?,
        )?)
    }
    pub(super) fn source(&self, proxy: bool, key: KeyUse) -> &'static SourceSpec {
        self.with_quota(proxy, key, &super::QUOTA)
    }
    pub(super) fn with_quota(
        &self,
        proxy: bool,
        key: KeyUse,
        quota: &'static biomcp_fetch::Quota,
    ) -> &'static SourceSpec {
        let base = if proxy {
            "https://synthetic.test:443".to_owned()
        } else {
            format!("https://127.0.0.1:{}", self.address.port())
        };
        Box::leak(Box::new(SourceSpec {
            source: biomcp_types::Source::MyGene,
            user_agent: None,
            terms: Terms {
                name: "Synthetic fixture",
                url: "https://example.test/terms",
                usage: biomcp_types::Use::Open,
            },
            base: Box::leak(base.into_boxed_str()),
            key,
            quota,
            deadline: Duration::from_secs(5),
        }))
    }
    pub(super) fn received(&self) -> Check<Seen> {
        Ok(self.events.recv_timeout(Duration::from_secs(5))?)
    }
    pub(super) fn wait_message(&self) -> Check {
        loop {
            if matches!(self.received()?, Seen::Message) {
                return Ok(());
            }
        }
    }
    pub(super) fn wait_request(&self) -> Check {
        loop {
            if matches!(self.received()?, Seen::Request(_, _)) {
                return Ok(());
            }
        }
    }
    pub(super) fn finish(mut self) -> Check {
        self.stop();
        if let Some(thread) = self.thread.take() {
            thread.join().map_err(|_| "fixture worker panicked")??;
        }
        Ok(())
    }
    fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            // A closed receiver already stopped the peer.
            let _ = stop.send(());
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.stop();
        // Explicit finish surfaces worker failures. During an assertion panic only cleanup remains.
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
async fn serve(
    mut socket: Socket,
    tls: Arc<rustls::ServerConfig>,
    hold: Hold,
    status: u8,
    proxy: (bool, bool),
    events: mpsc::Sender<Seen>,
    messages: Arc<Replies>,
) -> Check {
    if proxy.0 {
        if proxy.1 {
            let mut outer = (*tls).clone();
            outer.alpn_protocols = vec![b"http/1.1".to_vec()];
            socket = Box::new(
                tokio_rustls::TlsAcceptor::from(Arc::new(outer))
                    .accept(socket)
                    .await?,
            );
        }
        let mut header = Vec::new();
        while !header.ends_with(b"\r\n\r\n") {
            if header.len() > 8192 {
                return Err("proxy header exceeded bound".into());
            }
            header.push(socket.read_u8().await?);
        }
        events.send(Seen::Connect(String::from_utf8(header)?))?;
        if hold == Hold::Connect {
            std::future::pending::<()>().await;
        }
        socket
            .write_all(b"HTTP/1.1 200 Connection established\r\n\r\n")
            .await?;
    }
    if hold == Hold::Tls {
        std::future::pending::<()>().await;
    }
    let mut socket = tokio_rustls::TlsAcceptor::from(tls).accept(socket).await?;
    if socket.get_ref().1.alpn_protocol() != Some(b"h2") {
        let replies = messages.http.as_ref().ok_or("unexpected HTTP/1 request")?;
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            if request.len() > 8192 {
                return Err("HTTP request exceeded bound".into());
            }
            request.push(socket.read_u8().await?);
        }
        let request = String::from_utf8(request)?;
        let reply = &replies
            .iter()
            .find(|(prefix, _)| request.starts_with(prefix))
            .ok_or("unexpected HTTP source request")?
            .1;
        events.send(Seen::Http(request))?;
        socket.write_all(format!("HTTP/1.1 200 Synthetic\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", reply.len()).as_bytes()).await?;
        socket.write_all(reply).await?;
        socket.shutdown().await?;
        return Ok(());
    }
    let mut connection = h2::server::handshake(socket).await?;
    let mut replies = tokio::task::JoinSet::new();
    while let Some(request) = connection.accept().await {
        let (request, response) = request?;
        replies.spawn(reply(
            request,
            response,
            hold,
            status,
            events.clone(),
            messages.clone(),
        ));
    }
    while let Some(reply) = replies.join_next().await {
        reply??;
    }
    Ok(())
}
async fn reply(
    request: http::Request<h2::RecvStream>,
    mut response: h2::server::SendResponse<prost::bytes::Bytes>,
    hold: Hold,
    status: u8,
    events: mpsc::Sender<Seen>,
    messages: Arc<Replies>,
) -> Check {
    let (head, mut body) = request.into_parts();
    if hold == Hold::Upload {
        events.send(Seen::Request(head.headers, Vec::new()))?;
        std::future::pending::<()>().await;
        return Ok(());
    }
    let mut request = Vec::new();
    while let Some(data) = body.data().await {
        let data = data?;
        if request.len() + data.len() > 65536 {
            return Err("request exceeded bound".into());
        }
        request.extend_from_slice(&data);
        body.flow_control().release_capacity(data.len())?;
    }
    events.send(Seen::Path(head.uri.path().to_owned()))?;
    events.send(Seen::Request(head.headers, request))?;
    if hold == Hold::Headers {
        std::future::pending::<()>().await;
    }
    let headers = http::Response::builder()
        .status(messages.status)
        .header("content-type", "application/grpc")
        .body(())?;
    let mut stream = response.send_response(headers, false)?;
    if hold == Hold::Message {
        std::future::pending::<()>().await;
    }
    for message in &messages.messages {
        let mut frame = vec![0];
        frame.extend_from_slice(&(message.len() as u32).to_be_bytes());
        frame.extend_from_slice(message);
        stream.send_data(prost::bytes::Bytes::from(frame), false)?;
        events.send(Seen::Message)?;
    }
    if hold == Hold::Trailers {
        std::future::pending::<()>().await;
    }
    if hold == Hold::Interrupt {
        stream.send_reset(h2::Reason::INTERNAL_ERROR);
        return Ok(());
    }
    let mut trailers = http::HeaderMap::new();
    trailers.insert("grpc-status", status.to_string().parse()?);
    trailers.insert("grpc-message", "echo-synthetic-secret".parse()?);
    stream.send_trailers(trailers)?;
    events.send(Seen::Trailers)?;
    Ok(())
}

pub(super) struct Call {
    thread: Option<thread::JoinHandle<Result<(), biomcp_fetch::RequestError>>>,
    pub(super) end: std::time::Instant,
}
impl Call {
    pub(super) fn start(
        fetch: biomcp_fetch::Fetch,
        source: &'static SourceSpec,
        budget: Duration,
    ) -> Check<Self> {
        let (started, start) = mpsc::sync_channel(1);
        let thread = thread::Builder::new()
            .name("fixture-caller".into())
            .spawn(move || {
                let deadline = fetch.deadline(budget);
                // Receiver closure means the owning test has already failed.
                let _ = started.send(std::time::Instant::now() + deadline.remaining());
                fetch.leg(source, deadline).grpc(
                    &source.grpc("/synthetic.Tensor/Exchange"),
                    &[],
                    |_| Ok(()),
                )
            })?;
        let mut call = Self {
            thread: Some(thread),
            end: std::time::Instant::now(),
        };
        call.end = start.recv_timeout(Duration::from_secs(5))?;
        Ok(call)
    }
    pub(super) fn is_finished(&self) -> bool {
        self.thread
            .as_ref()
            .is_none_or(thread::JoinHandle::is_finished)
    }
    pub(super) fn finish(mut self) -> Check<Result<(), biomcp_fetch::RequestError>> {
        Ok(self
            .thread
            .take()
            .ok_or("missing caller")?
            .join()
            .map_err(|_| "caller panicked")?)
    }
}
impl Drop for Call {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
