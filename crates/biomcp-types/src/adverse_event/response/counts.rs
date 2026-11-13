//! Source-native reaction buckets without rates, denominators or causal inference.
use biodata::Text;
use serde::{Deserialize, Serialize};

/// One exact reaction term and its openFDA report count.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AdverseEventReactionCount {
    /// Exact native MedDRA term, without normalization or inferred drug association.
    pub term: Text,
    /// Native count of matching reports containing this term, not exposed people.
    pub count: u64,
}

/// Matching reports FDA received in one calendar year or month.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AdverseEventReceivedCount {
    /// Receipt year as YYYY or month as YYYY-MM.
    pub received: Box<str>,
    /// Matching reports with an FDA receipt date in this period, summed from openFDA's daily
    /// receipt-date counts; reports, not exposed people.
    pub count: u64,
    /// The requested range, or the source's data as of its last update, covers only part of
    /// this period.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub partial: bool,
}

/// Bounded native count response; buckets may overlap and are never summed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AdverseEventReactionCounts {
    /// Native reaction buckets in provider order; a report may contribute to several.
    pub items: Box<[AdverseEventReactionCount]>,
    /// The provider returned as many terms as requested; more terms may exist.
    /// This does not establish a total or prove that terms were omitted.
    pub limit_reached: bool,
}
