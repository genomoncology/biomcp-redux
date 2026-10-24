use crate::{Build, GenomicInterval, Source, Text};
use serde::{Deserialize, Serialize};
/// A provider's unchanged notation and optional source coordinate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantRepresentation {
    /// Provider responsible for this representation.
    pub source: Source,
    /// Unchanged source notation.
    pub notation: Text,
    /// Reference accession or provider reference identifier.
    pub reference: Text,
    /// Provider's original assembly label, including older or unstated builds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly: Option<Text>,
    /// Explicitly counted coordinate, only when supplied by the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<GenomicInterval>,
    /// Source reference allele without complementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_allele: Option<String>,
    /// Source alternate allele without repair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_allele: Option<String>,
}
/// One source record with its exact provider aliases and allele representations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantIdentity {
    /// Identity provider.
    pub source: Source,
    /// Exact provider record key.
    pub key: Text,
    /// Identifiers asserted by the same provider, without local alias inference.
    pub aliases: Box<[Text]>,
    /// Every source-qualified representation retained for this record.
    pub representations: Box<[VariantRepresentation]>,
}
/// A compact annotation fact retains its original source and label.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantFact {
    /// Annotation supplier inside MyVariant.
    pub source: Text,
    /// Source field name, without a common severity ranking.
    pub field: Text,
    /// Provider text, preserved without interpretation.
    pub value: Text,
}
/// Enough source identity to select a candidate and inspect compact facts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantHeader {
    /// Checked source identity.
    pub identity: VariantIdentity,
    /// Compact source-attributed annotation facts.
    pub facts: Box<[VariantFact]>,
}
crate::macros::wire_enum! {
    /// Identity resolution preserves ambiguity, rejection and unavailable sources.
    VariantResolution {
        /// One exact source identity.
        Resolved => "resolved",
        /// Several provider candidates; none was chosen.
        Candidates => "candidates",
        /// Provider reported no matching identity.
        Absent => "absent",
        /// Provider rejected the unchanged input.
        Rejected => "rejected",
        /// Provider could not answer.
        Unavailable => "unavailable",
    }
}
crate::macros::wire_enum! {
    /// Explicit resolver service; the default never selects an alternative silently.
    VariantResolver {
        /// ClinGen Allele Registry.
        Car => "car",
        /// Mutalyzer normalizer.
        Mutalyzer => "mutalyzer",
        /// VariantValidator validator.
        Variantvalidator => "variantvalidator",
    }
}
impl VariantResolver {
    /// Declared source policy for this service.
    pub fn source(self) -> Source {
        match self {
            Self::Car => Source::AlleleRegistry,
            Self::Mutalyzer => Source::Mutalyzer,
            Self::Variantvalidator => Source::VariantValidator,
        }
    }
}
/// Only explicitly supported coordinate assemblies can be selected by the caller.
pub fn variant_build(build: Build) -> bool {
    matches!(build, Build::Grch37 | Build::Grch38)
}
