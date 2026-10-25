//! Pure typed follow-up requests; source names never become chosen drug identities.
use crate::{Action, Outcome};
use biomcp_types::*;
use serde_json::json;
fn run(request:Request) -> Action {Action::Run{request,fresh:false}}
fn gene_query(gene:&biodata::GeneSymbol) -> Option<PgxGet> { serde_json::from_value(json!({"gene":gene,"source":"cpic"})).ok() }
fn section(q:&PgxGet, s:PgxSection) -> Option<Request> { q.with_section(s,PgxSource::Cpic).ok().map(Request::GetPgx) }
pub(super) fn steps(request:&Request, out:&Outcome) -> Vec<(StepKind,Action)> {
    let mut next=vec![];
    if out.status == Status::Failed {return next;}
    match (request,&out.body) {
        (Request::SearchPgx(q),QueryBody::PgxPage(body)) => {
            let offset=body.page.offset.saturating_add(body.page.returned);
            if body.page.returned>0 && u64::from(offset)<body.page.total {next.push((StepKind::Page,run(Request::SearchPgx(q.at_offset(offset)))));}
            if let Some(card)=gene_query(q.gene()) {
                next.push((StepKind::Section,run(Request::GetPgx(card.clone()))));
                if let Some(r)=section(&card,PgxSection::Recommendations) {next.push((StepKind::Section,run(r)));}
            }
        }
        (Request::GetPgx(q), QueryBody::PgxCard(Some(_))) => {
            for s in [PgxSection::Recommendations,PgxSection::Guidelines] {if let Some(r)=section(q,s){next.push((StepKind::Section,run(r)));}}
        }
        (Request::GetPgx(q), QueryBody::PgxRecommendations(PgxRecommendations::View{page,..} | PgxRecommendations::Table{page,..})) => page_step(q,page,&mut next),
        (Request::GetPgx(q),QueryBody::PgxGuidelines(body)) => {
            let mut seen=std::collections::BTreeSet::new();
            for gene in body.items.iter().flat_map(|r| r.genes.iter()) {
                if gene.symbol == *q.gene() || !seen.insert(gene.symbol.clone()) {continue;}
                if let Ok(query)=serde_json::from_value(json!({"input":[gene.symbol]})) {next.push((StepKind::Related,run(Request::GetGene(query))));}
            }
            page_step(q,&body.page,&mut next);
        }
        (Request::GetPgx(q),QueryBody::PgxDiplotype(_)) => {if let Some(card)=gene_query(q.gene()) && let Some(r)=section(&card,PgxSection::Recommendations){next.push((StepKind::Section,run(r)));}}
        _=>{}
    }
    next.truncate(5); next
}
fn page_step(q:&PgxGet,p:&PgxPageInfo,next:&mut Vec<(StepKind,Action)>) {
    let offset=p.offset.saturating_add(p.returned);
    if p.returned>0 && u64::from(offset)<p.total && let Ok(query)=q.at_section_offset(offset){next.push((StepKind::Page,run(Request::GetPgx(query))));}
}
