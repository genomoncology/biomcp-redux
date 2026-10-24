use crate::{Capped, EntrezGeneId, Section, Text};
use serde::{Deserialize, Serialize};
crate::source::canonical_enum! {
    /// Normalized annotation category in deterministic tie order.
    AnnotationKind, SourceUnknown {
        /// Gene mention.
        Gene => "gene",
        /// Genetic variant mention.
        Variant => "variant",
        /// Disease mention.
        Disease => "disease",
        /// Chemical mention.
        Chemical => "chemical",
    }
}
/// One normalized entity with its mention count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AnnotatedEntity {
    /// Source concept type.
    pub kind: AnnotationKind,
    /// Source normalized name.
    pub name: Text,
    /// Number of annotation mentions.
    pub count: u64,
    /// Gene identity, including a variant's associated gene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrez_id: Option<EntrezGeneId>,
    /// Source MeSH identity for a disease or chemical.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Text>,
    /// Source HGVS expression for a variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgvs: Option<Text>,
    /// Source reference SNP identity for a variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsid: Option<Text>,
}
/// Capped entity annotations and the distinct total.
pub type Annotations = Capped<AnnotatedEntity>;
/// Landed article sections with independent source states.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleSections {
    /// Citation counts from NIH iCite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<Section<Citations>>,
    /// Normalized PubTator3 entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Section<Annotations>>,
}

/// Independently reported iCite counts; absent lists are never confirmed zeros.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Citations {
    /// Number of citing PubMed papers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cited_by: Option<u64>,
    /// Number of distinct clinical citing papers, when the list was present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cited_by_clinical: Option<u64>,
    /// Number of distinct references, when the list was present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<u64>,
    /// Relative citation ratio, only when confirmed non-provisional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcr: Option<crate::FiniteNumber>,
}
