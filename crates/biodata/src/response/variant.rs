use crate::{Source, Text, VariantGet, VariantHeader, VariantResolution};
use serde::{Deserialize, Serialize};
/// A compact identity card, including every provider candidate and explicit rejection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
