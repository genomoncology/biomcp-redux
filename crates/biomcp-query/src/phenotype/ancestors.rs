//! Every ancestor of one HP term in one OLS reply, independent of Monarch.
use super::{empty, problem, recovery};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::ols;
use biomcp_types::{PhenotypeAncestors, PhenotypeGet, ReplyBody, Request, Source};
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
            out.problems.extend(page.problems);
            let items = super::sections::page(&mut out, q, &page.items);
            out.body = (PhenotypeAncestors {
                items,
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
