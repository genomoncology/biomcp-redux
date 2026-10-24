//! Article section preparation and dispatch, following the gene section pattern.
mod annotations;
mod citations;
pub(crate) mod fulltext;
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
                supplements: (selected(query, ArticleSectionWord::Supplements)
                    && card.pmcid.is_none())
                .then_some(Section::NotApplicable),
                annotations: (selected(query, ArticleSectionWord::Annotations)
                    && card.pmid.is_none())
                .then_some(Section::NotApplicable),
                citations: (selected(query, ArticleSectionWord::Citations) && card.pmid.is_none())
                    .then_some(Section::NotApplicable),
                fulltext: (selected(query, ArticleSectionWord::Fulltext) && card.pmcid.is_none())
                    .then_some(Section::NotApplicable),
            });
        }
        item.more = [
            (
                ArticleSectionWord::Annotations,
                ArticleSection::Annotations,
                card.pmid.is_some(),
            ),
            (
                ArticleSectionWord::Fulltext,
                ArticleSection::Fulltext,
                card.pmcid.is_some(),
            ),
            (
                ArticleSectionWord::Supplements,
                ArticleSection::Supplements,
                card.pmcid.is_some(),
            ),
            (
                ArticleSectionWord::Citations,
                ArticleSection::Citations,
                card.pmid.is_some(),
            ),
        ]
        .into_iter()
        .filter(|(word, _, available)| *available && !selected(query, *word))
        .map(|(_, section, _)| section)
        .collect();
    }
    requested
}
pub(super) fn run<'a>(
    cx: &Ctx<'a>,
    deadline: &mut Deadline<'a>,
    query: &ArticleGet,
    out: &mut crate::Outcome,
) {
    let fulltext = selected(query, ArticleSectionWord::Fulltext);
    let supplements = selected(query, ArticleSectionWord::Supplements);
    if fulltext || supplements {
        fulltext::run(cx, deadline, out, fulltext, supplements);
    }
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
