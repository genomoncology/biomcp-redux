use crate::{Action, Ctx, Outcome, status};
use biodata::{
    ArticleInput, ArticleResolution, ArticleSupplement, QueryBody, Request, Section, Source,
    Status, StepKind, Supplements,
};
use biomcp_adapters::pmc_oa;
use biomcp_fetch::Deadline;
pub(super) fn run<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, out: &mut Outcome) {
    let QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let leg = cx.fetch.leg(&pmc_oa::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut error = None;
    let mut loaded = Vec::new();
    for item in &mut cards.items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        let Some(sections) = &mut item.sections else {
            continue;
        };
        if sections.supplements.is_some() {
            continue;
        }
        let Some(id) = card.pmcid else {
            continue;
        };
        let section = if let Some((_, section)) =
            loaded.iter().find(|(found, _)| *found == (id, card.pmid))
        {
            section.clone()
        } else {
            let section = match pmc_oa::read(&leg, id, card.pmid) {
                Ok(Some(doc)) => {
                    let files = doc
                        .supplements
                        .into_iter()
                        .map(|v| v.file)
                        .collect::<Vec<_>>();
                    if let Some(first) = files.iter().find(|v| v.kind.readable())
                        && out.next.len() < 5
                        && let Ok(query) =
                            ArticleSupplement::new(ArticleInput::Pmcid(id), first.name.clone())
                    {
                        out.next.push((
                            StepKind::Page,
                            Action::Run {
                                request: Request::ArticleSupplement(query),
                                fresh: false,
                            },
                        ));
                    }
                    Section::Data {
                        source: Source::PmcOa,
                        data: Supplements {
                            license: doc.license,
                            files: files.into(),
                        },
                    }
                }
                Ok(None) => Section::Unavailable {
                    source: Source::PmcOa,
                    reason: biodata::Reason::NotOpen,
                },
                Err(e) => {
                    error.get_or_insert(e);
                    out.local = out.local.or(status::local(e));
                    Section::Unavailable {
                        source: Source::PmcOa,
                        reason: match e {
                            biomcp_fetch::RequestError::Source(e) => e.reason,
                            _ => biodata::Reason::Unavailable,
                        },
                    }
                }
            };
            loaded.push(((id, card.pmid), section.clone()));
            section
        };
        if matches!(section, Section::Unavailable { .. }) {
            item.status = Status::Partial;
        }
        sections.supplements = Some(section);
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    if !loaded.is_empty() {
        out.sources
            .push(status::source(Source::PmcOa, leg.finish(), error));
    }
    out.status = status::status(cards.items.iter().map(|v| v.status));
}
