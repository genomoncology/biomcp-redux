use super::DrugId;
use crate::{Attribution, PartialDate, Text};
use serde::{Deserialize, Serialize};
/// A native date kept with its original role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorizationDate {
    /// Native date role, without assigning approval meaning.
    pub role: Text,
    /// Source date at its stated precision.
    pub date: PartialDate,
}
/// A source-scoped regulator event; never an identity-card field.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorizationEvent {
    /// Native regulator record identifier.
    pub record: DrugId,
    /// Explicit native product number, never a synthesized first product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_number: Option<Text>,
    /// Jurisdiction explicitly stated by the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Text>,
    /// Original source status; historical positives imply no current status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Text>,
    /// Original native event type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Text>,
    /// All native dates with separate roles.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub dates: Box<[AuthorizationDate]>,
    /// Original indication scope when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indication: Option<Text>,
    /// One source record and its native provenance.
    pub attribution: Attribution,
}
