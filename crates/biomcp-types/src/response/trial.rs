use serde::{Deserialize, Serialize};
/// Registry page preserving source order and exact request parameters.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialPage {
    /// Native study headers in registry order.
    pub items: Box<[crate::TrialHeader]>,
    /// Exact parameters sent to the study search endpoint.
    pub request: Box<[TrialParameter]>,
    /// Exact cached native count request when a continuation omits its total.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_request: Option<Box<[TrialParameter]>>,
    /// Registry's source processing timestamp, independently of retrieval time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_updated: Option<crate::Text>,
    /// Unresolved gene identity, without a registry request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene_resolution: Option<crate::GeneResolution>,
    /// Exact condition identity and source-provided mapping, absent for literal text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_resolution: Option<crate::TrialConditionResolution>,
    /// Publisher attribution.
    pub attribution: String,
}
impl super::ResponseBody for TrialPage {
    const SCHEMA: &'static str = "biomcp.trial-page/5";
    type Query = crate::TrialSearch;
}

/// One exact source request parameter, in canonical name order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialParameter {
    /// Native parameter name.
    pub name: String,
    /// Exact unencoded parameter value.
    pub value: String,
}
