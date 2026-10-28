use crate::{Allele, Build, Expression, GeneRef, Text, VariantId};
use serde::{Deserialize, Serialize};
/// One source's own expression, assembly and optional source-stated interbase allele.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Representation {
    /// Source asserting this form.
    pub source: Text,
    /// Published notation kept unchanged.
    pub expression: Expression,
    /// Assembly stated by the source, omitted when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    /// Source-supplied allele, without local normalization or equivalence inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allele: Option<Allele>,
}
/// Variant entity with source-stated identities, forms and genes; no disclosure policy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Variant {
    /// All source identifiers, without inferred aliases.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub ids: Box<[VariantId]>,
    /// Every source-qualified representation.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub representations: Box<[Representation]>,
    /// Genes stated by the sources.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub genes: Box<[GeneRef]>,
}
