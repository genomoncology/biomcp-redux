//! Strict Host authorities with explicit ports and normalized names.
use std::net::{IpAddr, SocketAddr};
#[derive(PartialEq, Eq)]
pub(super) struct Authority {
    name: String,
    port: Option<u16>,
}
impl Authority {
    pub(super) fn parse(value: &str) -> Option<Self> {
        if value.is_empty() || !value.is_ascii() || value.bytes().any(|b| b.is_ascii_whitespace()) {
            return None;
        }
        let (name, port) = if let Some(rest) = value.strip_prefix('[') {
            let (name, suffix) = rest.split_once(']')?;
            let ip: std::net::Ipv6Addr = name.parse().ok()?;
            let port = if suffix.is_empty() {
                None
            } else {
                Some(suffix.strip_prefix(':')?.parse().ok()?)
            };
            (ip.to_string(), port)
        } else {
            let (name, port) = match value.split_once(':') {
                Some((name, port)) => (name, Some(port.parse().ok()?)),
                None => (value, None),
            };
            if name.is_empty()
                || name.len() > 253
                || !name.split('.').all(|label| {
                    !label.is_empty()
                        && label.len() <= 63
                        && !label.starts_with('-')
                        && !label.ends_with('-')
                        && label
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
                })
            {
                return None;
            }
            (name.to_ascii_lowercase(), port)
        };
        if port == Some(0) {
            return None;
        }
        Some(Self { name, port })
    }
    pub(super) fn permits(&self, requested: &Self) -> bool {
        self.name == requested.name && self.port.is_none_or(|port| requested.port == Some(port))
    }
    pub(super) fn loopback(&self, address: SocketAddr) -> bool {
        (self.name == "localhost" || self.name.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback()))
            && self.port.unwrap_or(80) == address.port()
    }
}
