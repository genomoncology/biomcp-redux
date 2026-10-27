//! Open exactly one checked proxy route with the shared platform TLS verifier.
use super::{Limits, dns};
use base64::Engine;
use biomcp_types::Reason;
use rustls::{ClientConfig, pki_types::ServerName};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub(super) trait Io: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Io for T {}
pub(super) type Socket = Box<dyn Io>;

pub(super) async fn open(
    uri: &ureq::http::Uri,
    proxy: Option<&ureq::Proxy>,
    tls: Arc<ClientConfig>,
    limits: Limits,
) -> Result<Socket, Reason> {
    let host = uri.host().ok_or(Reason::Rejected)?.trim_matches(['[', ']']);
    let port = uri.port_u16().unwrap_or(443);
    let mut socket: Socket = if let Some(proxy) = proxy {
        let address = proxy.uri();
        let name = address
            .host()
            .ok_or(Reason::Proxy)?
            .trim_matches(['[', ']']);
        let secure = address.scheme_str() == Some("https");
        let port = address.port_u16().unwrap_or(if secure { 443 } else { 80 });
        let socket = tcp(name, port).await.map_err(|_| Reason::Proxy)?;
        if secure {
            let mut config = (*tls).clone();
            config.alpn_protocols = vec![b"http/1.1".to_vec()];
            handshake(socket, name, Arc::new(config), false)
                .await
                .map_err(|_| Reason::Proxy)?
        } else {
            socket
        }
    } else {
        tcp(host, port).await?
    };
    if let Some(proxy) = proxy {
        tunnel(&mut socket, uri, proxy, limits).await?;
    }
    handshake(socket, host, tls, true).await
}
async fn tcp(host: &str, port: u16) -> Result<Socket, Reason> {
    let address = dns::address(host, port).await?;
    // One chosen address, one connection attempt. No address failover or retry.
    tokio::net::TcpStream::connect(address)
        .await
        .map(|socket| Box::new(socket) as Socket)
        .map_err(|_| Reason::Unavailable)
}
async fn handshake(
    socket: Socket,
    host: &str,
    config: Arc<ClientConfig>,
    h2: bool,
) -> Result<Socket, Reason> {
    let name = ServerName::try_from(host.to_owned()).map_err(|_| Reason::Tls)?;
    let stream = tokio_rustls::TlsConnector::from(config)
        .connect(name, socket)
        .await
        .map_err(|_| Reason::Tls)?;
    if h2 && stream.get_ref().1.alpn_protocol() != Some(b"h2") {
        return Err(Reason::Tls);
    }
    Ok(Box::new(stream))
}
async fn tunnel(
    socket: &mut Socket,
    uri: &ureq::http::Uri,
    proxy: &ureq::Proxy,
    limits: Limits,
) -> Result<(), Reason> {
    let host = uri.host().ok_or(Reason::Proxy)?;
    let target = format!("{host}:{}", uri.port_u16().unwrap_or(443));
    let mut request = format!("CONNECT {target} HTTP/1.1\r\nHost: {target}\r\n");
    if let Some(username) = proxy.username() {
        let password = proxy.password().unwrap_or("");
        if username.len().saturating_add(password.len()) > limits.metadata_bytes / 2 {
            return Err(Reason::Proxy);
        }
        let auth =
            base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));
        request.push_str(&format!("Proxy-Authorization: Basic {auth}\r\n"));
    }
    request.push_str("\r\n");
    if request.len() > limits.metadata_bytes {
        return Err(Reason::Proxy);
    }
    socket
        .write_all(request.as_bytes())
        .await
        .map_err(|_| Reason::Proxy)?;
    let mut header = Vec::new();
    while !header.ends_with(b"\r\n\r\n") {
        if header.len() == limits.metadata_bytes {
            return Err(Reason::Proxy);
        }
        let byte = socket.read_u8().await.map_err(|_| Reason::Proxy)?;
        header.push(byte);
    }
    let line = header
        .split(|byte| *byte == b'\n')
        .next()
        .ok_or(Reason::Proxy)?;
    let mut fields = line.split(|byte| *byte == b' ');
    if !matches!(fields.next(), Some(b"HTTP/1.1" | b"HTTP/1.0")) || fields.next() != Some(b"200") {
        return Err(Reason::Proxy);
    }
    Ok(())
}
