use crate::{Ctx, Outcome, status};
use biomcp_adapters::fda_510k;
use biomcp_types::{
    DiagnosticCard, DiagnosticGet, DiagnosticSearch, Problem, ProblemCode, QueryBody, Request,
    Source, SourceState, Status, StepKind,
};
pub(super) fn get(cx: &Ctx<'_>, q: &DiagnosticGet) -> Outcome {
    let mut out = super::empty(QueryBody::DiagnosticCard(None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&fda_510k::SPEC, deadline).fresh(cx.fresh);
    let error = match fda_510k::get(&leg, q) {
        Ok(card) => {
            {
                let found = card.is_some();
                crate::status::item(&mut out, found, false, false);
            };
            if card.is_some() {
                if let Ok(search) = serde_json::from_value::<DiagnosticSearch>(
                    serde_json::json!({"source":"fda-cdx","device_id":q.id()}),
                ) {
                    super::next::push(
                        &mut out,
                        StepKind::Pivot,
                        Request::SearchDiagnostic(search),
                        cx.fresh,
                    );
                }
                if !cx.fresh {
                    super::next::push(
                        &mut out,
                        StepKind::Fresh,
                        Request::GetDiagnostic(q.clone()),
                        true,
                    );
                }
            }
            out.body = QueryBody::DiagnosticCard(card.map(DiagnosticCard::Clearance));
            None
        }
        Err(failure) => {
            if failure.identity_conflict {
                out.problems.push(Problem {
                    source: Source::Fda510k,
                    record: Some(q.id().clone()),
                    field: "k_number".into(),
                    code: ProblemCode::IdentityConflict,
                });
            }
            Some(failure.error)
        }
    };
    out.local = error.and_then(status::local);
    let source = status::source(Source::Fda510k, leg.finish(), error);
    if source.state == SourceState::Stale && out.status() == Status::Complete {
        crate::status::item(&mut out, true, false, true);
    }
    out.sources.push(source);
    command.finish(deadline.remaining());
    out
}
