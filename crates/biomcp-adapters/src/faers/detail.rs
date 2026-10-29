//! Checked exact report get before ordinary cache admission.
use super::{SPEC, detail_raw as raw};
use biodata::{FaersReport, ReportId};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, ReplyMetadata, RequestError};
/// A single checked source report, absent only for the native no-match reply.
#[derive(Debug)]
pub struct Detail {
    /// Rejected optional source fields.
    pub problems: Vec<biomcp_types::Problem>,
    /// Exact selected native record.
    pub report: Option<FaersReport>,
    /// Credential-free URL used for this successful request.
    pub report_url: biodata::Text,
    /// Metadata belongs to the original source reply.
    pub metadata: ReplyMetadata,
}
/// Fetch one ID without choosing a version or a result among multiple records.
pub fn get(leg: &Leg<'_>, id: &ReportId) -> Result<Detail, RequestError> {
    let expression = format!("safetyreportid:\"{id}\"");
    let endpoint = SPEC.get("").fresh(Freshness::Days(1)).json_absence();
    let (reply, metadata) = leg.json_received_checked::<raw::Response>(
        &endpoint,
        &[
            ("search", Arg::Text(&expression)),
            ("limit", Arg::Num(5)),
            ("skip", Arg::Num(0)),
            ("sort", Arg::Text("receivedate:desc")),
        ],
        |response| {
            if let raw::Response::Found { results, .. } = response
                && results.first().is_none_or(|r| r.safetyreportid != *id)
            {
                return Err(BadReply);
            }
            Ok(())
        },
    )?;
    let report = match reply {
        raw::Response::Found { results, .. } => {
            results.into_iter().next().map(super::detail_map::report)
        }
        raw::Response::Absent { .. } => None,
    };
    let url = crate::registry::records::record_url(
        biomcp_types::Source::Faers,
        crate::registry::records::RecordKind::AdverseEvent,
        id.as_str(),
    )
    .ok_or(biomcp_fetch::InputError::Serialization)?;
    let report_url =
        biodata::Text::new(&url).map_err(|_| biomcp_fetch::InputError::Serialization)?;
    let (report, problems) = match report {
        Some(mapped) => (Some(mapped.value), mapped.problems),
        None => (None, Vec::new()),
    };
    Ok(Detail {
        problems,
        report,
        report_url,
        metadata,
    })
}
