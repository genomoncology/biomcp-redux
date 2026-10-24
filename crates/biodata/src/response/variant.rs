use crate::{Source, SourceRecord, Text, Variant, VariantGet};
use serde::{Deserialize, Serialize};
/// A compact identity card, including every provider candidate and explicit rejection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantCard {
    /// Identity outcome, without selecting the first candidate.
    pub resolution: VariantResolution,
    /// Responsible identity source.
    pub source: Source,
    /// All retained source identities.
    pub candidates: Box<[VariantHeader]>,
    /// Provider rejection text, when present, separate from unchanged query input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection: Option<Text>,
}
impl super::ResponseBody for VariantCard {
    const SCHEMA: &'static str = "biomcp.variant_card/1";
    type Query = VariantGet;
}

/// Consumer identity header with an exact source selection and compact annotation entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantHeader {
    /// Source-qualified variant entity.
    pub identity: Variant,
    /// Source responsible for this identity.
    pub source: Source,
    /// Exact source record, including source keys that are expressions rather than identifiers.
    pub record: SourceRecord,
    /// Compact MyVariant annotations, attributed by their field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myvariant: Option<Variant>,
    /// Explicitly requested source sections for this exact candidate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<Box<VariantSections>>,
}
crate::macros::wire_enum! {
    /// Identity resolution preserves candidates and source validation outcomes.
    VariantResolution {
        /// One checked provider identity.
        Resolved => "resolved",
        /// Several provider identities; none was selected.
        Candidates => "candidates",
        /// Provider reported no match.
        Absent => "absent",
        /// Provider rejected the unchanged input.
        Rejected => "rejected",
        /// Provider could not answer.
        Unavailable => "unavailable",
    }
}

/// Source-specific sections, absent unless explicitly selected.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantSections {
    /// Explicit keyed OncoKB source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oncokb: Option<crate::Section<crate::VariantOncokb>>,
    /// Explicit clinvar source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clinvar: Option<crate::Section<crate::VariantAssertions>>,
    /// Explicit erepo source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub erepo: Option<crate::Section<crate::VariantAssertions>>,
    /// Explicit population source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub population: Option<crate::Section<crate::VariantPopulation>>,
    /// Explicit gwas source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gwas: Option<crate::Section<crate::VariantGwas>>,
    /// Explicit civic source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub civic: Option<crate::Section<crate::VariantCivic>>,
    /// Explicit hotspots source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hotspots: Option<crate::Section<crate::VariantHotspots>>,
    /// Explicit predict source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predict: Option<crate::Section<crate::VariantPredictors>>,
    /// Explicit conservation source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conservation: Option<crate::Section<crate::VariantConservation>>,
    /// Explicit cosmic source result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cosmic: Option<crate::Section<crate::VariantCosmic>>,
}
