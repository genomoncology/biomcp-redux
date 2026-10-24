use crate::{Ctx, status};
use biodata::{
    Annotations, ArticleCardItem, ArticleResolution, Pmid, Problem, Reason, Section, Source,
    SourceOutcome, SourceState, Status,
};
use biomcp_adapters::pubtator;
use biomcp_fetch::{Deadline, RequestError};
pub(super) struct Result {
    pub values: Vec<(Pmid, Section<Annotations>)>,
    pub source: SourceOutcome,
    pub problems: Vec<Problem>,
    pub local: Option<RequestError>,
}
pub(super) fn ids(items: &[ArticleCardItem]) -> Vec<Pmid> {
    let mut ids = Vec::new();
    for item in items {
        if let Some(ArticleResolution::Resolved { card, .. }) = &item.answer
            && let Some(id) = card.pmid
            && !ids.contains(&id)
        {
            ids.push(id);
        }
    }
    ids
}
pub(super) fn run<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, ids: &[Pmid]) -> Result {
    let leg = cx.fetch.leg(&pubtator::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let chunks: Vec<_> = ids.chunks(pubtator::BATCH_LIMIT).collect();
    let replies = leg.batch(&chunks, |leg, ids| pubtator::annotations(leg, ids));
    let error = replies.iter().find_map(|r| r.as_ref().err().copied());
    let local = replies
        .iter()
        .find_map(|r| r.as_ref().err().copied().and_then(status::local));
    let mut values = Vec::new();
    let mut problems = Vec::new();
    for (chunk, reply) in chunks.iter().zip(replies) {
        match reply {
            Ok(rows) => {
                for (id, mapped) in rows {
                    values.push((id, mapped.value));
                    status::append(&mut problems, mapped.problems);
                }
            }
            Err(error) => {
                let reason = match error {
                    RequestError::Source(e) => e.reason,
                    _ => Reason::Unavailable,
                };
                values.extend(chunk.iter().map(|id| {
                    (
                        *id,
                        Section::Unavailable {
                            source: Source::PubTator,
                            reason,
                        },
                    )
                }));
            }
        }
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    Result {
        values,
        source: status::source(Source::PubTator, leg.finish(), error),
        problems,
        local,
    }
}
pub(super) fn join(items: &mut [ArticleCardItem], result: &Result) {
    for item in items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        let Some((id, section)) = result.values.iter().find(|(id, _)| Some(*id) == card.pmid)
        else {
            continue;
        };
        if matches!(section, Section::Unavailable { .. })
            || result.source.state == SourceState::Stale
            || result.problems.iter().any(|p| {
                p.record
                    .as_ref()
                    .is_some_and(|r| r.as_str() == id.to_string())
            })
        {
            item.status = Status::Partial;
        }
        if let Some(sections) = &mut item.sections {
            sections.annotations = Some(section.clone());
        }
    }
}
