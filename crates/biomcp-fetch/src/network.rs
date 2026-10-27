//! Checked process network settings and separate pools for each route.
use crate::{FetchError, Secret};
use rustls::pki_types::{CertificateDer, pem::PemObject};
use std::{fmt, sync::Arc};

/// Checked additional trust anchors. Formatting never exposes certificate bytes.
#[derive(Clone, Default)]
pub struct Certificates(Vec<CertificateDer<'static>>);
impl Certificates {
    /// Decode an explicit PEM bundle and reject empty or malformed certificate sets.
    pub fn from_pem(bytes: &[u8]) -> Result<Self, FetchError> {
        let certs: Vec<_> = CertificateDer::pem_slice_iter(bytes)
            .collect::<Result<_, _>>()
            .map_err(|_| FetchError::Certificates)?;
        if certs.is_empty() {
            return Err(FetchError::Certificates);
        }
        let mut roots = rustls::RootCertStore::empty();
        for cert in &certs {
            roots
                .add(cert.clone())
                .map_err(|_| FetchError::Certificates)?;
        }
        Ok(Self(certs))
    }
}
impl fmt::Debug for Certificates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Certificates")
            .field("count", &self.0.len())
            .finish()
    }
}
/// Owned, checked routing and trust settings; construction never reads the environment.
#[derive(Clone, Default)]
pub struct Network {
    http: Option<ureq::Proxy>,
    https: Option<ureq::Proxy>,
    bypass: Vec<Bypass>,
    roots: Certificates,
}
impl fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Network")
            .field("roots", &self.roots)
            .finish_non_exhaustive()
    }
}
impl Network {
    pub(crate) fn requires_proxy(&self, explicit: bool, uri: &ureq::http::Uri) -> bool {
        !matches!(self.route(explicit, uri), Route::Direct)
    }
    fn route(&self, explicit: bool, uri: &ureq::http::Uri) -> Route {
        if !explicit && self.bypass.iter().any(|rule| rule.matches(uri)) {
            return Route::Direct;
        }
        if uri.scheme_str() == Some("https") {
            if explicit || self.https.is_some() {
                Route::Https
            } else {
                Route::Direct
            }
        } else if explicit || self.http.is_some() {
            Route::Http
        } else {
            Route::Direct
        }
    }
    pub(crate) fn rpc_proxy(
        &self,
        explicit: Option<&Secret>,
        uri: &ureq::http::Uri,
    ) -> Result<Option<ureq::Proxy>, FetchError> {
        if explicit.is_some() {
            return proxy(explicit);
        }
        Ok(match self.route(false, uri) {
            Route::Direct => None,
            Route::Http => self.http.clone(),
            Route::Https => self.https.clone(),
        })
    }
    pub(crate) fn rpc_tls(&self) -> Result<rustls::ClientConfig, FetchError> {
        crate::tls::configuration(self.roots.0.clone())
    }

    /// Check HTTP/HTTPS proxy URLs, comma-separated bypass rules and additive roots.
    /// Bypass supports exact hosts/IPs, .domain (including the domain), *.domain
    /// (subdomains only), optional ports, and a lone * for every destination.
    pub fn new(
        http: Option<Secret>,
        https: Option<Secret>,
        no_proxy: &str,
        roots: Certificates,
    ) -> Result<Self, FetchError> {
        Ok(Self {
            http: proxy(http.as_ref())?,
            https: proxy(https.as_ref())?,
            roots,
            bypass: no_proxy
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(Bypass::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}
fn proxy(value: Option<&Secret>) -> Result<Option<ureq::Proxy>, FetchError> {
    value
        .map(|value| {
            let proxy = ureq::Proxy::new(value.value()).map_err(|_| FetchError::Proxy)?;
            if !matches!(proxy.uri().scheme_str(), Some("http" | "https")) {
                return Err(FetchError::Proxy);
            }
            Ok(proxy)
        })
        .transpose()
}
#[derive(Clone)]
struct Bypass {
    host: String,
    port: Option<u16>,
    suffix: bool,
    bare: bool,
}
impl Bypass {
    fn parse(text: &str) -> Result<Self, FetchError> {
        if text == "*" {
            return Ok(Self {
                host: String::new(),
                port: None,
                suffix: true,
                bare: true,
            });
        }
        let (suffix, bare, text) = if let Some(rest) = text.strip_prefix("*.") {
            (true, false, rest)
        } else if let Some(rest) = text.strip_prefix('.') {
            (true, true, rest)
        } else {
            (false, true, text)
        };
        let authority: ureq::http::uri::Authority = text.parse().map_err(|_| FetchError::Proxy)?;
        let host = authority
            .host()
            .trim_matches(['[', ']'])
            .to_ascii_lowercase();
        if host.is_empty()
            || host.contains(['*', '/', '@'])
            || (suffix && host.parse::<std::net::IpAddr>().is_ok())
        {
            return Err(FetchError::Proxy);
        }
        // Authority accepts a nonnumeric port; refuse rather than silently widen a rule.
        let tail = if text.starts_with('[') {
            text.split_once(']')
                .map(|(_, tail)| tail)
                .ok_or(FetchError::Proxy)?
        } else {
            text.strip_prefix(authority.host())
                .ok_or(FetchError::Proxy)?
        };
        let port = if tail.is_empty() {
            None
        } else {
            Some(
                tail.strip_prefix(':')
                    .ok_or(FetchError::Proxy)?
                    .parse::<u16>()
                    .map_err(|_| FetchError::Proxy)?,
            )
        };
        Ok(Self {
            host,
            port,
            suffix,
            bare,
        })
    }
    fn matches(&self, uri: &ureq::http::Uri) -> bool {
        let host = uri
            .host()
            .unwrap_or("")
            .trim_matches(['[', ']'])
            .to_ascii_lowercase();
        let port = uri
            .port_u16()
            .unwrap_or(if uri.scheme_str() == Some("https") {
                443
            } else {
                80
            });
        self.port.is_none_or(|required| required == port)
            && ((self.bare && host == self.host)
                || (self.suffix
                    && (self.host.is_empty() || host.parse::<std::net::IpAddr>().is_err())
                    && (self.host.is_empty()
                        || host
                            .strip_suffix(&self.host)
                            .is_some_and(|prefix| prefix.ends_with('.')))))
    }
}
pub(crate) struct Agents {
    direct: ureq::Agent,
    publisher: ureq::Agent,
    http: ureq::Agent,
    https: ureq::Agent,
}
impl Agents {
    pub(crate) fn shutdown(&self) {
        self.direct.clone().shutdown();
        self.publisher.clone().shutdown();
        self.http.clone().shutdown();
        self.https.clone().shutdown();
    }
    pub(crate) fn new(network: &Network, explicit: Option<&Secret>) -> Result<Self, FetchError> {
        let tls = Arc::new(crate::tls::configuration(network.roots.0.clone())?);
        let explicit = proxy(explicit)?;
        let make = |proxy| {
            use ureq::unversioned::{
                resolver::DefaultResolver,
                transport::{Connector, TcpConnector},
            };
            let config = ureq::Agent::config_builder()
                .proxy(proxy)
                .http_status_as_error(false)
                .max_redirects(0)
                .build();
            let connector =
                ().chain(crate::tls::ProxyConnector::default())
                    .chain(TcpConnector::default())
                    .chain(crate::tls::TlsConnector(tls.clone()));
            ureq::Agent::with_parts(config, connector, DefaultResolver::default())
        };
        let publisher = {
            use ureq::unversioned::transport::{Connector, TcpConnector};
            let config = ureq::Agent::config_builder()
                .proxy(None)
                .http_status_as_error(false)
                .max_redirects(0)
                .build();
            let connector =
                ().chain(TcpConnector::default())
                    .chain(crate::tls::TlsConnector(tls.clone()));
            ureq::Agent::with_parts(
                config,
                connector,
                crate::destination::PublicResolver::new(network.clone(), explicit.is_some()),
            )
        };
        Ok(Self {
            direct: make(None),
            publisher,
            http: make(explicit.clone().or_else(|| network.http.clone())),
            https: make(explicit.or_else(|| network.https.clone())),
        })
    }
    pub(crate) fn publisher(&self) -> &ureq::Agent {
        &self.publisher
    }
    pub(crate) fn for_url(&self, network: &Network, explicit: bool, url: &str) -> &ureq::Agent {
        let Ok(uri) = url.parse::<ureq::http::Uri>() else {
            return &self.direct;
        };
        match network.route(explicit, &uri) {
            Route::Direct => &self.direct,
            Route::Http => &self.http,
            Route::Https => &self.https,
        }
    }
}

#[derive(Clone, Copy)]
enum Route {
    Direct,
    Http,
    Https,
}
