use super::{empty, overall, problem, recovery};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{hpo, monarch};
use biomcp_fetch::{InputError, RequestError};
use biomcp_types::{
    PhenotypeGet, PhenotypeSource, PhenotypeTermCard, QueryBody, Request, Source, Status,
};
/// Read exactly one source card, with no merge or fallback request.
pub fn get(cx: &Ctx<'_>, q: &PhenotypeGet) -> Outcome {
    let mut out = empty(QueryBody::PhenotypeCard(None));
    if q.section().is_some() {
        out.local = Some(InputError::Path.into());
        return out;
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let (spec, source) = match q.source() {
        PhenotypeSource::Hpo => (&hpo::SPEC, Source::Hpo),
        PhenotypeSource::Monarch => (&monarch::SPEC, Source::Monarch),
    };
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let reply = match q.source() {
        PhenotypeSource::Hpo => hpo::get(&leg, q).map(|r| r.card),
        PhenotypeSource::Monarch => monarch::phenotype::get(&leg, q).map(|r| r.card),
    };
    let error = reply.as_ref().err().copied();
    let absent =
        matches!(error,Some(RequestError::Source(e)) if e.reason==biomcp_types::Reason::NotFound);
    out.sources.push(status::source(
        source,
        leg.finish(),
        if absent { None } else { error },
    ));
    out.local = error.and_then(status::local);
    match reply {
        Ok(card) => apply(&mut out, card),
        _ if absent => out.status = Status::Absent,
        _ => (),
    }
    problem(&mut out, source, q.id().as_str().parse().ok());
    recovery(&mut out, Request::GetPhenotype(q.clone()));
    command.finish(deadline.remaining());
    out
}
fn apply(out: &mut Outcome, card: PhenotypeTermCard) {
    out.body = QueryBody::PhenotypeCard(Some(card));
    out.status = overall(out, false);
}
