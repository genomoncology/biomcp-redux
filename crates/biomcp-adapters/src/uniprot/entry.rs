//! Exact UniProt entry identity is checked before every cache admission.
use super::SPEC;
use crate::Mapped;
use biodata::{ProteinAudit, ProteinHeader, UniProtIsoform};
use biomcp_fetch::{Freshness, Leg, RequestError};
use serde::{Deserialize, Serialize};
pub(super) mod map;
pub(super) mod raw;
/// A source-native card without sequence, references or other sections.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    /// Exact returned identity and native metadata.
    pub header: ProteinHeader,
    /// Independent native entry and sequence audit.
    pub audit: ProteinAudit,
}
/// Fetch exactly the requested accession, preserving an explicit isoform suffix.
pub fn entry(leg: &Leg<'_>, id: &UniProtIsoform) -> Result<Mapped<Option<Entry>>, RequestError> {
    let ep = SPEC
        .get("")
        .with_path(format!("/uniprotkb/{id}.json"))?
        .identity_encoding()
        .fresh(Freshness::Days(7));
    match leg.json_checked_cache::<raw::Entry>(&ep, &[], |raw| raw.admitted(id)) {
        Ok(raw) => Ok(map::entry(raw, id)),
        Err(RequestError::Source(e)) if e.reason == biomcp_types::Reason::NotFound => Ok(Mapped {
            value: None,
            problems: Vec::new(),
        }),
        Err(e) => Err(e),
    }
}
