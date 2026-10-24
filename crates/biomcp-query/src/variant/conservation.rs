//! Restricted components remain unavailable until their own terms permit use.
use crate::Ctx;
use biodata::{VariantConservation, VariantGet, VariantHeader};
use biomcp_fetch::Deadline;
pub(super) fn section(
    _cx: &Ctx<'_>,
    _query: &VariantGet,
    _candidates: &[VariantHeader],
    _deadline: Deadline<'_>,
) -> super::components::Result<VariantConservation> {
    super::components::restricted()
}
