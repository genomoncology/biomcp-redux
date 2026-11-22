//! The FDA's indication-level table of withdrawn accelerated approvals.
use biodata::Text;
use serde::{Deserialize, Serialize};

/// One withdrawn cancer indication as listed by FDA.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugWithdrawal {
    /// The drug name as FDA printed it.
    pub drug: Box<str>,
    /// The indication FDA withdrew.
    pub indication: Box<str>,
    /// The accelerated approval date as FDA printed it.
    pub approval_date: Box<str>,
    /// The withdrawal date as FDA printed it.
    pub withdrawal_date: Box<str>,
    /// FDA's source table.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
    /// The approval notice linked from the indication, including archived notices.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub indication_url: Option<Text>,
}

/// Matching indications from the FDA withdrawal list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugWithdrawalPage {
    /// Number of matching indications.
    pub total: u64,
    /// Collection date when the shipped table answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_date: Option<Box<str>>,
    /// Rows as listed by FDA.
    #[serde(default)]
    pub items: Box<[DrugWithdrawal]>,
}

impl crate::ResponseBody for DrugWithdrawalPage {
    const SCHEMA: &'static str = "biomcp.drug-withdrawal-page/1";
    type Query = super::DrugSearch;
}
