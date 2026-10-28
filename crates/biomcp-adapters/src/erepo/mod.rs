//! ClinGen evidence documents by exact CAid, without gene-wide fallback.
mod history;
mod raw;
use crate::quotas;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::{Source, VariantAssertion};
use std::time::Duration;
/// Exact public evidence repository policy; record rights remain source-specific.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "ClinGen sharing policy; record rights unverified",
        url: "https://clinicalgenome.org/about/terms-of-use/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::Erepo,
    base: "https://erepo.clinicalgenome.org",
    key: KeyUse::None,
    quota: &quotas::CLINGEN,
    deadline: Duration::from_secs(8),
};
const FETCH: Endpoint = SPEC
    .get("/evrepo/api/summary/classifications")
    .json_absence()
    .fresh(Freshness::Days(1));
/// Read native pages until completion, retaining source order and checked request identity.
pub fn assertions(
    leg: &Leg<'_>,
    caid: &str,
    selection: Option<(&str, Option<&str>)>,
) -> Result<Vec<VariantAssertion>, RequestError> {
    let records = records(leg, caid)?;
    if let Some((id, Some(version))) = selection {
        let record = records.iter().find(|r| r.id == id).ok_or_else(bad)?;
        // The summary names its own exact current document version.
        // Older selections use their own SEPIO body and cannot borrow these facts.
        if record.version != version {
            let uuid = record.uuid.as_deref().ok_or_else(bad)?;
            if !record
                .versions
                .as_ref()
                .is_some_and(|v| v.iter().any(|v| v == version))
            {
                return Err(bad());
            }
            return history::fetch(leg, caid, id, uuid, version).map(|r| vec![r]);
        }
    }
    records.into_iter().map(raw::Record::map).collect()
}
fn records(leg: &Leg<'_>, caid: &str) -> Result<Vec<raw::Record>, RequestError> {
    let mut rows = Vec::new();
    let mut page = 1;
    loop {
        let params = [
            ("columns", Arg::Text("caId")),
            ("values", Arg::Text(caid)),
            ("matchTypes", Arg::Text("exact")),
            ("pgSize", Arg::Num(3)),
            ("pg", Arg::Num(page)),
        ];
        let (reply, _) = leg.json_received_checked::<raw::ReplyBody>(&FETCH, &params, |r| {
            if r.data
                .as_ref()
                .is_none_or(|rows| rows.iter().all(|r| r.ca_id == caid))
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        })?;
        let data = reply.data.unwrap_or_default();
        let complete = data.len() < 3;
        for row in data {
            rows.push(row);
        }
        if complete {
            return Ok(rows);
        }
        page = page.checked_add(1).ok_or_else(bad)?;
    }
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Erepo,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
