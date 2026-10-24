use biodata::Source;
use std::time::Duration;

/// A source declaration enforced by the transport.
#[derive(Debug)]
pub struct SourceSpec {
    /// Source reported on failures.
    pub source: Source,
    /// Publisher-declared terms, reviewed by the source adapter.
    pub terms: Terms,
    /// Absolute base URL.
    pub base: &'static str,
    /// Credential policy.
    pub key: KeyUse,
    /// Provider limit shared with all sources using its key.
    pub quota: &'static crate::Quota,
    /// Maximum duration of this source's leg.
    pub deadline: Duration,
}

impl SourceSpec {
    /// Declare a JSON GET endpoint belonging to this source.
    pub const fn get(&'static self, path: &'static str) -> Endpoint {
        Endpoint {
            spec: self,
            path: std::borrow::Cow::Borrowed(path),
            fresh: Freshness::Never,
            method: Method::Get,
            idempotent: true,
            identity_encoding: false,
            octet_stream: false,
            byte_limit: None,
        }
    }
    /// Declare a form POST endpoint. Retries require explicit idempotence.
    pub const fn post_form(&'static self, path: &'static str) -> Endpoint {
        let mut endpoint = self.get(path);
        endpoint.method = Method::Form;
        endpoint.idempotent = false;
        endpoint
    }
    /// Declare a JSON POST endpoint. Retries require explicit idempotence.
    pub const fn post_json(&'static self, path: &'static str) -> Endpoint {
        let mut endpoint = self.get(path);
        endpoint.method = Method::Json;
        endpoint.idempotent = false;
        endpoint
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Method {
    Get,
    Form,
    Json,
}

/// A declared endpoint, always attached to its source.
#[derive(Clone, Debug)]
pub struct Endpoint {
    pub(crate) spec: &'static SourceSpec,
    pub(crate) path: std::borrow::Cow<'static, str>,
    pub(crate) fresh: Freshness,
    pub(crate) method: Method,
    pub(crate) idempotent: bool,
    pub(crate) identity_encoding: bool,
    pub(crate) octet_stream: bool,
    pub(crate) byte_limit: Option<u64>,
}

/// Whether a source requires a credential before sending.
#[derive(Clone, Copy, Debug)]
pub enum KeyUse {
    /// No credential is sent.
    None,
    /// Send the credential when configured.
    Optional(Credential),
    /// Refuse all requests when the credential is absent.
    Required(Credential),
}

/// The credential name and its placement in an outgoing request.
#[derive(Clone, Copy, Debug)]
pub struct Credential {
    /// Configured secret to use.
    pub secret: SecretName,
    /// Where the provider expects its credential.
    pub place: KeyPlace,
}

/// Configuration names of supported secrets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum SecretName {
    /// NCBI API key.
    NcbiApiKey,
    /// Semantic Scholar API key.
    SemanticScholarApiKey,
    /// OncoKB API key.
    OncokbApiKey,
}

/// Credential insertion location.
#[derive(Clone, Copy, Debug)]
pub enum KeyPlace {
    /// URL query field.
    Query(&'static str),
    /// HTTP request header.
    Header(&'static str),
    /// Authorization bearer token.
    Bearer,
}

/// A parameter value. Identifier sets are sorted before sending.
#[derive(Debug)]
pub enum Arg<'a> {
    /// Outgoing-only contact, excluded from persistent identities.
    Contact(&'a crate::Secret),
    /// Literal text.
    Text(&'a str),
    /// Unsigned integer.
    Num(u64),
    /// Identifier set joined by commas in lexical order.
    Ids(&'a [Box<str>]),
}

impl Arg<'_> {
    pub(crate) fn value(&self) -> String {
        match self {
            Self::Contact(_) => String::new(),
            Self::Text(value) => (*value).to_owned(),
            Self::Num(value) => value.to_string(),
            Self::Ids(values) => {
                let mut values: Vec<_> = values.iter().map(AsRef::as_ref).collect();
                values.sort_unstable();
                values.join(",")
            }
        }
    }
}

/// How long a checked endpoint reply remains fresh.
#[derive(Clone, Copy, Debug, Default)]
pub enum Freshness {
    /// Bypass all cache reads and writes.
    #[default]
    Never,
    /// Fresh for this many minutes.
    Minutes(u16),
    /// Fresh for this many hours.
    Hours(u16),
    /// Fresh for this many days.
    Days(u16),
}
impl Freshness {
    pub(crate) fn seconds(self) -> Option<u64> {
        match self {
            Self::Never => None,
            Self::Minutes(n) => Some(u64::from(n) * 60),
            Self::Hours(n) => Some(u64::from(n) * 3600),
            Self::Days(n) => Some(u64::from(n) * 86400),
        }
    }
}
impl Endpoint {
    /// Declare a bounded opaque binary reply independently of JSON/XML limits.
    pub const fn bytes(mut self, max_bytes: u64) -> Self {
        self.byte_limit = Some(max_bytes);
        self
    }
    /// Accept the reviewed bucket media type while retaining JSON/XML decoding and body limits.
    pub const fn octet_stream(mut self) -> Self {
        self.octet_stream = true;
        self
    }
    /// Request an uncompressed representation from a source with broken compression.
    pub const fn identity_encoding(mut self) -> Self {
        self.identity_encoding = true;
        self
    }
    /// Permit retries for a POST that only reads provider data.
    pub const fn idempotent(mut self) -> Self {
        self.idempotent = true;
        self
    }
    /// Set the checked reply's cache freshness.
    pub const fn fresh(mut self, fresh: Freshness) -> Self {
        self.fresh = fresh;
        self
    }
}

impl Endpoint {
    /// Replace only the path, retaining the declared source, method and policy.
    /// Accept canonical absolute paths with unreserved ASCII segments only.
    pub fn with_path(mut self, path: impl Into<String>) -> Result<Self, crate::InputError> {
        let path = path.into();
        if !path.starts_with('/')
            || path.starts_with("//")
            || !path
                .bytes()
                .all(|ch| ch.is_ascii_alphanumeric() || b"/-._~".contains(&ch))
            || path.split('/').any(|part| matches!(part, "." | ".."))
        {
            return Err(crate::InputError::Path);
        }
        self.path = std::borrow::Cow::Owned(path);
        Ok(self)
    }
}

/// Publisher evidence for a source's reuse category; not a grant of rights.
#[derive(Clone, Copy, Debug)]
pub struct Terms {
    /// Publisher license or terms name.
    pub name: &'static str,
    /// Publisher page documenting the terms.
    pub url: &'static str,
    /// Reviewed category used by the source switch.
    pub usage: biodata::Use,
}
