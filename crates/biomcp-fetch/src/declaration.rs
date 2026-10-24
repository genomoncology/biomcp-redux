use biodata::Source;
use std::time::Duration;

/// A source declaration enforced by the transport.
#[derive(Debug)]
pub struct SourceSpec {
    /// Source reported on failures.
    pub source: Source,
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
        }
    }
    /// Declare a form POST endpoint. Retries require explicit idempotence.
    pub const fn post_form(&'static self, path: &'static str) -> Endpoint {
        Endpoint {
            method: Method::Form,
            idempotent: false,
            ..self.get(path)
        }
    }
    /// Declare a JSON POST endpoint. Retries require explicit idempotence.
    pub const fn post_json(&'static self, path: &'static str) -> Endpoint {
        Endpoint {
            method: Method::Json,
            idempotent: false,
            ..self.get(path)
        }
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
}

/// Credential insertion location.
#[derive(Clone, Copy, Debug)]
pub enum KeyPlace {
    /// URL query field.
    Query(&'static str),
    /// HTTP request header.
    Header(&'static str),
}

/// A parameter value. Identifier sets are sorted before sending.
#[derive(Debug)]
pub enum Arg<'a> {
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
        if !path.starts_with('/') || path.starts_with("//")
            || !path.bytes().all(|ch| ch.is_ascii_alphanumeric() || b"/-._~".contains(&ch))
            || path.split('/').any(|part| matches!(part, "." | ".."))
        {
            return Err(crate::InputError::Path);
        }
        self.path = std::borrow::Cow::Owned(path);
        Ok(self)
    }
}
