//! Index native disease identifiers and names in the validity exports.
use crate::Mapped;
use biomcp_fetch::{BundleInfo, Find, KeyKind, Keys};
use biomcp_types::{DiseaseCuration, DiseaseId};

/// One export's checked matches and the snapshot that supplied them.
#[derive(Debug)]
pub struct CurationMatches {
    /// Matching source assertions and any row refusals.
    pub rows: Mapped<Vec<DiseaseCuration>>,
    /// Metadata of the local source copy used for the lookup.
    pub snapshot: BundleInfo,
}

pub(crate) fn index(keys: &mut Keys, id: Option<&str>, name: Option<&str>) {
    // Keep the existing bundle admission rule for rows without a usable gene key.
    if keys.exact.is_empty() {
        return;
    }
    keys.exact.extend(
        [("id", id), ("name", name)]
            .into_iter()
            .filter_map(|(kind, value)| {
                value.map(|value| (KeyKind::Xref, format!("{kind}:{value}").into()))
            }),
    );
}

pub(crate) struct Lookup<'a>(&'a DiseaseId);

impl<'a> Lookup<'a> {
    pub(crate) fn new(id: &'a DiseaseId) -> Self {
        Self(id)
    }

    pub(crate) fn key(&self) -> String {
        let kind = if matches!(self.0, DiseaseId::Concept(_)) {
            "id"
        } else {
            "name"
        };
        format!("{kind}:{}", self.0)
    }

    pub(crate) fn find(key: &str) -> Find<'_> {
        Find::ExactOrUnindexed(KeyKind::Xref, key)
    }

    pub(crate) fn matches(&self, id: Option<&str>, name: Option<&str>) -> bool {
        let candidate = if matches!(self.0, DiseaseId::Concept(_)) {
            id
        } else {
            name
        };
        candidate.is_some_and(|value| value.eq_ignore_ascii_case(self.0.as_str()))
    }
}
