pub use biodata::{PathwayRef, ReactomeId};
use serde::{Deserialize, Serialize};
/// A bounded list with the total before its cap was applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "Capped_{T}"))]
pub struct Capped<T> {
    // The total comes before the rows (0942): a reader of only the start sees it.
    /// Distinct references before capping.
    pub total: u64,
    /// References in stable source order.
    #[serde(default)]
    pub items: Box<[T]>,
}
