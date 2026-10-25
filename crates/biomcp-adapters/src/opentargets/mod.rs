//! Canonical OPENTARGETS source declaration.
use biomcp_fetch::{KeyUse, Quota, Rate, SourceKey, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
// Conservative local policy, not a claim about the publisher's quota.
static QUOTA: Quota = Quota::new(
    SourceKey::declare("opentargets"),
    Rate::per_second(1),
    None,
    1,
);
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Opentargets,
    base: "https://api.platform.opentargets.org/api/v4",
    terms: Terms {
        name: crate::drug_sources::OPENTARGETS_TERMS.name,
        url: crate::drug_sources::OPENTARGETS_TERMS.url,
        usage: crate::drug_sources::OPENTARGETS_TERMS.usage,
    },
    quota: &QUOTA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
