//! Canonical diagnostic publisher declaration; runtime dispatch belongs to later slices.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::FdaPma,
    base: "https://api.fda.gov/device/pma.json",
    terms: Terms {
        name: "openFDA terms and license; third-party exceptions",
        url: "https://open.fda.gov/terms/",
        usage: Use::Open,
    },
    quota: &crate::quotas::OPENFDA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
mod raw;
mod map;
pub use map::{header, regulatory, fragments};
use biodata::DeviceSubmission;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_types::DiagnosticGet;
/// Complete source application members or one exact selected supplement.
pub struct Records {
    /// Native records in original order, without harmonized values.
    pub rows: Vec<DeviceSubmission>,
    /// True only when the source application exceeds its single admitted page.
    pub incomplete: bool,
}
/// Validate exact source identity before any record can enter the ordinary cache.
pub fn get(leg: &Leg<'_>, query: &DiagnosticGet) -> Result<Records, RequestError> {
    let mut selector = format!("pma_number:\"{}\"", query.id());
    if let Some(supplement) = query.supplement() {
        selector.push_str(&format!(" AND supplement_number:\"{supplement}\""));
    }
    let endpoint = SPEC.get("").fresh(Freshness::Days(1)).bytes(10 * 1024 * 1024).json_absence();
    let incomplete = std::cell::Cell::new(false);
    let reply = leg.json_received_checked::<raw::Response>(&endpoint, &[("search",Arg::Text(&selector)),("limit",Arg::Num(100)),("skip",Arg::Num(0))], |reply| {
        if let raw::Response::Page {meta,results} = reply {
            if meta.results.total > 100 { incomplete.set(true); return Err(BadReply); }
            if meta.results.skip != 0 || meta.results.limit != 100 || results.iter().any(|row| row.pma_number.as_str() != query.id().as_str() || row.supplement_number.as_ref().is_none_or(|cell| query.supplement().is_some_and(|s| s.as_str() != cell.as_str()))) {return Err(BadReply);}
            let mut seen = std::collections::BTreeSet::new();
            if results.iter().any(|row| !seen.insert(row.supplement_number.clone())) {return Err(BadReply);}
        }
        Ok(())
    });
    if incomplete.get() { return Ok(Records {rows:vec![],incomplete:true}); }
    let (reply, _) = reply?;
    Ok(match reply {
        raw::Response::Page {meta,results} => Records {rows:results.into_iter().map(raw::Record::mapped).collect(),incomplete:meta.results.total > 100},
        raw::Response::Absent {..} => Records {rows:vec![],incomplete:false},
    })
}

fn bad() -> RequestError { biomcp_fetch::SourceError {source:Source::FdaPma,reason:biomcp_types::Reason::BadReply,status:Some(200),retry_after:None}.into() }
