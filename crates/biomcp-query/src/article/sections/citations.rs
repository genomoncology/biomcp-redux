use crate::{Ctx, Outcome, status};
use biodata::{ArticleCardItem, ArticleResolution, Pmid, QueryBody, Reason, Section, Source, SourceState, Status};
use biomcp_fetch::Deadline;
use super::super::order;
pub(super) fn run<'a>(cx:&Ctx<'a>, deadline:&mut Deadline<'a>, ids:&[Pmid], out:&mut Outcome) {
    let result=order::fetch(cx,deadline,ids);
    let QueryBody::ArticleCards(cards)=&mut out.body else {return;};
    for item in &mut cards.items {
        join(item,&result);
    }
    out.local=out.local.or(result.local);
    out.sources.push(result.source);
    out.status=status::status(cards.items.iter().map(|i|i.status));
}
fn join(item:&mut ArticleCardItem,result:&order::Counts) {
    let Some(ArticleResolution::Resolved {card,..})=&item.answer else {return;};
    let Some(id)=card.pmid else {return;};
    let section=if let Some(row)=result.rows.iter().find(|r|r.pmid==id) {
        Section::Data {source:Source::Icite,data:row.counts.clone()}
    } else if result.error.is_some() {
        Section::Unavailable {source:Source::Icite,reason:result.source.reason.unwrap_or(Reason::Unavailable)}
    } else {Section::Empty {source:Source::Icite,record:None}};
    if matches!(section,Section::Unavailable {..}) || result.source.state==SourceState::Stale {item.status=Status::Partial;}
    if let Some(sections)=&mut item.sections {sections.citations=Some(section);}
}
