use crate::{Action, Outcome};
use biodata::{ArticleGet, ArticleInput, ArticleResolution, QueryBody, Request, Status, StepKind};
pub(super) fn get(query: &ArticleGet, out: &mut Outcome) {
    if matches!(out.status, Status::Failed | Status::Partial) {
        out.next.push((StepKind::Fresh, Action::Run { request: Request::GetArticle(query.clone()), fresh: true }));
    }
    let QueryBody::ArticleCards(cards) = &out.body else { return };
    for item in &cards.items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else { continue };
        let Some(id) = card.pmid.filter(|_| !item.more.is_empty()) else { continue };
        let words = item.more.iter().filter_map(|s| s.to_string().parse().ok()).collect();
        if let Ok(query) = ArticleGet::with_sections(vec![ArticleInput::Pmid(id)].into(), words) {
            let step = (StepKind::Section, Action::Run { request: Request::GetArticle(query), fresh: false });
            if out.next.len() < 5 && !out.next.contains(&step) { out.next.push(step); }
        }
    }
}
