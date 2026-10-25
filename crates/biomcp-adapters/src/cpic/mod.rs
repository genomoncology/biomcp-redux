//! Canonical pharmacogenomics publisher declaration; runtime dispatch arrives separately.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Cpic,
    base: "https://api.cpicpgx.org/v1",
    terms: Terms {
        name: crate::pgx_sources::TERMS.name,
        url: crate::pgx_sources::TERMS.url,
        usage: crate::pgx_sources::TERMS.usage,
    },
    quota: &crate::quotas::CPIC,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
