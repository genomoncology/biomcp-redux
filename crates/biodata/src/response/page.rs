use serde::{Deserialize, Serialize};
/// Search page counts after filtering and deduplication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PageInfo {
    /// Skipped results.
    pub offset: u32,
    /// Results returned on this page.
    pub returned: u32,
    /// Total matching identities.
    pub total: u64,
}
