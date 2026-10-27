use crate::{Action, Outcome};

use biomcp_types::ArticleInput;
use biomcp_types::{ArticleGet, ArticleResolution, QueryBody, Request, Status, StepKind};
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
                    biomcp_types::Section::NotApplicable
                        | biomcp_types::Section::Unavailable {
                            reason: biomcp_types::Reason::NotOpen,
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
                vec![biomcp_types::ArticleSectionWord::Pdf].into(),
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
            Request::ArticleCitations(biomcp_types::ArticleEdges::new(id)),
        );

        push(
            &mut out.next,
            StepKind::Disambiguate,
            Request::ArticleCitations(
                biomcp_types::ArticleEdges::new(id)
                    .sourced(biomcp_types::EdgeSource::SemanticScholar),
            ),
        );
        push(
            &mut out.next,
            StepKind::Disambiguate,
            Request::ArticleRecommend(biomcp_types::ArticleRecommend::new(ArticleInput::Pmid(id))),
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
pub(super) fn search(
    query: &biomcp_types::ArticleSearch,
    total: u64,
    partial: bool,
    out: &mut Outcome,
) {
    if out.sources.iter().any(|source| {
        source
            .notices
            .iter()
            .any(|notice| matches!(notice, biomcp_types::SourceNotice::NotFoundWords { .. }))
    }) && let Some(quoted) = query.quoted()
    {
        out.next.insert(
            0,
            (
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::SearchArticle(quoted),
                    fresh: false,
                },
            ),
        );
    }
    if !partial && total < u64::from(query.limit()) {
        out.next.extend(query.term_drops().into_iter().map(|query| {
            (
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::SearchArticle(query),
                    fresh: false,
                },
            )
        }));
    }
    variant_step(query, out);
}

fn variant_step(search: &biomcp_types::ArticleSearch, out: &mut Outcome) {
    if search.sources() == [biomcp_types::ArticleSource::Pubmed]
        && search.gene().is_some()
        && search.variant().is_some()
    {
        out.next.push((
            StepKind::Disambiguate,
            crate::Action::Run {
                request: Request::SearchArticle(search.with_litvar()),
                fresh: false,
            },
        ));
    }
}

/// Existing source-given author identities survive ordinary pool paging.
pub(super) fn authors(out: &mut Outcome) {
    let items = match &out.body {
        QueryBody::ArticlePage(page) => page.items.as_ref(),
        QueryBody::ArticleEdges(page) => page.items.as_ref(),
        QueryBody::ArticleRecommendations(page) => page.items.as_ref(),
        _ => return,
    };
    for id in items
        .iter()
        .filter_map(|row| row.header.as_ref())
        .filter_map(|header| header.first_author_id.as_ref())
    {
        let step = (
            StepKind::Pivot,
            Action::Run {
                request: Request::GetAuthor(biomcp_types::AuthorGet::one(
                    biodata::AuthorId::SemanticScholar(id.clone()),
                )),
                fresh: false,
            },
        );
        if !out.next.contains(&step) {
            out.next.push(step);
        }
    }
}

/// Unsearchable bodies still permit an explicit abstract read.
pub(super) fn text(query: &biomcp_types::ArticleText, out: &mut Outcome) {
    if matches!(&out.body, QueryBody::ArticleText(body)
        if !matches!(body.content, biomcp_types::Section::Data { .. }))
        && let Ok(get) = ArticleGet::new(vec![query.input().clone()].into())
    {
        out.next.push((
            StepKind::Disambiguate,
            Action::Run {
                request: Request::GetArticle(get),
                fresh: false,
            },
        ));
    }
}
