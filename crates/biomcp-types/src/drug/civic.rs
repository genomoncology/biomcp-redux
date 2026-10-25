use super::{DrugGet, DrugHeader};
use crate::{CivicClaimPage, CivicMember, CivicStatement, SourceRecord};
use biodata::Text;
use serde::{Deserialize, Serialize};

/// Independent accepted therapy claim collections, without profile count inference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCivicStreams {
    /// Evidence items filtered by the selected native therapy and optional profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<DrugCivicPage>,
    /// Assertions filtered independently by the selected native therapy and optional profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions: Option<DrugCivicPage>,
}
/// One native claim stream and its own continuation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCivicPage {
    /// Native claim order; each row carries the profile to which that claim belongs.
    pub records: Box<[DrugCivicClaim]>,
    /// Stream totals and continuation, independent of every other stream.
    pub page: CivicClaimPage<DrugGet>,
}
/// One native claim with its complete source therapy set and molecular profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCivicClaim {
    /// Source molecular profile context; its member expression is never collapsed.
    pub profile: DrugCivicProfile,
    /// Existing source claim mapping, including unmapped native facts.
    pub claim: CivicStatement,
    /// Complete native therapy identities, without cross-source equivalence.
    pub therapies: Box<[DrugHeader]>,
    /// Native combination interaction label when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub therapy_interaction_type: Option<Text>,
}
/// Profile context embedded in a therapy claim, with no invented profile totals.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCivicProfile {
    /// Native profile identity and link.
    pub record: SourceRecord,
    /// Complete source expression.
    pub label: Text,
    /// Source complex-profile flag.
    pub compound: bool,
    /// Every source member, including members without a common-model subject.
    pub members: Box<[CivicMember]>,
}
