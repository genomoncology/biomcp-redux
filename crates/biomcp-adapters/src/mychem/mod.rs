//! Canonical MYCHEM source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Mychem,
    base: "https://mychem.info/v1",
    terms: Terms {
        name: crate::drug_sources::MYCHEM_TERMS.name,
        url: crate::drug_sources::MYCHEM_TERMS.url,
        usage: crate::drug_sources::MYCHEM_TERMS.usage,
    },
    quota: &crate::quotas::BIOTHINGS,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
