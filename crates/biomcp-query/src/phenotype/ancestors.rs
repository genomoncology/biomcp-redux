//! Every ancestor of one HP term in one OLS reply, independent of Monarch.
use super::{empty, problem, recovery};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::ols;
use biomcp_types::{
    PhenotypeAncestor, PhenotypeAncestors, PhenotypeGet, PhenotypeSection, ReplyBody, Request,
    Source, StepKind,
};
pub(super) fn get(cx: &Ctx<'_>, q: &PhenotypeGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&ols::SPEC, deadline).fresh(cx.fresh);
    let reply = ols::ancestors(&leg, q.id());
    let error = reply.as_ref().err().copied();
    let mut out = empty(PhenotypeAncestors::into_reply(q.clone(), None));
    out.sources
        .push(status::source(Source::Ols, leg.finish(), error));
    out.local = error.and_then(status::local);
    match reply {
        Ok(Some(page)) => {
            out.include_refusals(page.refusals);
            out.problems.extend(page.problems);
            let items = super::sections::page(&mut out, q, &page.items);
            replacements(&mut out, &page.replaced_by);
            out.body = (PhenotypeAncestors {
                items,
                obsolete: page.obsolete.then_some(true),
                replaced_by: page.replaced_by.into(),
                components: page.components,
                fetched: page
                    .metadata
                    .fetched
                    .and_then(|value| u64::try_from(value).ok()),
            })
            .with_query(q.clone());
        }
        Ok(None) => crate::status::item(&mut out, false, false, false),
        Err(_) => (),
    }
    problem(&mut out, Source::Ols, q.id().as_str().parse().ok());
    recovery(&mut out, Request::GetPhenotype(q.clone()));
    command.finish(deadline.remaining());
    out
}
/// An obsolete term has no ancestors; its named replacement does.
fn replacements(out: &mut Outcome, terms: &[PhenotypeAncestor]) {
    for term in terms {
        // A replacement outside HP gets no step; the reply still names it.
        if let Ok(id) = term.id.to_string().parse::<biodata::HpId>()
            && let Ok(request) = PhenotypeGet::new(id).with_section(PhenotypeSection::Ancestors)
        {
            out.next.push(crate::next::Step::new(
                StepKind::Section,
                &[Source::Ols],
                super::run(Request::GetPhenotype(request)),
            ));
        }
    }
}
