//! Canonical diagnostic publisher declaration; runtime dispatch belongs to later slices.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::WhoIvd,
    base: "https://extranet.who.int/prequal",
    terms: Terms {
        name: "WHO terms of use; export has no narrower license; commercial and substantial reproduction reserved",
        url: "https://www.who.int/about/policies/terms-of-use",
        usage: Use::Noncommercial,
    },
    quota: &crate::quotas::WHO,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
