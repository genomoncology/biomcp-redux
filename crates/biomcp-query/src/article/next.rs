use crate::{Action, Outcome};
use biodata::{ArticleGet, ArticleInput, ArticleResolution, QueryBody, Request, Status, StepKind};
pub(super) fn get(query: &ArticleGet, out: &mut Outcome) {
    if matches!(out.status, Status::Failed | Status::Partial) {
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request: Request::GetArticle(query.clone()),
                fresh: true,
            },
        ));
    }
    let QueryBody::ArticleCards(cards) = &out.body else {
        return;
    };
    for item in &cards.items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        if item.sections.as_ref().is_some_and(|s| {
            matches!(
                s.fulltext,
                Some(
                    biodata::Section::NotApplicable
                        | biodata::Section::Unavailable {
                            reason: biodata::Reason::NotOpen,
                            ..
                        }
                )
            )
        }) && let Some(input) = card
            .pmid
            .map(ArticleInput::Pmid)
            .or_else(|| card.doi.clone().map(ArticleInput::Doi))
            && let Ok(query) = ArticleGet::with_sections(
                vec![input].into(),
                vec![biodata::ArticleSectionWord::Pdf].into(),
            )
        {
            push(&mut out.next, StepKind::Section, Request::GetArticle(query));
        }
        let input = card
            .pmid
            .map(ArticleInput::Pmid)
            .or_else(|| card.europepmc_id.clone().map(ArticleInput::EuropePmc));
        let Some(input) = input else {
            continue;
        };
        if !item.more.is_empty() {
            let words = item
                .more
                .iter()
                .filter_map(|s| s.to_string().parse().ok())
                .collect();
            if let Ok(query) = ArticleGet::with_sections(vec![input].into(), words) {
                push(&mut out.next, StepKind::Section, Request::GetArticle(query));
            }
        }
        let Some(id) = card.pmid else {
            continue;
        };
        push(
            &mut out.next,
            StepKind::Disambiguate,
            Request::ArticleCitations(biodata::ArticleEdges::new(id)),
        );

        push(
            &mut out.next,
            StepKind::Disambiguate,
            Request::ArticleCitations(
                biodata::ArticleEdges::new(id).sourced(biodata::EdgeSource::SemanticScholar),
            ),
        );
        push(
            &mut out.next,
            StepKind::Disambiguate,
            Request::ArticleRecommend(biodata::ArticleRecommend::new(ArticleInput::Pmid(id))),
        );
    }
}
fn push(next: &mut Vec<(StepKind, Action)>, kind: StepKind, request: Request) {
    let step = (
        kind,
        Action::Run {
            request,
            fresh: false,
        },
    );
    if next.len() < 5 && !next.contains(&step) {
        next.push(step);
    }
}

/// A short complete pool offers caller-controlled alternatives without running them.
pub(super) fn term_drops(query: &biodata::ArticleSearch, total: u64, partial: bool, out: &mut Outcome) {
    if !partial && total < u64::from(query.limit()) {
        out.next.extend(query.term_drops().into_iter().map(|query| (
            StepKind::Disambiguate,
            Action::Run { request: Request::SearchArticle(query), fresh: false },
        )));
    }
}
