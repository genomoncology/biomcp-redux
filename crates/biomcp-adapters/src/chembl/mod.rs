//! Canonical CHEMBL source declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy shared by health, request admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Chembl,
    base: "https://www.ebi.ac.uk/chembl/api/data",
    terms: Terms {
        name: crate::drug_sources::CHEMBL_TERMS.name,
        url: crate::drug_sources::CHEMBL_TERMS.url,
        usage: crate::drug_sources::CHEMBL_TERMS.usage,
    },
    quota: &crate::quotas::CHEMBL,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
