//! Canonical EMA source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Ema,
    base: "https://www.ema.europa.eu",
    terms: Terms {
        name: crate::drug_sources::EMA_TERMS.name,
        url: crate::drug_sources::EMA_TERMS.url,
        usage: crate::drug_sources::EMA_TERMS.usage,
    },
    quota: &crate::quotas::EMA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
