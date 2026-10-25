//! Canonical OPENTARGETS source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Opentargets,
    base: "https://api.platform.opentargets.org/api/v4",
    terms: Terms {
        name: crate::drug_sources::OPENTARGETS_TERMS.name,
        url: crate::drug_sources::OPENTARGETS_TERMS.url,
        usage: crate::drug_sources::OPENTARGETS_TERMS.usage,
    },
    quota: &crate::quotas::OPENTARGETS,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
