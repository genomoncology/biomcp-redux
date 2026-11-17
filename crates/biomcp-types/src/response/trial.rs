use serde::{Deserialize, Serialize};
/// Registry page preserving source order and source processing time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialPage {
    /// Native study headers in registry order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[crate::TrialHeader]>,
    /// Registry's source processing timestamp, independently of retrieval time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_updated: Option<crate::SourceUpdated>,
    /// Unresolved gene identity, without a registry request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene_resolution: Option<crate::GeneResolution>,
    /// Exact condition identity and source-provided mapping, absent for literal text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_resolution: Option<crate::TrialConditionResolution>,
    /// With --site-count, in place of rows: facilities in the most matched trials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_counts: Option<TrialSiteCounts>,
    /// Publisher attribution.
    pub attribution: String,
}

/// Facilities counted across the matched trials read, each name exactly as the registry
/// writes it with its city and country; different spellings of one facility stay apart.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialSiteCounts {
    /// Registry total of matched trials.
    pub trials_matched: u64,
    /// Number of trial records read across the returned registry pages.
    pub trials_read: u64,
    /// Distinct facility, city and country rows among the trials read.
    pub facilities_total: u64,
    /// Facilities by number of trials, most first, then by name.
    pub facilities: Box<[TrialSiteCount]>,
}

/// One registered facility and the number of trials read that list it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialSiteCount {
    /// Facility name as registered.
    pub facility: Box<str>,
    /// Site city as registered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<str>>,
    /// Site country as registered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<str>>,
    /// Trials read that list this site.
    pub trials: u64,
}
impl super::ResponseBody for TrialPage {
    const SCHEMA: &'static str = "biomcp.trial-page/18";
    type Query = crate::TrialSearch;
}

/// One exact source request parameter, in canonical name order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialParameter {
    /// Native parameter name.
    pub name: String,
    /// Exact unencoded parameter value.
    pub value: String,
}
