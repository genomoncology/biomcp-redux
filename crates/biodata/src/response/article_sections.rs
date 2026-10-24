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
    /// Licensed PMC body outline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulltext: Option<Section<crate::FullText>>,
    /// Normalized PubTator3 entities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Section<Annotations>>,
}
