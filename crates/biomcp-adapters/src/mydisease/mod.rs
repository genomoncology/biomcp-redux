//! Canonical mydisease publisher declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// Publisher policy shared by health, admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::MyDisease,
    base: "https://mydisease.info/v1",
    terms: Terms {
        name: "MyDisease source-specific terms",
        url: "https://docs.mydisease.info/en/latest/doc/data.html",
        usage: Use::Attribution,
    },
    quota: &crate::quotas::BIOTHINGS,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
