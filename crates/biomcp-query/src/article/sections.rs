//! Article section preparation and dispatch, following the gene section pattern.
mod annotations;
pub(crate) mod fulltext;
use crate::{Ctx, status};
use biodata::{
    ArticleCardItem, ArticleGet, ArticleResolution, ArticleSection, ArticleSections, Section,
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
                supplements: (query.sections().iter().any(|s| {
                    matches!(
                        s,
                        biodata::ArticleSectionWord::Supplements | biodata::ArticleSectionWord::All
                    )
                }) && card.pmcid.is_none())
                .then_some(Section::NotApplicable),
                annotations: (query.sections().iter().any(|s| {
                    matches!(
                        s,
                        biodata::ArticleSectionWord::Annotations | biodata::ArticleSectionWord::All
                    )
                }) && card.pmid.is_none())
                .then_some(Section::NotApplicable),
                fulltext: (query.sections().iter().any(|s| {
                    matches!(
                        s,
                        biodata::ArticleSectionWord::Fulltext | biodata::ArticleSectionWord::All
                    )
                }) && card.pmcid.is_none())
                .then_some(Section::NotApplicable),
            });
        } else {
            let mut more = Vec::new();
            if card.pmid.is_some() {
                more.push(ArticleSection::Annotations);
            }
            if card.pmcid.is_some() {
                more.push(ArticleSection::Fulltext);
                more.push(ArticleSection::Supplements);
            }
            item.more = more.into();
        }
    }
    requested
}
pub(super) fn run<'a>(
    cx: &Ctx<'a>,
    deadline: &mut Deadline<'a>,
    query: &ArticleGet,
    out: &mut crate::Outcome,
) {
    if query.sections().iter().any(|s| {
        matches!(
            s,
            biodata::ArticleSectionWord::Annotations | biodata::ArticleSectionWord::All
        )
    }) {
        annotations(cx, deadline, out);
    }
    let fulltext=query.sections().iter().any(|s|matches!(s,biodata::ArticleSectionWord::Fulltext|biodata::ArticleSectionWord::All));
    let supplements=query.sections().iter().any(|s|matches!(s,biodata::ArticleSectionWord::Supplements|biodata::ArticleSectionWord::All));
    if fulltext || supplements {fulltext::run(cx,deadline,out,fulltext,supplements);}

}
fn annotations<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, out: &mut crate::Outcome) {
    let biodata::QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let ids = annotations::ids(&cards.items);
    if ids.is_empty() {
        return;
    }
    let result = annotations::run(cx, deadline, &ids);
    annotations::join(&mut cards.items, &result);
    out.local = out.local.or(result.local);
    out.status = status::status(cards.items.iter().map(|i| i.status));
    status::append(&mut out.problems, result.problems);
    out.sources.push(result.source);
}
