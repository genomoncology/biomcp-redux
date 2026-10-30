//! Reuse the shared source snapshot store; admission precedes every retained read.
use super::{
    Action, AdverseEventGet, AdverseEventSection, Ctx, Outcome, QueryBody, Request, Source, Status,
    StepKind, Text, maude,
};
use crate::{status, store::Store};
use biomcp_fetch::{CacheError, Deadline, RequestError, SourceError};
use biomcp_types::{Reason, SourceOutcome};
use sha2::{Digest, Sha256};
#[derive(serde::Deserialize)]
struct Saved(
    biodata::MaudeReport,
    Vec<SourceOutcome>,
    #[serde(default)] Vec<biomcp_types::Problem>,
    #[serde(default)] maude::Positions,
);
// Keep the established payload and snapshot identity when no field was rejected.
impl serde::Serialize for Saved {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut tuple = serializer.serialize_tuple(if !self.3.is_empty() {
            4
        } else if self.2.is_empty() {
            2
        } else {
            3
        })?;
        tuple.serialize_element(&self.0)?;
        tuple.serialize_element(&self.1)?;
        if !self.2.is_empty() || !self.3.is_empty() {
            tuple.serialize_element(&self.2)?;
        }
        if !self.3.is_empty() {
            tuple.serialize_element(&self.3)?;
        }
        tuple.end()
    }
}
fn key(query: &AdverseEventGet) -> Vec<u8> {
    Sha256::digest(format!("biomcp.maude_snapshot/1\0{}", query.id()).as_bytes()).to_vec()
}
pub(super) fn load(
    cx: &Ctx<'_>,
    query: &AdverseEventGet,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Option<(biodata::MaudeReport, maude::Positions, Text)>, RequestError> {
    let leg = cx.fetch.leg(&maude::SPEC, deadline).fresh(cx.fresh);
    let denied = if cx.fetch.disabled(Source::Maude) {
        Some(
            SourceError {
                source: Source::Maude,
                reason: Reason::Disabled,
                status: None,
                retry_after: None,
            }
            .into(),
        )
    } else if query.section() == Some(AdverseEventSection::Devices) {
        biomcp_adapters::adverse_event_sources::components::gmdn(&leg).err()
    } else {
        None
    };
    if let Some(error) = denied {
        out.sources
            .push(status::source(Source::Maude, leg.finish(), Some(error)));
        return Err(error);
    }
    let (saved, snapshot) = if let Some(snapshot) = query.snapshot() {
        let store = Store::open(cx.fetch.cache_access()?, deadline)?;
        let Some(payload) =
            store.source_snapshot(snapshot.as_str(), &key(query), |s| cx.fetch.disabled(s))?
        else {
            out.body = QueryBody::PoolError { mismatch: false };
            out.next.push((
                StepKind::Restart,
                Action::Run {
                    request: Request::GetAdverseEvent(query.restart()),
                    fresh: false,
                },
            ));
            return Ok(None);
        };
        let saved: Saved = serde_json::from_str(&payload).map_err(|_| CacheError::Database)?;
        if saved.0.mdr_report_key != *query.id() || !saved.3.check(&saved.0) {
            return Err(CacheError::Database.into());
        }
        (saved, snapshot.clone())
    } else {
        let reply = maude::get(&leg, query);
        let error = reply.as_ref().err().copied();
        out.sources
            .push(status::source(Source::Maude, leg.finish(), error));
        if matches!(error,Some(RequestError::Source(e)) if e.reason==Reason::BadReply) {
            out.problems.push(biomcp_types::Problem {
                source: Source::Maude,
                record: Text::new(query.id().as_str()).ok(),
                field: "report".into(),
                code: biomcp_types::ProblemCode::ValueUnknown,
            });
        }
        let Some(report) = reply? else {
            out.status = Status::Absent;
            return Ok(None);
        };
        let saved = Saved(
            report.value.data,
            out.sources.clone(),
            report.problems,
            report.value.positions,
        );
        let payload = serde_json::to_string(&saved).map_err(|_| CacheError::Database)?;
        let mut store = Store::open(cx.fetch.cache_access()?, deadline)?;
        let snapshot = store.save_source_snapshot(&key(query), &payload, &out.sources)?;
        (saved, snapshot)
    };
    out.sources = saved.1;
    out.problems.extend(saved.2);
    out.status = if out
        .sources
        .iter()
        .any(|s| s.state == biomcp_types::SourceState::Stale)
    {
        Status::Partial
    } else {
        Status::Complete
    };
    Ok(Some((saved.0, saved.3, snapshot)))
}
