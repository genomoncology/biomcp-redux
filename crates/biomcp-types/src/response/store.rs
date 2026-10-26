/// Counts removed by explicit local store cleanup. Fresh state and bundles remain.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StoreClean {
    /// Expired source replies.
    pub replies: u64,
    /// Expired search pools.
    pub pools: u64,
    /// Items belonging to removed pools.
    pub items: u64,
    /// Expired trial and citation snapshots.
    pub snapshots: u64,
    /// Older incompatible cache files.
    pub files: u64,
}
impl super::ResponseBody for StoreClean {
    const SCHEMA: &'static str = "biomcp.store-clean/1";
    type Query = ();
}
