//! Checked blocking source requests with explicit configuration and deadlines.

mod cache;
mod cache_request;
mod deadline;
mod declaration;
mod error;
mod prepared;
mod quota;
mod record;
mod request;
mod retry;
mod secret;
pub mod xml;
#[cfg(test)]
mod xml_tests;

pub use cache::VERSION;
pub use deadline::Deadline;
pub use declaration::{
    Arg, Credential, Endpoint, Freshness, KeyPlace, KeyUse, SecretName, SourceSpec,
};
pub use error::{
    BadReply, CacheError, FetchError, InputError, RecordError, RequestError, SourceError,
};
pub use quota::{Quota, Rate, SourceKey};
pub use record::ReplayCache;
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
    /// Explicit SQLite cache path; absence disables storage.
    pub cache: Option<std::path::PathBuf>,
    /// Live request policy.
    pub mode: Mode,
    /// Owned case cache for replay writes or later offline reads.
    pub replay_cache: Option<ReplayCache>,
    /// Explicit HTTP proxy URL, redacted when formatted.
    pub proxy: Option<Secret>,
    /// Maximum encoded request and decoded response bytes, each bounded independently.
    pub max_body: u64,
    /// Per-source budget overrides.
    pub source_deadlines: BTreeMap<biodata::Source, Duration>,
}

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            secrets: Secrets::default(),
            cache: None,
            mode: Mode::default(),
            replay_cache: None,
            proxy: None,
            max_body: 8 * 1024 * 1024,
            source_deadlines: BTreeMap::new(),
        }
    }
}

/// One blocking client shared across source legs.
pub struct Fetch {
    agent: ureq::Agent,
    limiters: quota::Limiters,
    cache: Option<std::sync::Arc<cache::Cache>>,
    recordings: record::Recordings,
    clock: Option<deadline::VirtualClock>,
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
        let recordings = record::Recordings::open(&config.mode).map_err(FetchError::Recording)?;
        let cache = if matches!(config.mode, Mode::Replay { .. } | Mode::Offline)
            && config.replay_cache.is_some()
        {
            config.replay_cache.as_ref().map(|cache| cache.cache())
        } else if matches!(config.mode, Mode::Replay { .. }) {
            None
        } else {
            config
                .cache
                .as_deref()
                .map(cache::Cache::open)
                .transpose()
                .map_err(FetchError::Cache)?
                .map(std::sync::Arc::new)
        };
        let clock =
            matches!(config.mode, Mode::Replay { .. }).then(deadline::VirtualClock::default);
        Ok(Self {
            agent,
            cache,
            recordings,
            clock,
            config,
            limiters: quota::Limiters::default(),
        })
    }

    /// Create a real deadline or an initial replay budget for independent legs.
    pub fn deadline(&self, budget: Duration) -> Deadline<'_> {
        Deadline::start(budget, self.clock.as_ref())
    }

    /// Start a leg capped by this source's budget.
    /// Replay legs spend independent virtual budgets and share provider quota schedules.
    /// Requests reject deadlines from a different clock domain or replay client.
    #[expect(
        clippy::disallowed_types,
        reason = "Create the shared leg report lock without holding it during request work"
    )]
    pub fn leg<'a>(&'a self, spec: &'static SourceSpec, deadline: Deadline<'a>) -> Leg<'a> {
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
            clock: deadline.fork(),
            wrong_clock: !deadline.matches(self.clock.as_ref()),
            no_key,
            report: std::sync::Mutex::new(LegReport::default()),
        }
    }
}

/// One source's requests under a shared absolute deadline.
#[expect(
    clippy::disallowed_types,
    reason = "Keep typed report state shared across concurrent requests; never hold its lock during request work"
)]
#[derive(Debug)]
pub struct Leg<'a> {
    fetch: &'a Fetch,
    spec: &'static SourceSpec,
    deadline: Deadline<'a>,
    clock: Option<deadline::VirtualClock>,
    wrong_clock: bool,
    no_key: bool,
    report: std::sync::Mutex<LegReport>,
}

/// Request policy for live provider access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Capture each provider response in a new or empty directory.
    Record {
        /// Exclusive capture directory.
        dir: std::path::PathBuf,
    },
    /// Read only captured exchanges with virtual time.
    Replay {
        /// Existing recording directory.
        dir: std::path::PathBuf,
    },
    /// Serve only checked cache entries, including stale positive replies.
    Offline,
    /// Request a provider using the checked cache policy.
    Live {
        /// Skip cached reads and stale fallback.
        fresh: bool,
    },
}
impl Default for Mode {
    fn default() -> Self {
        Self::Live { fresh: false }
    }
}

/// The retained stale state of a completed source leg.
#[derive(Debug, Default)]
pub struct LegReport {
    /// Whether any request served a checked stale reply.
    pub stale: bool,
    /// Original typed provider failures that caused stale replies to be served.
    pub failures: Vec<SourceError>,
}
impl Leg<'_> {
    pub(crate) fn budget(&self) -> Deadline<'_> {
        self.deadline.bind(self.clock.as_ref())
    }
    /// Return this source leg's remaining real or virtual budget.
    pub fn remaining(&self) -> Duration {
        self.budget().remaining()
    }
    /// Finish the leg without clearing failures after later successful requests.
    pub fn finish(self) -> LegReport {
        self.report
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
