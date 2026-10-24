//! ClinGen evidence documents by exact CAid, without gene-wide fallback.
mod raw;
use crate::quotas;
use biodata::{Source, VariantAssertion};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use std::time::Duration;
/// Exact public evidence repository policy; record rights remain source-specific.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "ClinGen sharing policy; record rights unverified",
        url: "https://clinicalgenome.org/about/terms-of-use/",
        usage: biodata::Use::Restricted,
    },
    source: Source::Erepo,
    base: "https://erepo.clinicalgenome.org",
    key: KeyUse::None,
    quota: &quotas::CLINGEN,
    deadline: Duration::from_secs(8),
};
const FETCH: Endpoint = SPEC
    .get("/evrepo/api/summary/classifications")
    .fresh(Freshness::Days(1));
/// Read native pages until completion, retaining source order and checked request identity.
pub fn assertions(leg: &Leg<'_>, caid: &str) -> Result<Vec<VariantAssertion>, RequestError> {
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
            if r.data.iter().all(|r| r.ca_id == caid) {
                Ok(())
            } else {
                Err(BadReply)
            }
        })?;
        let complete = reply.data.len() < 3;
        for row in reply.data {
            rows.push(row.map()?);
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
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
