pub use biodata::{PathwayRef, ReactomeId, ReactomeIdInvalid};
use serde::{Deserialize, Serialize};
/// A bounded list with the total before its cap was applied.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Capped<T> {
    /// References in stable source order.
    pub items: Box<[T]>,
    /// Distinct references before capping.
    pub total: u64,
}
