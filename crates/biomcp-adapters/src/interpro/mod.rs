//! Integrated InterPro entries only; member-database material retains upstream rights.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration owns admission, terms and keyless request policy.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::InterPro,
    base: "https://www.ebi.ac.uk/interpro/api",
    terms: Terms {
        name: "CC0",
        url: "https://interpro-documentation.readthedocs.io/en/latest/license.html",
        usage: Use::Open,
    },
    key: KeyUse::None,
    quota: &crate::quotas::INTERPRO,
    deadline: Duration::from_secs(8),
};
