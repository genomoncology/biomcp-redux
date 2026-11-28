//! Canonical diagnostic publisher declaration; runtime dispatch belongs to later slices.
use biomcp_fetch::SourceSpec;
use biomcp_types::Source;
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::FdaPma,
    base: "https://api.fda.gov/device/pma.json",
    terms: crate::openfda::TERMS,
    quota: &crate::quotas::OPENFDA,
    key: crate::openfda_key::OPTIONAL,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
mod map;
mod raw;
pub use crate::text_parts::escaped_bytes;
mod detail;
pub use detail::{Records, get};
pub use map::{fragments, header, regulatory};

/// Complete source policy and command declaration.
pub(crate) mod decl;

mod search;
pub use search::{expression, search};

/// FDA document downloads retain their API source identity and never send its key.
pub static DOCUMENTS: biomcp_fetch::SourceSpec = biomcp_fetch::SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: biomcp_types::Source::FdaPma,
    base: "https://www.accessdata.fda.gov",
    terms: SPEC.terms,
    key: biomcp_fetch::KeyUse::None,
    quota: &crate::quotas::OPENFDA,
    deadline: std::time::Duration::from_secs(15),
    command_budget: None,
};

pub use crate::device_search::Page as SearchPage;
