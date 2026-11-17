//! 510(k) records counted by product code beside a search page (1019).
use biodata::Text;
use serde::{Deserialize, Serialize};

fn is_zero(n: &u64) -> bool {
    *n == 0
}

/// One FDA product code and the matching 510(k) records filed under it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticProductCodeCount {
    /// Native FDA product code.
    pub product_code: Text,
    /// Matching 510(k) records with this product code.
    pub count: u64,
}

/// Matching 510(k) records counted by product code with openFDA's count facet (1019).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticProductCodeCounts {
    /// Product codes in openFDA's order, most records first.
    pub items: Box<[DiagnosticProductCodeCount]>,
    /// Matching records openFDA counts under an empty product code; omitted when none.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub without_product_code: u64,
    /// Sum of the listed counts and the records without a product code.
    pub sum: u64,
    /// The sum equals the page total, so every matching record is counted under a listed code.
    pub sum_matches_total: bool,
    /// openFDA returned its most codes for one count, 1,000; more codes may exist. A narrower
    /// decision-date range lists every code.
    pub limit_reached: bool,
}
