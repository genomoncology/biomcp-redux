//! Source predictions retain their independent component terms and source labels.
use crate::Ctx;
use biodata::{VariantGet, VariantHeader, VariantPredictors};
use biomcp_fetch::Deadline;
pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: &mut Deadline<'_>,
) -> super::sections::Result<VariantPredictors> {
    super::components::measurements(cx, query, candidates, deadline, false)
}
