use super::{empty, overall, run};
use crate::{Ctx, Outcome, status};
use biodata::PathwayId;
use biomcp_adapters::reactome;
use biomcp_types::{PathwayGet, PathwaySection, QueryBody, Request, Source, StepKind};
/// Read one exact native human pathway; preserve historical absence without a replacement search.
pub fn get(cx: &Ctx<'_>, q: &PathwayGet) -> Outcome {
    let mut out = empty(QueryBody::PathwayCard(None));
    let PathwayId::Reactome(id) = q.id() else {
        return out;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx
        .fetch
        .leg(&reactome::SPEC, deadline)
        .fresh(cx.fresh || q.fresh());
    let reply = reactome::card(&leg, id, q.section() == Some(PathwaySection::Summary));
    let mut error = reply.as_ref().err().copied();
    if let Ok(mapped) = reply {
        out.problems = mapped.problems;
        let mut card = mapped.value;
        if let Some(value) = &mut card {
            match reactome::release(&leg) {
                Ok(release) => value.release = Some(release),
                Err(e) => {
                    error = Some(e);
                }
            }
            for section in &value.sections {
                if *section == PathwaySection::Summary
                    && Some(*section) != q.section()
                    && let Ok(query) = q.with_section(*section)
                {
                    out.next.push((
                        StepKind::Section,
                        run(Request::GetPathway(query)),
                    ));
                }
            }
        }
        let absent = card.is_none();
        out.body = QueryBody::PathwayCard(card);
        out.sources
            .push(status::source(Source::Reactome, leg.finish(), error));
        out.status = overall(&out, absent);
    } else {
        out.sources
            .push(status::source(Source::Reactome, leg.finish(), error));
    }
    out.local = error.and_then(status::local);
    command.finish(deadline.remaining());
    out
}
