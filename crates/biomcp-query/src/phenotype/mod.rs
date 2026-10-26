//! Literal source term discovery and independent exact term cards.
mod card;
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::monarch;
use biomcp_types::{PageInfo, PhenotypeGet, PhenotypePage, PhenotypeSearch, QueryBody, Request, Source, Status, StepKind};
pub use card::get;
/// Search one native HP page without source substitution or local selection.
pub fn search(cx: &Ctx<'_>, q: &PhenotypeSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&monarch::SPEC, deadline).fresh(cx.fresh);
    let reply = monarch::phenotype::search(&leg,q);
    let error = reply.as_ref().err().copied();
    let mut out = empty(QueryBody::PhenotypePage(PhenotypePage { items:Box::default(),components:Box::default(),fetched:None }));
    out.sources.push(status::source(Source::Monarch,leg.finish(),error));
    out.local = error.and_then(status::local);
    if let Ok(page) = reply {
        let returned = page.items.len() as u32;
        for item in &page.items {
            out.next.push((StepKind::Disambiguate,run(Request::GetPhenotype(PhenotypeGet::new(item.id.clone())))));
        }
        if returned > 0 && u64::from(q.offset()) + u64::from(returned) < page.total {
            out.next.push((StepKind::Page,run(Request::SearchPhenotype(q.at_offset(q.offset().saturating_add(returned))))));
        }
        out.page = Some(PageInfo { offset:q.offset(),returned,total:page.total,pool:None,truncated:None });
        out.body = QueryBody::PhenotypePage(PhenotypePage {items:page.items.into(),components:monarch::components().unwrap_or_default(),fetched:u64::try_from(page.metadata.fetched).ok().filter(|v|*v>0)});
        out.status = overall(&out,page.total==0);
    }
    problem(&mut out,Source::Monarch,None);
    command.finish(deadline.remaining());
    out
}
fn empty(body: QueryBody) -> Outcome {
    Outcome { status:Status::Failed,body,page:None,sources:Vec::new(),problems:Vec::new(),next:Vec::new(),local:None }
}
fn run(request: Request) -> Action { Action::Run {request,fresh:false} }
fn overall(out: &Outcome, absent: bool) -> Status {
    if out.sources.iter().any(|s|s.state==biomcp_types::SourceState::Stale) {Status::Partial}
    else if absent {Status::Absent} else {Status::Complete}
}
fn problem(out: &mut Outcome, source: Source, record: Option<biodata::Curie>) {
    if out.sources.iter().any(|s|s.reason==Some(biomcp_types::Reason::BadReply)) {
        out.problems.push(biomcp_types::Problem {source,record,field:"record".into(),code:biomcp_types::ProblemCode::ValueUnknown});
    }
}
