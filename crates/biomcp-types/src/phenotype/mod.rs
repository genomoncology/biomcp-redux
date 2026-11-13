mod query;
use crate::{ComponentLabel, DiseaseAssociationItem, ResponseBody};
use biodata::{
    PhenotypeCard, PhenotypeHeader, PhenotypeHierarchyLink, PhenotypeSynonym, PhenotypeXref,
};
pub use query::{
    PHENOTYPE_MATCH_MAX, PhenotypeGet, PhenotypeGroup, PhenotypeMetric, PhenotypeSearch,
    PhenotypeSection, PhenotypeSource,
};
use serde::{Deserialize, Serialize};
/// Independent source identity card and its available sections.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PhenotypeTermCard {
    /// Complete source-specific native card.
    pub term: PhenotypeCard,
    /// Sections available on this source record.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub sections: Box<[PhenotypeSection]>,
    /// Applicable publisher component terms.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub components: Box<[ComponentLabel]>,
}
impl ResponseBody for PhenotypeTermCard {
    const SCHEMA: &'static str = "biomcp.phenotype-card/5";
    type Query = PhenotypeGet;
}

page!(
    phenotype,
    PhenotypePage,
    PhenotypeHeader,
    PhenotypeSearch,
    "biomcp.phenotype-search/5",
    "Native HP search header page, without local ranking."
);
page!(
    phenotype,
    PhenotypeSynonyms,
    PhenotypeSynonym,
    PhenotypeGet,
    "biomcp.phenotype-synonyms/4",
    "Source synonym rows preserving publisher classes."
);
page!(
    phenotype,
    PhenotypeXrefs,
    PhenotypeXref,
    PhenotypeGet,
    "biomcp.phenotype-xrefs/4",
    "Source reference rows without equivalence promotion."
);
page!(
    phenotype,
    PhenotypeHierarchy,
    PhenotypeHierarchyLink,
    PhenotypeGet,
    "biomcp.phenotype-hierarchy/5",
    "Direct source hierarchy links without graph traversal."
);
/// Monarch's ranked genes or diseases for a set of HP terms, in Monarch's order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PhenotypeMatches {
    /// Ranked rows in source order; Monarch scores them, BioMCP does not rerank.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[PhenotypeMatch]>,
    /// The queried HP terms with the labels Monarch matched, in caller order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub terms: Box<[PhenotypeMatchTerm]>,
    /// Monarch's own statement of how it scored the rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<PhenotypeMatchMethod>,
    /// Monarch returns at most this many ranked rows and no further pages; an entity outside them is unranked here, not absent from Monarch.
    pub source_limit: u32,
    /// Applicable publisher component terms.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub components: Box<[ComponentLabel]>,
    /// Original source retrieval time, distinct from dataset release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched: Option<u64>,
}
impl ResponseBody for PhenotypeMatches {
    const SCHEMA: &'static str = "biomcp.phenotype-matches/1";
    type Query = PhenotypeSearch;
}
/// One ranked Monarch entity and why it matched.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PhenotypeMatch {
    /// One-based position in Monarch's order.
    pub rank: u32,
    /// Monarch entity identity, such as an HGNC gene or a MONDO disease.
    pub id: biodata::Curie,
    /// Monarch entity name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,
    /// Monarch entity category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<biodata::Curie>,
    /// Monarch's ranking score under the stated metric and directionality.
    pub score: biodata::FiniteNumber,
    /// Monarch's average of the entity's phenotypes' best matches to the set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_to_object_score: Option<biodata::FiniteNumber>,
    /// Monarch's average of the queried terms' best matches to the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_to_subject_score: Option<biodata::FiniteNumber>,
    /// Each queried term's best-matching entity phenotype, as Monarch reports it.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub best_matches: Box<[PhenotypeBestMatch]>,
}
/// One queried term's best match within a ranked entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PhenotypeBestMatch {
    /// Queried HP term.
    pub term: biodata::Curie,
    /// The entity's phenotype that matched it best.
    pub matched: biodata::Curie,
    /// Monarch label of the matched phenotype.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_label: Option<Box<str>>,
    /// Most informative common ancestor of the two terms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ancestor: Option<biodata::Curie>,
    /// Monarch label of that ancestor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ancestor_label: Option<Box<str>>,
    /// Monarch's score for this pair under the stated metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<biodata::FiniteNumber>,
}
/// One queried HP term and the label Monarch matched for it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PhenotypeMatchTerm {
    /// Queried HP term.
    pub id: biodata::HpId,
    /// Monarch label; absent when Monarch has no such term.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,
}
/// Monarch's stated search settings, unchanged.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PhenotypeMatchMethod {
    /// Similarity metric Monarch used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<Box<str>>,
    /// Which best matches Monarch averaged into the score.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directionality: Option<Box<str>>,
    /// Monarch candidate selection; hybrid scores only a Jaccard-prefiltered set, so a richly annotated entity can be left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<Box<str>>,
    /// Where Monarch's information content comes from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub information_content: Option<Box<str>>,
    /// Monarch engine that served the search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<Box<str>>,
}
macro_rules! associations {
    ($name:ident, $schema:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        pub struct $name {
            /// Native association rows in source order, without deduplication.
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            pub items: Box<[DiseaseAssociationItem]>,
            /// Publisher direct graph scope; this supplies no causal assertion.
            pub direct: bool,
            /// Applicable publisher component terms.
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            pub components: Box<[ComponentLabel]>,
            /// Original source retrieval time, distinct from dataset release.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub fetched: Option<u64>,
        }
        impl ResponseBody for $name {
            const SCHEMA: &'static str = $schema;
            type Query = PhenotypeGet;
        }
    };
}
associations!(
    PhenotypeDiseases,
    "biomcp.phenotype-diseases/7",
    "Direct source disease association rows without clinical inference."
);
associations!(
    PhenotypeGenes,
    "biomcp.phenotype-genes/7",
    "Direct source human gene association rows with native taxon."
);
