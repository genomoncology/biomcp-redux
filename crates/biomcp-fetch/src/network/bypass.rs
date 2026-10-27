//! Parse proxy bypass boundaries once for every transport route.
use crate::FetchError;
use std::net::IpAddr;
#[derive(Clone)]
pub(super) struct Bypass {
    host: Host,
    port: Option<u16>,
}
#[derive(Clone)]
enum Host {
    Any,
    Address(IpAddr),
    Network(IpAddr, u8),
    Name { name: String, suffix: bool, bare: bool },
}
impl Bypass {
    pub(super) fn parse(text: &str) -> Result<Option<Self>, FetchError> {
        if text.contains('@') { return Err(FetchError::ProxyBypass); }
        if text == "*" { return Ok(Some(Self { host: Host::Any, port: None })); }
        if let Ok(address) = text.parse::<IpAddr>() {
            return Ok(Some(Self { host: Host::Address(address), port: None }));
        }
        if let Some((address, prefix)) = text.split_once('/') {
            let Ok(address) = address.parse::<IpAddr>() else { return Ok(None); };
            let Ok(prefix) = prefix.parse::<u8>() else { return Ok(None); };
            if prefix > if address.is_ipv4() { 32 } else { 128 } { return Ok(None); }
            return Ok(Some(Self { host: Host::Network(address, prefix), port: None }));
        }
        let (suffix, bare, text) = if let Some(rest) = text.strip_prefix("*.") {
            (true, false, rest)
        } else if let Some(rest) = text.strip_prefix('.') {
            (true, true, rest)
        } else { (false, true, text) };
        // Inspect ports before hosts so malformed ports never widen rules.
        let (name, tail) = if let Some(rest) = text.strip_prefix('[') {
            let Some((name, tail)) = rest.split_once(']') else { return Ok(None); };
            (name, tail)
        } else if let Some((name, _)) = text.rsplit_once(':') {
            (name, &text[name.len()..])
        } else { (text, "") };
        let port = if tail.is_empty() { None } else {
            Some(tail.strip_prefix(':').and_then(|p| p.parse::<u16>().ok())
                .ok_or(FetchError::ProxyBypass)?)
        };
        let host = if let Ok(address) = name.parse::<IpAddr>() {
            if suffix { return Ok(None); }
            Host::Address(address)
        } else {
            if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.')) {
                return Ok(None);
            }
            Host::Name { name: name.to_ascii_lowercase(), suffix, bare }
        };
        Ok(Some(Self { host, port }))
    }
    pub(super) fn matches(&self, uri: &ureq::http::Uri) -> bool {
        let host = uri.host().unwrap_or("").trim_matches(['[', ']']).to_ascii_lowercase();
        let port = uri.port_u16().unwrap_or(if uri.scheme_str() == Some("https") {443} else {80});
        self.port.is_none_or(|required| required == port) && match &self.host {
            Host::Any => true,
            Host::Address(address) => host.parse::<IpAddr>().is_ok_and(|host| host == *address),
            Host::Network(address, prefix) => host.parse::<IpAddr>().is_ok_and(|host| match (address, host) {
                (IpAddr::V4(network), IpAddr::V4(host)) => {
                    let mask = u32::MAX.checked_shl(u32::from(32 - prefix)).unwrap_or(0);
                    u32::from(*network) & mask == u32::from(host) & mask
                }
                (IpAddr::V6(network), IpAddr::V6(host)) => {
                    let mask = u128::MAX.checked_shl(u32::from(128 - prefix)).unwrap_or(0);
                    u128::from(*network) & mask == u128::from(host) & mask
                }
                _ => false,
            }),
            Host::Name { name, suffix, bare } => (*bare && host == *name)
                || (*suffix && host.parse::<IpAddr>().is_err()
                    && host.strip_suffix(name).is_some_and(|prefix| prefix.ends_with('.'))),
        }
    }
}
