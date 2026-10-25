use super::{DrugGet, DrugSearch, DrugSection, DrugSource};
use crate::{CivicProfile, ResponseBody, Section, Step};
use biodata::{
    AuthorizationEvent, DrugId, DrugIdentityLevel, DrugIdentityRelation, DrugIndicationClaim,
    DrugInteractionClaim, DrugLabelReference, DrugTargetRelation, PartialDate, Text,
};
use serde::{Deserialize, Serialize};
/// One source candidate, without authorization, targets, label text or interactions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugHeader {
    /// Published native identifier.
    pub id: DrugId,
    /// The single source responsible for this header.
    pub source: DrugSource,
    /// Original source display name.
    pub name: Text,
    /// Level established only by the endpoint or record type.
    pub identity_level: DrugIdentityLevel,
    /// Native record type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Text>,
    /// Native source revision, omitted when the source supplies none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Source record URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Text>,
}
/// Exact source identity and explicit native links; deeper facts require named sections.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCard {
    /// Selected source-native identity.
    pub header: DrugHeader,
    /// Source-native alternate names, without local alias expansion.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub alternate_names: Box<[Text]>,
    /// Explicit source cross-references with their native relation and provenance.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub cross_references: Box<[DrugIdentityRelation]>,
    /// Explicit native parent links.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub parents: Box<[DrugIdentityRelation]>,
    /// Explicit native product links.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub products: Box<[DrugIdentityRelation]>,
    /// Explicit native formulation links.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub formulations: Box<[DrugIdentityRelation]>,
    /// Source record version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Source-stated update date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<PartialDate>,
    /// Actual retrieval date, separate from source publication or update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieved: Option<PartialDate>,
    /// Only disclosures available for this selected source identity.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub available_sections: Box<[DrugSection]>,
    /// Typed actions preserving exact source and identity.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub next: Box<[Step]>,
}
/// A single source candidate page in native order, without selecting the first hit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugPage {
    /// Native candidates, preserving their separate identity levels.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub items: Box<[DrugHeader]>,
    /// Native total when supplied; absent remains unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
}
/// One bounded native row page, without fabricating an unknown total.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct DrugRecords<T> {
    /// Native source rows in order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub records: Box<[T]>,
    /// Source row offset.
    pub offset: u32,
    /// Native total only when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Native source truncation indicator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    /// Dataset or bundle revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Actual retrieval date, never substituted for source revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieved: Option<PartialDate>,
}
/// A mechanical continuation of one native label heading.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelText {
    /// Exact source label set and revision.
    pub label: DrugLabelReference,
    /// Exact native label heading.
    pub heading: Text,
    /// Original text slice, preserving whitespace and wording.
    pub text: Box<str>,
    /// Continuation offset in the declared source text.
    pub offset: u32,
    /// Total source text length when known, without local clinical summarization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
}
/// One explicit native label heading and text continuation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabel {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub label: Section<DrugLabelText>,
}
/// Native target and mechanism relations, without a clinical indication inference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugTargets {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub targets: Section<DrugRecords<DrugTargetRelation>>,
}
/// Source-native indication wording, populations and combinations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugIndications {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub indications: Section<DrugRecords<DrugIndicationClaim>>,
}
/// Complete CIViC therapy/profile context with independent native claim continuations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCivic {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub civic: Section<DrugRecords<CivicProfile<DrugGet>>>,
}
/// Explicit native application/product history without a global approval flag.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugApprovals {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub approvals: Section<DrugRecords<AuthorizationEvent>>,
}
/// Source-scoped regulator events preserving date roles and original status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugRegulatory {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub regulatory: Section<DrugRecords<AuthorizationEvent>>,
}
/// Native DDInter partner rows and levels without invented management text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugInteractions {
    /// Selected native identity, checked independently of section facts.
    pub identity: DrugHeader,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub interactions: Section<DrugRecords<DrugInteractionClaim>>,
}
impl ResponseBody for DrugPage {
    const SCHEMA: &'static str = "biomcp.drug_page/1";
    type Query = DrugSearch;
}
impl ResponseBody for DrugCard {
    const SCHEMA: &'static str = "biomcp.drug_card/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugLabel {
    const SCHEMA: &'static str = "biomcp.drug_label/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugTargets {
    const SCHEMA: &'static str = "biomcp.drug_targets/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugIndications {
    const SCHEMA: &'static str = "biomcp.drug_indications/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugCivic {
    const SCHEMA: &'static str = "biomcp.drug_civic/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugApprovals {
    const SCHEMA: &'static str = "biomcp.drug_approvals/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugRegulatory {
    const SCHEMA: &'static str = "biomcp.drug_regulatory/1";
    type Query = DrugGet;
}
impl ResponseBody for DrugInteractions {
    const SCHEMA: &'static str = "biomcp.drug_interactions/1";
    type Query = DrugGet;
}
