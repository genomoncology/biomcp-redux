use serde::{Serialize, Deserialize};
/// Registry page preserving source order and exact request parameters.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialPage {
    /// Native study headers in registry order.
    pub items: Box<[crate::TrialHeader]>,
    /// Exact parameters sent to the study search endpoint.
    pub request: std::collections::BTreeMap<String, String>,
    /// Registry's source processing timestamp, independently of retrieval time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_updated: Option<crate::Text>,
    /// Unresolved gene identity, without a registry request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene_resolution: Option<crate::GeneResolution>,
    /// Publisher attribution.
    pub attribution: String,
}
impl super::ResponseBody for TrialPage {
    const SCHEMA: &'static str = "biomcp.trial_page/1";
    type Query = crate::TrialSearch;
}
