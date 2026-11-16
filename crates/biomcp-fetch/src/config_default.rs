//! Built-in transport configuration values.
use crate::{FetchConfig, Mode, Network, RateShare, Secrets, SourcePacing, grpc};
use std::{collections::BTreeMap, time::Duration};

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            rate_share: RateShare::default(),
            source_pacing: SourcePacing::default(),
            grpc_limits: grpc::Limits::default(),
            observations: None,
            cache_owners: Vec::new(),
            disabled_sources: Default::default(),
            disabled_uses: Default::default(),
            secrets: Secrets::default(),
            cache: None,
            cache_predecessors: Vec::new(),
            mode: Mode::default(),
            replay_cache: None,
            proxy: None,
            network: Network::default(),
            max_body: 8 * 1024 * 1024,
            source_deadlines: BTreeMap::new(),
            sync_budget: Duration::from_secs(120),
            auto_sync_max_bytes: 32 * 1024 * 1024,
            reply_cache_max_bytes: crate::cache::REPLY_CACHE_MAX_BYTES,
            source_data_max_age_days: 365,
        }
    }
}
