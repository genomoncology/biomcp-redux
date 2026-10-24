//! Checked blocking source requests with explicit configuration and deadlines.

mod deadline;
mod declaration;
mod error;
mod request;
mod secret;

pub use deadline::Deadline;
pub use declaration::{Arg, Credential, Endpoint, KeyPlace, KeyUse, SecretName, SourceSpec};
pub use error::{BadReply, FetchError, SourceError};
pub use secret::{Secret, Secrets};
use std::{collections::BTreeMap, fmt, time::Duration};

/// A decoded response must verify that it contains a usable answer.
pub trait Reply: serde::de::DeserializeOwned {
    /// Refuse provider error envelopes and incomplete answers.
    fn check(&self) -> Result<(), BadReply>;
}

/// Explicit transport configuration. Construction reads no environment settings.
#[derive(Debug)]
pub struct FetchConfig {
    /// Configured credentials.
    pub secrets: Secrets,
    /// Explicit HTTP proxy URL, redacted when formatted.
    pub proxy: Option<Secret>,
    /// Maximum decoded response body bytes.
    pub max_body: u64,
    /// Per-source budget overrides.
    pub source_deadlines: BTreeMap<biodata::Source, Duration>,
}

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            secrets: Secrets::default(),
            proxy: None,
            max_body: 8 * 1024 * 1024,
            source_deadlines: BTreeMap::new(),
        }
    }
}

/// One blocking client shared across source legs.
pub struct Fetch {
    agent: ureq::Agent,
    config: FetchConfig,
}

impl fmt::Debug for Fetch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fetch")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl Fetch {
    /// Open a client using platform TLS roots and only the supplied proxy.
    pub fn open(config: FetchConfig) -> Result<Self, FetchError> {
        if config.max_body == u64::MAX {
            return Err(FetchError::BodyLimit);
        }
        let proxy = config
            .proxy
            .as_ref()
            .map(|p| ureq::Proxy::new(p.value()))
            .transpose()
            .map_err(|_| FetchError::Proxy)?;
        let agent = ureq::Agent::config_builder()
            .proxy(proxy)
            .http_status_as_error(false)
            .max_redirects(0)
            .tls_config(
                ureq::tls::TlsConfig::builder()
                    .root_certs(ureq::tls::RootCerts::PlatformVerifier)
                    .build(),
            )
            .build()
            .new_agent();
        Ok(Self { agent, config })
    }

    /// Start an absolute monotonic deadline.
    pub fn deadline(&self, budget: Duration) -> Deadline {
        Deadline::after(budget)
    }

    /// Start a leg capped by this source's budget.
    pub fn leg(&self, spec: &'static SourceSpec, deadline: Deadline) -> Leg<'_> {
        let cap = self
            .config
            .source_deadlines
            .get(&spec.source)
            .copied()
            .unwrap_or(spec.deadline);
        let no_key = match spec.key {
            KeyUse::Required(credential) => self.config.secrets.get(credential.secret).is_none(),
            _ => false,
        };
        Leg {
            fetch: self,
            spec,
            deadline: deadline.child(cap),
            no_key,
        }
    }
}

/// One source's requests under a shared absolute deadline.
#[derive(Debug)]
pub struct Leg<'a> {
    fetch: &'a Fetch,
    spec: &'static SourceSpec,
    deadline: Deadline,
    no_key: bool,
}
