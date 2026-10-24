//! Publisher URLs and DNS answers checked at the connection boundary.
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use ureq::unversioned::{
    resolver::{DefaultResolver, ResolvedSocketAddrs, Resolver},
    transport::NextTimeout,
};

pub(crate) fn checked(raw: &str) -> Result<String, crate::InputError> {
    let url = url::Url::parse(raw).map_err(|_| crate::InputError::Path)?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || !matches!(url.host(), Some(url::Host::Domain(_)))
        || url.host_str().is_none_or(|host| {
            host.is_empty() || host.split('.').any(|label| label.is_empty())
        })
    {
        return Err(crate::InputError::Path);
    }
    Ok(url.into())
}

#[derive(Debug, Default)]
pub(crate) struct PublicResolver(DefaultResolver);
impl Resolver for PublicResolver {
    fn resolve(
        &self,
        uri: &ureq::http::Uri,
        config: &ureq::config::Config,
        timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        checked(&uri.to_string()).map_err(|_| ureq::Error::HostNotFound)?;
        let addresses = self.0.resolve(uri, config, timeout)?;
        if addresses.iter().any(|address| !public(address.ip())) {
            return Err(ureq::Error::HostNotFound);
        }
        // The TCP connector receives these exact checked socket addresses.
        // It never resolves the publisher hostname again.
        Ok(addresses)
    }
}
fn public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => v4(ip),
        IpAddr::V6(ip) => v6(ip),
    }
}
fn v4(ip: Ipv4Addr) -> bool {
    let n = u32::from(ip);
    const RESERVED: &[(u32, u32)] = &[
        (0x0000_0000, 8), (0x0a00_0000, 8), (0x6440_0000, 10),
        (0x7f00_0000, 8), (0xa9fe_0000, 16), (0xac10_0000, 12),
        (0xc000_0000, 24), (0xc000_0200, 24), (0xc058_6300, 24),
        (0xc0a8_0000, 16), (0xc612_0000, 15), (0xc633_6400, 24),
        (0xcb00_7100, 24), (0xe000_0000, 3),
    ];
    !RESERVED.iter().any(|(network, prefix)| n >> (32 - prefix) == network >> (32 - prefix))
}
fn v6(ip: Ipv6Addr) -> bool {
    let n = u128::from(ip);
    // Require native global unicast, excluding protocol assignments, documentation
    // and transition ranges that could tunnel to an unvalidated IPv4 destination.
    n >> 125 == 1
        && n >> 96 != 0x2001_0db8
        && n >> 105 != 0x2001_0000_u128 >> 9
        && n >> 112 != 0x2002
        && n >> 108 != 0x3fff0
}
