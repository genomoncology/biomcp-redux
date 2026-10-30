use super::{empty, finish_bundle};
use crate::{Action, Bundle, Ctx, Outcome};
use biomcp_adapters::who_ivd;
use biomcp_types::{DiagnosticCard, DiagnosticGet, QueryBody, Request, StepKind};

pub(super) fn get(cx: &Ctx<'_>, query: &DiagnosticGet) -> Outcome {
    let mut out = empty(QueryBody::DiagnosticCard(None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&who_ivd::SPEC, deadline);
    let result = who_ivd::get(&leg, query);
    let error = result.as_ref().err().copied();
    if let Ok((product, _info)) = result {
        {
        let (found, identity_failed, unavailable) = if product.is_some() {
            (true, false, false)
        } else {
            (false, false, false)
        };
        crate::status::item(&mut out, found, identity_failed, unavailable);
    };
        out.body = QueryBody::DiagnosticCard(product.map(DiagnosticCard::WhoIvdProduct));
        out.next.push((
            StepKind::Sync,
            Action::Sync {
                bundle: Bundle::WhoIvd,
            },
        ));
        if !cx.fresh {
            out.next.push((
                StepKind::Fresh,
                Action::Run {
                    request: Request::GetDiagnostic(query.clone()),
                    fresh: true,
                },
            ));
        }
    }
    finish_bundle(
        &mut out,
        leg,
        error,
        biomcp_types::Source::WhoIvd,
        Bundle::WhoIvd,
    );
    command.finish(deadline.remaining());
    out
}
