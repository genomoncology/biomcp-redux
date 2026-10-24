use crate::{Action, Ctx, Outcome, status};
use biodata::{
    ArticleInput, ArticleResolution, ArticleText, FullText, Pmcid, Reason, Request, Section,
    Source, Status, StepKind,
};
use biomcp_adapters::pmc_oa;
use biomcp_fetch::Deadline;
/// Outline owner; the same document reader supplies text commands.
pub(super) fn run<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, out: &mut Outcome) {
    let biodata::QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let leg = cx.fetch.leg(&pmc_oa::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut error = None;
    let mut loaded: Vec<((Pmcid, Option<biodata::Pmid>), Section<FullText>)> = Vec::new();
    for item in &mut cards.items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        let Some(sections) = &mut item.sections else {
            continue;
        };
        if sections.fulltext.is_some() {
            continue;
        }
        let Some(id) = card.pmcid else {
            continue;
        };
        let section = if let Some((_, section)) = loaded.iter().find(|(found, _)| *found == (id, card.pmid)) {
            section.clone()
        } else {
            let section = match pmc_oa::read(&leg, id, card.pmid) {
                Ok(Some(document)) => Section::Data {
                    source: Source::PmcOa,
                    data: outline(&document),
                },
                Ok(None) => unavailable(Reason::NotOpen),
                Err(e) => {
                    error.get_or_insert(e);
                    out.local = out.local.or(status::local(e));
                    unavailable(match e {
                        biomcp_fetch::RequestError::Source(e) => e.reason,
                        _ => Reason::Unavailable,
                    })
                }
            };
            loaded.push(((id, card.pmid), section.clone()));
            section
        };
        if matches!(section, Section::Unavailable { .. }) {
            item.status = Status::Partial;
        }
        if matches!(&section, Section::Data { data, .. } if !data.parts.is_empty())
            && out.next.len() < 5
        {
            next(id, 1, &mut out.next);
        }
        sections.fulltext = Some(section);
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::PmcOa, leg.finish(), error);
    if !loaded.is_empty() {
        if source.state == biodata::SourceState::Stale {
            for item in &mut cards.items {
                if item
                    .sections
                    .as_ref()
                    .is_some_and(|s| matches!(s.fulltext, Some(Section::Data { .. })))
                {
                    item.status = Status::Partial;
                }
            }
        }
        out.sources.push(source);
    }
    out.status = status::status(cards.items.iter().map(|i| i.status));
}
fn unavailable(reason: Reason) -> Section<FullText> {
    Section::Unavailable {
        source: Source::PmcOa,
        reason,
    }
}
pub(crate) fn outline(document: &pmc_oa::Document) -> FullText {
    FullText {
        license: document.license,
        processing_version: document.version,
        manuscript: document.manuscript,
        parts: document.body.parts.iter().map(|p| p.meta.clone()).collect(),
        figures: document.body.figures.clone().into(),
    }
}
pub(crate) fn next(id: Pmcid, part: u32, out: &mut Vec<(StepKind, Action)>) {
    if let Ok(query) = ArticleText::part(ArticleInput::Pmcid(id), part) {
        out.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleText(query),
                fresh: false,
            },
        ));
    }
}
