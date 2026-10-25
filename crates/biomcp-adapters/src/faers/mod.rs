//! Canonical openFDA publisher declaration; report dispatch belongs to its adapter slice.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, outcomes and request admission.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Faers,
    base: "https://api.fda.gov/drug/event.json",
    terms: Terms {
        name: crate::adverse_event_sources::FDA_TERMS.name,
        url: crate::adverse_event_sources::FDA_TERMS.url,
        usage: crate::adverse_event_sources::FDA_TERMS.usage,
    },
    quota: &crate::quotas::OPENFDA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
mod raw;
mod search;
pub use search::{Page, expression, search};
