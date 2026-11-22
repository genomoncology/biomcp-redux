//! Cell-line collection pages include all row arrays, even when empty.
use crate::Capped;
use serde::{Serialize, Serializer};

/// A bounded list with the total before its cap was applied.
#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "CappedPage_{T}"))]
pub(super) struct CappedPage<'a, T> {
    /// Distinct references before capping.
    total: u64,
    /// References in stable source order.
    #[cfg_attr(feature = "schema", schemars(default, extend("default" = [])))]
    items: &'a [T],
}

pub(super) fn serialize<T: Serialize, S: Serializer>(
    value: &Capped<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    CappedPage {
        total: value.total,
        items: &value.items,
    }
    .serialize(serializer)
}
