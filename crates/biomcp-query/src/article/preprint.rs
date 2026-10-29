//! Europe PMC owns preprint cards and document fetching; section dispatch stays shared.
use crate::{Ctx, Outcome, status};

use biomcp_adapters::europepmc;
use biomcp_fetch::Deadline;
use biomcp_types::ArticleInput;
use biomcp_types::{
    ArticleMatch, ArticleResolution, QueryBody, Section, Source, SourceState, Status,
};
pub(super) fn cards<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, out: &mut Outcome) {
    let QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let leg = cx.fetch.leg(&europepmc::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut error = None;
    let mut used = false;
    for item in &mut cards.items {
        let ArticleInput::EuropePmc(id) = &item.input else {
            continue;
        };
        used = true;
        match europepmc::core(&leg, id) {
            Ok((card, metadata)) => {
                out.problems.extend(card.problems);
                item.status = Status::Complete;
                item.answer = Some(ArticleResolution::Resolved {
                    card: card.value.card(),
                    r#match: ArticleMatch {
                        kind: item.input.kind(),
                    },
                    fetched: metadata.fetched.and_then(|value| u64::try_from(value).ok()),
                });
            }
            Err(e) => {
                error.get_or_insert(e);
                out.local = out.local.or(status::local(e));
                item.status = Status::Failed;
                item.answer = None;
            }
        }
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::EuropePmc, leg.finish(), error);
    if used {
        if source.state == SourceState::Stale {
            for item in &mut cards.items {
                if matches!(item.input, ArticleInput::EuropePmc(_))
                    && item.status == Status::Complete
                {
                    item.status = Status::Partial;
                }
            }
        }
        out.sources.push(source);
        out.status = status::status(cards.items.iter().map(|i| i.status));
    }
}
pub(super) fn fulltext<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, out: &mut Outcome) {
    let QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let leg = cx.fetch.leg(&europepmc::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut error = None;
    let mut used = false;
    for item in &mut cards.items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        let Some(id) = &card.europepmc_id else {
            continue;
        };
        let Some(sections) = &mut item.sections else {
            continue;
        };
        used = true;
        sections.fulltext = Some(match europepmc::read(&leg, id) {
            Ok(document) => {
                if !document.body.parts.is_empty() {
                    super::sections::fulltext::next_input(
                        ArticleInput::EuropePmc(id.clone()),
                        1,
                        &mut out.next,
                    );
                }
                Section::Data {
                    source: Source::EuropePmc,
                    data: super::sections::fulltext::outline(&document),
                }
            }
            Err(e) => {
                error.get_or_insert(e);
                out.local = out.local.or(status::local(e));
                item.status = Status::Partial;
                Section::unavailable(
                    Source::EuropePmc,
                    crate::status::reason(e).unwrap_or(biomcp_types::Reason::Unavailable),
                )
            }
        });
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::EuropePmc, leg.finish(), error);
    if used {
        if source.state == SourceState::Stale {
            for item in &mut cards.items {
                if matches!(item.input, ArticleInput::EuropePmc(_))
                    && item.status == Status::Complete
                {
                    item.status = Status::Partial;
                }
            }
        }
        if let Some(previous) = out
            .sources
            .iter_mut()
            .find(|s| s.source == Source::EuropePmc)
        {
            *previous = source;
        } else {
            out.sources.push(source);
        }
        out.status = status::status(cards.items.iter().map(|i| i.status));
    }
}
