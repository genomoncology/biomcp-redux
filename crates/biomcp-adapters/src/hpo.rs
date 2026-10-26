//! Direct exact-term HPO service policy; no discovery or bundle endpoint.
use crate::disease_sources::components::HPO;
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// The same publisher declaration governs direct terms and aggregator components.
/// Reply retrieval time never establishes an ontology version or release date.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::Hpo,
    base: "https://ontology.jax.org",
    terms: Terms {
        name: HPO.name,
        url: HPO.url,
        usage: HPO.usage,
    },
    key: KeyUse::None,
    quota: &crate::quotas::HPO,
    deadline: Duration::from_secs(15),
};
