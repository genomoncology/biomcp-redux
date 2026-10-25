use super::{SPEC, raw};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, ReplyMetadata, RequestError};
use biomcp_types::{AdverseEventGet, AdverseEventSection};
/// One identity-checked retained response; section reads share its ordinary transport cache.
#[derive(Debug)]
pub struct Report {
    /// Kept native source fields, without administrative contacts.
    pub record: raw::Report,
    /// Original retrieval metadata.
    pub metadata: ReplyMetadata,
}
/// Exact numeric native key; never probe another endpoint or source family.
pub fn get(leg: &Leg<'_>, query: &AdverseEventGet) -> Result<Option<Report>, RequestError> {
    if query.section() == Some(AdverseEventSection::Devices) {
        crate::adverse_event_sources::components::gmdn(leg)?;
    }
    let selector = format!("mdr_report_key:\"{}\"", query.id().as_str());
    let endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
    let (reply, metadata) = leg.json_received_checked::<raw::Response>(
        &endpoint,
        &[("search", Arg::Text(&selector)), ("limit", Arg::Num(1))],
        |reply| {
            if let raw::Response::Page { meta, results } = reply
                && (meta.results.total != 1
                    || meta.results.skip != 0
                    || meta.results.limit != 1
                    || results.iter().any(|r| r.mdr_report_key != *query.id()))
            {
                return Err(BadReply);
            }
            Ok(())
        },
    )?;
    Ok(match reply {
        raw::Response::Page { results, .. } => results
            .into_iter()
            .next()
            .map(|record| Report { record, metadata }),
        raw::Response::Absent { .. } => None,
    })
}
