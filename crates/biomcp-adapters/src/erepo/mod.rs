//! ClinGen evidence documents by exact CAid, without gene-wide fallback.
mod history;
mod raw;
use crate::{Mapped, mapped::Problems, quotas};
use biomcp_fetch::{Arg, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
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
) -> Result<Mapped<Vec<VariantAssertion>>, RequestError> {
    let mapped = records(leg, caid)?;
    let records = mapped.value;
    let mut cx = Problems { source: Source::Erepo, record: None, list: mapped.problems };
    if let Some((id, Some(version))) = selection {
        let record = records.iter().find(|r| r.id == id).ok_or_else(bad)?;
        // The summary names its own exact current document version.
        // Older selections use their own SEPIO body and cannot borrow these facts.
        if record.version != version {
            let uuid = record.uuid.as_ref().and_then(crate::raw::Field::as_ref).ok_or_else(bad)?;
            if !record
                .versions
                .as_ref()
                .and_then(crate::raw::Field::as_ref)
                .is_some_and(|v| v.iter().filter_map(crate::raw::Field::as_ref).any(|v| v.as_str() == version))
            {
                return Err(bad());
            }
            let mapped = history::fetch(leg, caid, id, uuid, version)?;
            cx.list.extend(mapped.problems);
            return Ok(Mapped { value: mapped.value.into_iter().collect(), problems: cx.list });
        }
    }
    let value = records.into_iter().filter_map(|row| row.map(caid, &mut cx)).collect();
    Ok(Mapped { value, problems: cx.list })
}
fn records(leg: &Leg<'_>, caid: &str) -> Result<Mapped<Vec<raw::Record>>, RequestError> {
    let mut rows = Vec::new();
    let mut cx = Problems { source: Source::Erepo, record: None, list: Vec::new() };
    let mut page = 1;
    loop {
        let params = [
            ("columns", Arg::Text("caId")),
            ("values", Arg::Text(caid)),
            ("matchTypes", Arg::Text("exact")),
            ("pgSize", Arg::Num(3)),
            ("pg", Arg::Num(page)),
        ];
        let (reply, _) = leg.json_received_checked::<raw::ReplyBody>(&FETCH, &params, |_| Ok(()))?;
        let data = reply.data.unwrap_or_default();
        let complete = data.len() < 3;
        for raw in data {
            cx.record = None;
            let Some(row) = cx.field("record", Some(raw)) else { continue; };
            cx.record = cx.text("PCERDocID", Some(&row.id));
            let version = cx.text("docVersion", Some(&row.version));
            let allele = cx.optional("caId", Some(row.ca_id.parse::<biodata::VariantId>()));
            if cx.record.is_some() && version.is_some() && allele.is_some() {
                if row.ca_id == caid { rows.push(row); }
                else { cx.required::<()>("caId", None); }
            }
        }
        if complete {
            return Ok(Mapped { value: rows, problems: cx.list });
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
