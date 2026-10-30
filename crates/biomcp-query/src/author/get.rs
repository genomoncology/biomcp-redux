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
    let mut out = empty(QueryBody::AuthorCards(AuthorCards {
        items: Box::default(),
    }));
    let mut items = Vec::new();
    for input in q.ids() {
        let (spec, source) = match input {
            AuthorId::SemanticScholar(_) => (&semantic_scholar::SPEC, Source::SemanticScholar),
            AuthorId::Orcid(_) => (&biomcp_adapters::orcid::SPEC, Source::Orcid),
        };
        let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
        let reply = match input {
            AuthorId::SemanticScholar(id) => {
                author::card(&leg, id).map(|value| biomcp_adapters::Mapped {
                    value: Some(value.value),
                    problems: value.problems,
                })
            }
            AuthorId::Orcid(id) => biomcp_adapters::orcid::card(&leg, id),
        };
        let mut state = match &reply {
            Ok(mapped) if mapped.value.is_some() => Status::Complete,
            Ok(_) => Status::Absent,
            Err(biomcp_fetch::RequestError::Source(e)) if e.reason == Reason::NotFound => {
                Status::Absent
            }
            Err(e) => {
                out.local = out.local.or(status::local(*e));
                Status::Failed
            }
        };
        let outcome = status::source(
            source,
            leg.finish(),
            super::source_error(reply.as_ref().err().copied()),
        );
        if outcome.state == SourceState::Stale && state == Status::Complete {
            state = Status::Partial;
        }
        if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source) {
            status::retain_source_state(existing, &outcome);
        } else {
            out.sources.push(outcome);
        }
        let card = reply.ok().and_then(|mapped| {
            out.problems.extend(mapped.problems);
            mapped.value
        });
        items.push(AuthorCardItem {
            input: input.clone(),
            status: state,
            card,
        });
    }
    status::get(&mut out, items.iter().map(|i| i.status));
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
