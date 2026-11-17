//! NIH RePORTER project search, paged by RePORTER itself.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::nih_reporter;
use biomcp_types::{GrantPage, GrantSearch, ReplyBody, Request, Source, Status, StepKind};

/// RePORTER refuses an offset past this.
const MAX_OFFSET: u64 = 14_999;

/// Ask RePORTER for one page and offer the next while RePORTER can serve it.
pub(crate) fn search(cx: &Ctx<'_>, query: &GrantSearch) -> Outcome {
    let command = cx
        .fetch
        .source_command_deadline(&nih_reporter::SPEC, cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&nih_reporter::SPEC, deadline).fresh(cx.fresh);
    let reply = nih_reporter::search(&leg, query);
    let error = reply.as_ref().err().copied();
    let mut out = status::empty(GrantPage::into_reply(query.clone(), None));
    out.sources
        .push(status::source(Source::NihReporter, leg.finish(), error));
    match reply {
        Ok((total, items, problems)) => {
            out.problems = problems;
            publish(&mut out, query, total, items);
        }
        Err(error) => out.local = status::local(error),
    }
    if out.status() == Status::Failed {
        out.next.push(crate::next::Step::run(
            StepKind::Fresh,
            &[Source::NihReporter],
            Request::SearchGrant(query.clone()),
            true,
        ));
    }
    command.finish(deadline.remaining());
    out
}
fn publish(out: &mut Outcome, query: &GrantSearch, total: u64, items: Vec<biomcp_types::Grant>) {
    let returned = u32::try_from(items.len()).unwrap_or(u32::MAX);
    let next = u64::from(query.offset()) + u64::from(returned);
    out.page = Some(status::page_info(query.offset(), returned, total));
    status::search(out);
    out.body = (GrantPage {
        items: items.into(),
    })
    .with_query(query.clone());
    if returned > 0 && next < total && next <= MAX_OFFSET {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[Source::NihReporter],
            Request::SearchGrant(query.at_offset(u32::try_from(next).unwrap_or(u32::MAX))),
            false,
        ));
    }
}
