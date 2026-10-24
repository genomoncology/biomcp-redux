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
        let input = card.pmid.map(ArticleInput::Pmid).or_else(|| card.europepmc_id.clone().map(ArticleInput::EuropePmc));
        let Some(input) = input else { continue; };
        if !item.more.is_empty() {
            let words = item
                .more
                .iter()
                .filter_map(|s| s.to_string().parse().ok())
                .collect();
            if let Ok(query) = ArticleGet::with_sections(vec![input].into(), words)
            {
                push(&mut out.next, StepKind::Section, Request::GetArticle(query));
            }
        }
        let Some(id) = card.pmid else { continue; };
        push(
            &mut out.next,
            StepKind::Disambiguate,
            Request::ArticleCitations(biodata::ArticleEdges::new(id)),
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
