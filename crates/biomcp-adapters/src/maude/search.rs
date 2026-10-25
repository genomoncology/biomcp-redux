use super::{SPEC, raw};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_types::{AdverseEventHeader, AdverseEventSearch, AdverseEventSource, Source};
/// Literal analyzed device generic-name phrase; no wildcard or synonym expansion.
pub fn expression(query: &AdverseEventSearch) -> String {
    format!("device.generic_name:\"{}\"", query.text().as_str().replace('\\',"\\\\").replace('"',"\\\""))
}
/// Read one source-ordered report page after component admission.
pub fn search(leg: &Leg<'_>, query: &AdverseEventSearch) -> Result<crate::faers::Page, RequestError> {
    crate::adverse_event_sources::components::gmdn(leg)?;
    let selector = expression(query);
    let endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
    let (reply, metadata) = leg.json_received_checked::<raw::Response>(&endpoint, &[
        ("search",Arg::Text(&selector)),("limit",Arg::Num(query.limit().into())),("skip",Arg::Num(query.offset().into())),("sort",Arg::Text("date_received:desc")),
    ], |reply| {
        if let raw::Response::Page {meta,..}=reply && (meta.results.skip != query.offset() || meta.results.limit != query.limit()) { return Err(BadReply); }
        Ok(())
    })?;
    let (items,total,updated) = match reply {
        raw::Response::Page {meta,results} => (results.into_iter().map(|r|header(&r)).collect(),meta.results.total,meta.last_updated),
        raw::Response::Absent {..} => (Vec::new(),0,None),
    };
    Ok(crate::faers::Page { components: crate::adverse_event_sources::components::labels(Source::Maude,Source::Gmdn)?,items,total,updated,metadata })
}
pub(super) fn header(row: &raw::Report) -> AdverseEventHeader {
    AdverseEventHeader { id:row.mdr_report_key.clone(),source:AdverseEventSource::Maude,version:None,serious:None,event_type:row.event_type.clone(),first_received:row.date_received.clone() }
}
