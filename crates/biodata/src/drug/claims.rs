use super::{DrugId, DrugIdentity};
use crate::{Attribution, GeneRef, PartialDate, Text};
use serde::{Deserialize, Serialize};
/// One independently identified label set and revision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelReference {
    /// Published SPL set or native label record ID.
    pub id: DrugId,
    /// Native label revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Source-stated revision date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revised: Option<PartialDate>,
    /// Native label URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Text>,
    /// Label provenance.
    pub attribution: Attribution,
}
/// Original label text under one exact native heading.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelSection {
    /// Label identity and revision.
    pub label: DrugLabelReference,
    /// Exact native field or heading.
    pub heading: Text,
    /// Original text, preserving whitespace; consumer paging never summarizes it.
    pub text: Box<str>,
}
/// A native target or mechanism relation, without an indication inference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugTargetRelation {
    /// Source drug identity.
    pub drug: DrugIdentity,
    /// Source-stated target gene.
    pub target: GeneRef,
    /// Native relationship label.
    pub relation: Text,
    /// Native mechanism wording, when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mechanism: Option<Text>,
    /// Native relation provenance.
    pub attribution: Attribution,
}
/// A source indication retaining its full population and combination wording.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugIndicationClaim {
    /// Source drug identity.
    pub drug: DrugIdentity,
    /// Complete native indication wording.
    pub indication: Text,
    /// Population wording explicitly supplied separately.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub population: Option<Text>,
    /// Complete source-stated combination members, without local joins.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub combination: Box<[DrugIdentity]>,
    /// Native claim provenance.
    pub attribution: Attribution,
}
/// One native interaction pair and level, without invented management advice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugInteractionClaim {
    /// Selected native anchor ID and name.
    pub anchor: DrugIdentity,
    /// Native partner ID and name.
    pub partner: DrugIdentity,
    /// Original native level, without a computed severity.
    pub level: Text,
    /// Pair record and bundle provenance.
    pub attribution: Attribution,
}
