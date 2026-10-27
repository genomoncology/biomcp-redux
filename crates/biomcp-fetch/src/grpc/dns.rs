//! Keep one system resolver slot with the documented bounded cancellation exception.
use biomcp_types::Reason;
use std::{
    io,
    net::{IpAddr, SocketAddr, ToSocketAddrs},
};

struct Resolve(tokio::task::JoinHandle<io::Result<SocketAddr>>);
impl Drop for Resolve {
    fn drop(&mut self) {
        // This cancels queued jobs. An already started system lookup may finish
        // after cancellation under the platform resolver timeout (the PM ruling).
        self.0.abort();
    }
}
pub(super) async fn address(host: &str, port: u16) -> Result<SocketAddr, Reason> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(SocketAddr::new(ip, port));
    }
    let host = host.to_owned();
    resolve(move || {
        (host.as_str(), port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::ErrorKind::AddrNotAvailable.into())
    })
    .await
}
async fn resolve(
    lookup: impl FnOnce() -> io::Result<SocketAddr> + Send + 'static,
) -> Result<SocketAddr, Reason> {
    let mut task = Resolve(tokio::task::spawn_blocking(lookup));
    (&mut task.0)
        .await
        .map_err(|_| Reason::Unavailable)?
        .map_err(|_| Reason::Unavailable)
}
