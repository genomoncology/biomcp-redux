//! Source predictions retain their independent component terms and source labels.
use crate::Ctx;

use biomcp_fetch::Deadline;
use biomcp_types::{VariantGet, VariantHeader, VariantPredictors};
pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: &mut Deadline<'_>,
) -> super::sections::Result<VariantPredictors> {
    super::components::measurements(cx, query, candidates, deadline, false)
}
