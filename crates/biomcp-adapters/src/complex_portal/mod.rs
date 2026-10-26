//! Exact native Complex Portal participants; no inferred membership.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration owns admission, terms and keyless request policy.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::ComplexPortal,
    base: "https://www.ebi.ac.uk/intact/complex-ws",
    terms: Terms {
        name: "CC0",
        url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11701666/",
        usage: Use::Open,
    },
    key: KeyUse::None,
    quota: &crate::quotas::COMPLEX_PORTAL,
    deadline: Duration::from_secs(8),
};
