//! One bounded NCI biomarker observation uses the documented native selectors.
use biodata::NcitId;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, RequestError};
use biomcp_types::{BiomarkerCard, BiomarkerSearch};
/// Maximum complete biomarker observation before a typed narrowing refusal.
pub const MAX_BYTES: u64 = 1024 * 1024;
const SEARCH: Endpoint = super::SPEC
    .get("/biomarkers")
    .fresh(Freshness::Days(1))
    .once();
/// Retrieve native name or synonym matches, without source paging controls.
pub fn search(
    leg: &Leg<'_>,
    q: &BiomarkerSearch,
) -> Result<crate::Mapped<Vec<BiomarkerCard>>, RequestError> {
    let args: Vec<_> = q
        .name()
        .map(|name| ("name", Arg::Text(name.as_str())))
        .into_iter()
        .collect();
    leg.json_bounded::<super::biomarker_raw::Page>(&SEARCH, &args, MAX_BYTES)
        .map(super::biomarker_map::page)
}
/// Search exact native codes; reject unrelated identities before cache admission.
pub fn get(leg: &Leg<'_>, id: NcitId) -> Result<crate::Mapped<Vec<BiomarkerCard>>, RequestError> {
    let code = id.to_string();
    leg.json_bounded_checked::<super::biomarker_raw::Page>(
        &SEARCH,
        &[("codes", Arg::Text(&code))],
        MAX_BYTES,
        |page| {
            if page.data.iter().all(|row| row.codes.contains(id)) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )
    .map(super::biomarker_map::page)
}
