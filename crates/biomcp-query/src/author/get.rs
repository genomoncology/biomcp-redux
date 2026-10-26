use super::{empty, recovery};
use crate::{Ctx, Outcome, status};
use biodata::AuthorId;
use biomcp_adapters::semantic_scholar::{self, author};
use biomcp_types::{
    AuthorCardItem, AuthorCards, AuthorGet, QueryBody, Reason, Request, Source, SourceState, Status,
};
/// Read each exact requested record once and preserve every input slot.
pub fn get(cx: &Ctx<'_>, q: &AuthorGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx
        .fetch
        .leg(&semantic_scholar::SPEC, deadline)
        .fresh(cx.fresh);
    let mut out = empty(QueryBody::AuthorCards(AuthorCards {
        items: Box::default(),
    }));
    let mut items = Vec::new();
    let mut error = None;
    for input in q.ids() {
        let AuthorId::SemanticScholar(id) = input else {
            continue;
        };
        let reply = author::card(&leg, id);
        let status = match &reply {
            Ok(_) => Status::Complete,
            Err(biomcp_fetch::RequestError::Source(e)) if e.reason == Reason::NotFound => {
                Status::Absent
            }
            Err(e) => {
                out.local = out.local.or(status::local(*e));
                Status::Failed
            }
        };
        if let Err(e) = &reply
            && !matches!(e, biomcp_fetch::RequestError::Source(e) if e.reason == Reason::NotFound)
        {
            error = Some(*e);
        }
        items.push(AuthorCardItem {
            input: input.clone(),
            status,
            card: reply.ok(),
        });
    }
    let source = status::source(Source::SemanticScholar, leg.finish(), error);
    if source.state == SourceState::Stale {
        for item in &mut items {
            if item.status == Status::Complete {
                item.status = Status::Partial;
            }
        }
    }
    out.sources.push(source);
    out.status = status::status(items.iter().map(|i| i.status));
    recovery(cx, &mut out, Request::GetAuthor(q.clone()));
    for item in &items {
        if item.card.is_some() && out.next.len() < 5 {
            let step = (
                biomcp_types::StepKind::Pivot,
                crate::Action::AuthorPapers(biomcp_types::AuthorPapersQuery::new(
                    item.input.clone(),
                )),
            );
            if !out.next.contains(&step) {
                out.next.push(step);
            }
        }
    }
    out.body = QueryBody::AuthorCards(AuthorCards {
        items: items.into(),
    });
    out.next.truncate(5);
    command.finish(deadline.remaining());
    out
}
