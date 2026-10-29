//! Exact NCIt lookups keep multiple native rows as candidates.
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::nci;
use biomcp_types::{BiomarkerGet, BiomarkerResult, QueryBody, Reason, Source, Status, StepKind};
/// Read every native row attached to one exact code.
pub fn get(cx: &Ctx<'_>, q: &BiomarkerGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&nci::SPEC, deadline).fresh(cx.fresh);
    let reply = nci::biomarker::get(&leg, &q.id);
    let error = reply.as_ref().err().copied();
    let source = status::source(Source::Nci, leg.finish(), error);
    let mut out = Outcome {
        status: Status::Failed,
        body: QueryBody::BiomarkerResult(BiomarkerResult {
            card: None,
            candidates: Box::default(),
        }),
        page: None,
        sources: vec![source],
        problems: Vec::new(),
        next: Vec::new(),
        local: error.and_then(status::local),
    };
    if let Some(error) = error {
        if status::reason(error) == Some(Reason::TooLarge) {
            out.body = QueryBody::BiomarkerTooBroad;
        }
        if status::reason(error) == Some(Reason::NoKey) {
            out.next.push((StepKind::Key, Action::KeyHelp(Source::Nci)));
        }
    }
    if let Ok(reply) = reply {
        let mut rows = reply.value;
        let total = rows.len();
        if let Some(source) = out.sources.first_mut() {
            source.count = Some(total as u64);
            source.fetched = Some(total as u32);
        }
        out.status = if !reply.problems.is_empty()
            || out
                .sources
                .iter()
                .any(|s| s.state == biomcp_types::SourceState::Stale)
        {
            Status::Partial
        } else if total == 0 {
            Status::Absent
        } else {
            Status::Complete
        };
        for row in &rows {
            for id in &row.header.codes {
                let step = (StepKind::Pivot, Action::Run {
                    request: biomcp_types::Request::SearchTrial(biomcp_types::TrialSearch::for_nci_biomarker(id.clone())),
                    fresh: false,
                });
                out.next.push(step);
            }
        }
        out.problems = reply.problems;
        out.body = QueryBody::BiomarkerResult(BiomarkerResult {
            card: if total == 1 { rows.pop() } else { None },
            candidates: rows.into(),
        });
    }
    command.finish(deadline.remaining());
    out
}
