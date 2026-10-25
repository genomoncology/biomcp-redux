use biodata::{Attribution, Concept, Curie, DiseaseAssociation, DiseaseRelation, DiseaseStatistic, DiseaseSynonym, Text};
use crate::{ComponentTerms, DiseaseGet, DiseaseOntology, DiseaseSearch, DiseaseSection, ResponseBody};
use serde::{Deserialize, Serialize};
/// One source candidate without a selected identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseHeader {
    /// Native concept and optional exact ID.
    pub concept: Concept,
    /// The explicit queried ontology.
    pub ontology: DiseaseOntology,
    /// Source record and version.
    pub attribution: Attribution,
}
/// Exact identity and available pages without hidden enrichment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseCard {
    /// Exact requested identifier.
    pub requested_id: Curie,
    /// Actual returned identifier.
    pub returned_id: Curie,
    /// Native identity and complete primary label.
    pub concept: Concept,
    /// Complete source definition when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Text>,
    /// Publisher obsolete flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obsolete: Option<bool>,
    /// Publisher alternatives, never an automatic replacement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consider: Option<Box<[Curie]>>,
    /// Publisher explicit replacement identifiers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<Box<[Curie]>>,
    /// Publisher record and version.
    pub attribution: Attribution,
    /// Available named sections.
    pub sections: Box<[DiseaseSection]>,
    /// Applicable component terms.
    pub components: Box<[ComponentTerms]>,
}
impl ResponseBody for DiseaseCard { const SCHEMA: &'static str = "biomcp.disease_card/1"; type Query = DiseaseGet; }
/// One publisher catalog site, independent of ontology identities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseSite {
    /// Native publisher site code.
    pub code: Text,
    /// Complete publisher site label.
    pub label: Text,
    /// Publisher active-state flag.
    pub active: bool,
}
macro_rules! page {
    ($name:ident, $row:ty, $query:ty, $schema:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        pub struct $name {
            /// Native source rows; totals in the envelope count rows, not distinct entities.
            pub items: Box<[$row]>,
            /// Applicable component use terms, independent of row provenance.
            pub components: Box<[ComponentTerms]>,
        }
        impl ResponseBody for $name {
            const SCHEMA: &'static str = $schema;
            type Query = $query;
        }
    };
}
page!(DiseasePage, DiseaseHeader, DiseaseSearch, "biomcp.disease_page/1", "One source-ordered disease candidate page.");
page!(DiseaseSynonyms, DiseaseSynonym, DiseaseGet, "biomcp.disease_synonyms/1", "One publisher synonym page with scope and language.");
page!(DiseaseXrefs, DiseaseRelation, DiseaseGet, "biomcp.disease_xrefs/1", "Native cross-reference predicates without equivalence promotion.");
page!(DiseaseHierarchy, DiseaseRelation, DiseaseGet, "biomcp.disease_hierarchy/1", "Native parent relations with both endpoints.");
page!(DiseaseGenes, DiseaseAssociation, DiseaseGet, "biomcp.disease_genes/1", "Native association rows, not a unique-gene count.");
page!(DiseasePhenotypes, DiseaseAssociation, DiseaseGet, "biomcp.disease_phenotypes/1", "Native phenotype association rows without diagnostic inference.");
page!(DiseaseSites, DiseaseSite, DiseaseSearch, "biomcp.disease_sites/1", "Publisher site catalog without an ontology-to-site mapping.");
page!(DiseaseIncidence, DiseaseStatistic, DiseaseGet, "biomcp.disease_incidence/1", "Native incidence rows preserving requested and returned dimensions.");
page!(DiseaseSurvival, DiseaseStatistic, DiseaseGet, "biomcp.disease_survival/1", "Native annual survival rows with separate observed and modeled values.");
