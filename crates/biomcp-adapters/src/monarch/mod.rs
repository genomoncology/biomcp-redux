//! Canonical monarch publisher declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// Publisher policy shared by health, admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Monarch,
    base: "https://api-v3.monarchinitiative.org",
    terms: Terms {
        name: "Monarch source-specific licensing",
        url: "https://monarch-app.monarchinitiative.org/Licensing/",
        usage: Use::Attribution,
    },
    quota: &crate::quotas::MONARCH,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
