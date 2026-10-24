//! Checked blocking source requests with explicit configuration and deadlines.

mod observe;
pub use observe::{MAX_OBSERVATIONS, Observation, Observations, Shape};
mod batch;
mod bundle;
mod cache;
mod cache_access;
pub use cache_access::CacheAccess;
mod cache_request;
mod deadline;
mod declaration;
mod error;
mod network;
mod prepared;
mod tls;
pub use network::{Certificates, Network};
mod quota;
mod record;
mod request;
mod retry;
mod secret;
pub mod xml;

pub use batch::MAX_BATCH_WORKERS;
pub use bundle::{Bundle, BundleInfo, Find, KeyKind, Keys, MAX_BUNDLE_BYTES, Row, RowFormat};
pub use cache::{SchemaOwner, VERSION, check_owners};
pub use deadline::{CommandDeadline, Deadline, LocalDeadline};
pub use declaration::{
    Arg, Credential, Endpoint, Freshness, KeyPlace, KeyUse, SecretName, SourceSpec, Terms,
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
    /// Explicit bounded wire summaries, disabled by default.
    pub observations: Option<std::sync::Arc<Observations>>,
    /// Other schema owners checked before any cache mutation.
    pub cache_owners: Vec<SchemaOwner>,
    /// Source identities switched off before keys, caches, replay or requests.
    pub disabled_sources: std::collections::BTreeSet<biodata::Source>,
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
    /// Checked environment routes and additive trust certificates.
    pub network: Network,
    /// Maximum encoded request and decoded response bytes, each bounded independently.
    pub max_body: u64,
    /// Independent explicit bundle synchronization budget.
    pub sync_budget: Duration,
    /// Maximum automatically downloaded bundle bytes.
    pub auto_sync_max_bytes: u64,
    /// Per-source budget overrides.
    pub source_deadlines: BTreeMap<biodata::Source, Duration>,
}

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            observations: None,
            cache_owners: Vec::new(),
            disabled_sources: Default::default(),
            secrets: Secrets::default(),
            cache: None,
            mode: Mode::default(),
            replay_cache: None,
            proxy: None,
            network: Network::default(),
            max_body: 8 * 1024 * 1024,
            source_deadlines: BTreeMap::new(),
            sync_budget: Duration::from_secs(120),
            auto_sync_max_bytes: 32 * 1024 * 1024,
        }
    }
}

/// One blocking client shared across source legs.
#[expect(
    clippy::disallowed_types,
    reason = "Reserve each bundle replay import once per Fetch"
)]
#[derive(Clone)]
pub struct Fetch {
    agents: std::sync::Arc<network::Agents>,
    limiters: std::sync::Arc<quota::Limiters>,
    cache: Option<std::sync::Arc<cache::Cache>>,
    recordings: std::sync::Arc<record::Recordings>,
    clock: Option<std::sync::Arc<deadline::VirtualClock>>,
    config: std::sync::Arc<FetchConfig>,
    bundle_imports: std::sync::Arc<std::sync::Mutex<std::collections::BTreeSet<&'static str>>>,
    disabled_sources: std::collections::BTreeSet<biodata::Source>,
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
    pub fn open(mut config: FetchConfig) -> Result<Self, FetchError> {
        if config.max_body == u64::MAX {
            return Err(FetchError::BodyLimit);
        }
        let agents = network::Agents::new(&config.network, config.proxy.as_ref())?;
        if matches!(config.mode, Mode::Replay { .. }) && config.replay_cache.is_none() {
            config.replay_cache = Some(
                ReplayCache::temporary_with_owners(&config.cache_owners)
                    .map_err(FetchError::Cache)?,
            );
        }
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
                .map(|path| cache::Cache::open_with_owners(path, &config.cache_owners))
                .transpose()
                .map_err(FetchError::Cache)?
                .map(std::sync::Arc::new)
        };
        let clock =
            matches!(config.mode, Mode::Replay { .. }).then(deadline::VirtualClock::default);
        Ok(Self {
            agents: std::sync::Arc::new(agents),
            cache,
            recordings: std::sync::Arc::new(recordings),
            clock: clock.map(std::sync::Arc::new),
            disabled_sources: config.disabled_sources.clone(),
            config: std::sync::Arc::new(config),
            limiters: std::sync::Arc::default(),
            bundle_imports: std::sync::Arc::default(),
        })
    }

    /// Whether the explicit source policy prevents every read and request.
    pub fn disabled(&self, source: biodata::Source) -> bool {
        self.disabled_sources.contains(&source)
    }

    /// Restrict a command while retaining shared quotas, clocks, recordings and connections.
    /// Existing source restrictions cannot be removed.
    pub fn restrict_sources(&self, disabled: std::collections::BTreeSet<biodata::Source>) -> Self {
        let mut restricted = self.clone();
        restricted.disabled_sources.extend(disabled);
        restricted
    }

    /// Report whether a configured credential is nonempty without exposing it.
    pub fn has_key(&self, name: SecretName) -> bool {
        self.config.secrets.get(name).is_some()
    }

    /// Create a real deadline or an initial replay budget for independent legs.
    pub fn deadline(&self, budget: Duration) -> Deadline<'_> {
        Deadline::start(budget, self.clock.as_deref())
    }

    /// Own a sequential command's replay origin without changing independent leg consumers.
    pub fn command_deadline(&self, budget: Duration) -> CommandDeadline<'_> {
        CommandDeadline::start(budget, self.clock.as_deref())
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
        let no_key = !self.disabled(spec.source)
            && match spec.key {
                KeyUse::Required(credential) => {
                    self.config.secrets.get(credential.secret).is_none()
                }
                _ => false,
            };
        Leg {
            fetch: self,
            spec,
            deadline: deadline.child(cap),
            clock: deadline.fork(),
            wrong_clock: !deadline.matches(self.clock.as_deref()),
            no_key,
            report: std::sync::Arc::new(std::sync::Mutex::new(LegReport::default())),
            nested_batch: false,
            refreshed_bundles: Default::default(),
            fresh: matches!(self.config.mode, Mode::Live { fresh: true }),
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
    report: std::sync::Arc<std::sync::Mutex<LegReport>>,
    nested_batch: bool,
    fresh: bool,
    refreshed_bundles: std::sync::Arc<std::sync::Mutex<std::collections::BTreeSet<&'static str>>>,
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
    /// Successful replies with their canonical identity and original fetch time.
    pub replies: Vec<ReplyMetadata>,
    /// Whether any request served a checked stale reply.
    pub stale: bool,
    /// Original typed provider failures that caused stale replies to be served.
    pub failures: Vec<SourceError>,
}
impl Leg<'_> {
    /// Bypass cached reads and stale fallback for this leg and its batch children.
    /// The process policy cannot be weakened by a call.
    pub fn fresh(mut self, fresh: bool) -> Self {
        self.fresh |= fresh;
        self
    }
    pub(crate) fn budget(&self) -> Deadline<'_> {
        self.deadline.bind(self.clock.as_ref())
    }
    /// Bind the source leg's remaining budget for cooperative local parsing.
    pub fn local_deadline(&self) -> LocalDeadline {
        self.budget().local()
    }
    /// Return this source leg's remaining real or virtual budget.
    pub fn remaining(&self) -> Duration {
        self.budget().remaining()
    }
    /// Finish the leg without clearing failures after later successful requests.
    pub fn finish(self) -> LegReport {
        std::mem::take(
            &mut *self
                .report
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        )
    }
}

/// Provenance of one checked response, independent of other replies in its leg.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplyMetadata {
    /// Credential-free request identity.
    pub identity: [u8; 32],
    /// Original fetch time in Unix seconds; zero for legacy recordings without a timestamp.
    pub fetched: i64,
}
