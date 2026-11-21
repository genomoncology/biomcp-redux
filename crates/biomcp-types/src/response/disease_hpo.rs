//! Disease matches and gene links as supplied by one named HPO release.
use crate::{DiseaseSearch, Text};
use biodata::{Curie, HpId};
use serde::{Deserialize, Serialize};

/// One requested term and the release's annotations that satisfy it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HpoTermMatch {
    /// Caller-supplied HPO ID.
    pub requested: HpId,
    /// Positive annotation IDs, including exact terms and descendants.
    pub annotated: Box<[HpId]>,
}

/// A gene link supplied for the original disease ID.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HpoGeneLink {
    /// Original NCBI Gene identifier.
    pub id: Curie,
    /// Gene symbol from the release.
    pub symbol: Text,
    /// Publisher association type.
    pub association_type: Text,
    /// The source named by the release.
    pub source: Text,
}

/// One disease with positive support for all requested terms.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct HpoDiseaseMatch {
    /// Disease identifier exactly as supplied, including its namespace.
    pub id: Curie,
    /// Disease name from the annotation file.
    pub label: Text,
    /// Positive evidence for each requested term.
    pub matches: Box<[HpoTermMatch]>,
    /// Gene links supplied for this original ID.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub gene_links: Box<[HpoGeneLink]>,
    /// The release supplies no gene links for this ID.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub gene_links_missing: bool,
}

page!(@body, HpoDiseasePage, HpoDiseaseMatch, DiseaseSearch, "biomcp.disease-hpo-page/1",
    "One page of disease matches from a named HPO release.", [Eq],
    "Distinct original disease IDs in identifier order.", [
        /// Publisher release tag shared by all three files.
        pub release: Box<str>,
    ], []);
