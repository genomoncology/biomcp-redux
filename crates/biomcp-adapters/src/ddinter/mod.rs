//! Canonical DDINTER source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Ddinter,
    base: "https://ddinter.scbdd.com",
    terms: Terms {
        name: crate::drug_sources::DDINTER_TERMS.name,
        url: crate::drug_sources::DDINTER_TERMS.url,
        usage: crate::drug_sources::DDINTER_TERMS.usage,
    },
    quota: &crate::quotas::DDINTER,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
