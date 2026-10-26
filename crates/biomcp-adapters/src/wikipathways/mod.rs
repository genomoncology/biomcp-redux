//! Publisher policy for explicit WikiPathways catalog synchronization.
mod raw;
mod catalog;
pub use catalog::{card,search,sync};
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;

/// One declaration owns publisher terms, admission and bundle download policy.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::WikiPathways,
    base: "https://www.wikipathways.org",
    terms: Terms {
        name: "CC0 1.0 waiver",
        url: "https://www.wikipathways.org/help.html",
        usage: Use::Open,
    },
    key: KeyUse::None,
    quota: &crate::quotas::WIKIPATHWAYS,
    deadline: Duration::from_secs(120),
};
