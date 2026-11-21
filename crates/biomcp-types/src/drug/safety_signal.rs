//! FDA's quarterly safety-signal rows and their report provenance.
use crate::ResponseBody;
use biodata::{PartialDate, Text};
use serde::{Deserialize, Serialize};

/// One row from a quarterly FDA safety-signal report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugSafetySignal {
    /// Product or product class as FDA names it.
    pub product: Text,
    /// FDA's potential signal or new safety information.
    pub signal: Text,
    /// FDA's additional information and follow-up.
    pub additional_information: Box<str>,
    /// Quarter as labeled by FDA.
    pub quarter: Text,
    /// FDA's quarterly report address.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
}

/// A page of matches from FDA's quarterly reports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugSafetySignalPage {
    /// Number of matching rows.
    pub total: u64,
    /// When the shipped tables were collected, if used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_date: Option<PartialDate>,
    /// Listed reports that could not be read.
    pub unread_quarters: u32,
    /// FDA's statement on the meaning of a listing.
    pub caution: Box<str>,
    /// Matching rows.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[DrugSafetySignal]>,
}

impl ResponseBody for DrugSafetySignalPage {
    const SCHEMA: &'static str = "biomcp.drug-safety-signal-page/1";
    type Query = super::DrugSearch;
}
