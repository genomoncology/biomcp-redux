//! Index native disease identifiers and names in the validity exports.
use crate::Mapped;
use biodata::Text;
use biomcp_fetch::{BundleInfo, Find, KeyKind, Keys};
use biomcp_types::{DiseaseCuration, DiseaseId};
use caseless::Caseless;

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
    if let Some(name) = name {
        keys.exact.push((KeyKind::CurationTitle, fold(name).into()));
    }
}

/// Select curations by an exact native identity or a literal publisher title prefix.
#[derive(Clone, Copy, Debug)]
pub enum CurationLookup<'a> {
    /// Exact native identifier or title.
    Exact(&'a DiseaseId),
    /// Prefix of a publisher's own disease title.
    TitlePrefix(&'a Text),
}

impl CurationLookup<'_> {
    pub(crate) fn key(&self) -> String {
        match self {
            Self::Exact(id @ DiseaseId::Concept(_)) => format!("id:{id}"),
            Self::Exact(name) => format!("name:{name}"),
            Self::TitlePrefix(prefix) => fold(prefix.as_str()),
        }
    }

    pub(crate) fn find<'a>(&self, key: &'a str) -> Find<'a> {
        match self {
            Self::Exact(_) => Find::ExactOrUnindexed(KeyKind::Xref, key),
            Self::TitlePrefix(_) => Find::PrefixOrUnindexed(KeyKind::CurationTitle, key),
        }
    }

    pub(crate) fn matches(&self, id: Option<&str>, name: Option<&str>, key: &str) -> bool {
        match self {
            Self::Exact(requested @ DiseaseId::Concept(_)) => same(id, requested),
            Self::Exact(requested) => same(name, requested),
            Self::TitlePrefix(_) => name.is_some_and(|name| fold(name).starts_with(key)),
        }
    }
}

fn same(candidate: Option<&str>, requested: &DiseaseId) -> bool {
    candidate.is_some_and(|value| value.eq_ignore_ascii_case(requested.as_str()))
}

fn fold(title: &str) -> String {
    title.chars().default_case_fold().collect()
}
