//! Canonical seer publisher declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// Publisher policy shared by health, admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Seer,
    base: "https://seer.cancer.gov/statistics-network/explorer/source/content_writers",
    terms: Terms { name: "SEER/NCI Using This Website", url: "https://seer.cancer.gov/about/using-website.html", usage: Use::Open },
    quota: &crate::quotas::SEER,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
