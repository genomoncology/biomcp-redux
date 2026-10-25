//! Canonical diagnostic publisher declaration; runtime dispatch belongs to later slices.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::FdaCdx,
    base: "https://www.fda.gov",
    terms: Terms {
        name: "FDA website policies; third-party exceptions retained",
        url: "https://www.fda.gov/about-fda/about-website/website-policies",
        usage: Use::Open,
    },
    quota: &crate::quotas::FDA_WEBSITE,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
