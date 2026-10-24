//! Article section preparation and dispatch, following the gene section pattern.
mod annotations;
mod citations;
use crate::{Ctx, status};
use biodata::{
    ArticleCardItem, ArticleGet, ArticleResolution, ArticleSection, ArticleSectionWord,
    ArticleSections, Section,
};
use biomcp_fetch::Deadline;
pub(super) fn prepare(query: &ArticleGet, items: &mut [ArticleCardItem]) -> bool {
    let requested = !query.sections().is_empty();
    for item in items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        if requested {
            item.sections = Some(ArticleSections {
                annotations: (selected(query, ArticleSectionWord::Annotations)
                    && card.pmid.is_none())
                .then_some(Section::NotApplicable),
                citations: (selected(query, ArticleSectionWord::Citations) && card.pmid.is_none())
                    .then_some(Section::NotApplicable),
            });
            item.more = [
                (ArticleSectionWord::Annotations, ArticleSection::Annotations),
                (ArticleSectionWord::Citations, ArticleSection::Citations),
            ]
            .into_iter()
            .filter(|(word, _)| !selected(query, *word))
            .map(|(_, section)| section)
            .collect();
        } else if card.pmid.is_some() {
            item.more = Box::new([ArticleSection::Annotations, ArticleSection::Citations]);
        }
    }
    requested
}
pub(super) fn run<'a>(
    cx: &Ctx<'a>,
    query: &ArticleGet,
    deadline: &mut Deadline<'a>,
    out: &mut crate::Outcome,
) {
    let biodata::QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let ids = annotations::ids(&cards.items);
    if ids.is_empty() {
        return;
    }
    if selected(query, ArticleSectionWord::Annotations) {
        let result = annotations::run(cx, deadline, &ids);
        annotations::join(&mut cards.items, &result);
        out.local = out.local.or(result.local);
        out.status = status::status(cards.items.iter().map(|i| i.status));
        status::append(&mut out.problems, result.problems);
        out.sources.push(result.source);
    }
    if selected(query, ArticleSectionWord::Citations) {
        citations::run(cx, deadline, &ids, out);
    }
}
fn selected(query: &ArticleGet, word: ArticleSectionWord) -> bool {
    query.sections().contains(&ArticleSectionWord::All) || query.sections().contains(&word)
}
