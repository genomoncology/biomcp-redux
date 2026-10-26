use super::next;
use crate::{Ctx, Outcome, status};
use biodata::UniProtIsoform;
use biomcp_adapters::{hgnc, uniprot};
use biomcp_fetch::{Find, KeyKind, Leg, RequestError};
use biomcp_types::{ProteinCard, ProteinCardItem, ProteinGet, ProteinInput, QueryBody, Source, Status};
/// Exact accessions resolve independently and remain in caller order.
pub fn get(cx: &Ctx<'_>, q: &ProteinGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let hgnc_leg = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(cx.fresh);
    let leg = cx.fetch.leg(&uniprot::SPEC, deadline).fresh(cx.fresh);
    let mut out = super::empty(QueryBody::ProteinCard(ProteinCard { items: Box::default() }));
    let mut hgnc_error = None;
    let mut protein_error = None;
    let mut items = Vec::new();
    for input in q.input() {
        let result = ids(&hgnc_leg, input, &mut out);
        if let Err(e) = result { hgnc_error = Some(e); out.local = out.local.or(status::local(e)); }
        let mut item = empty_item(input.clone());
        if let Ok(ids) = result {
            hydrate(&leg, ids, &mut item, &mut out, &mut protein_error);
        }
        items.push(item);
    }
    if q.input().iter().any(|v| matches!(v, ProteinInput::Gene(_))) {
        out.sources.push(status::source(Source::Hgnc, hgnc_leg.finish(), hgnc_error));
    }
    out.sources.push(status::source(Source::UniProt, leg.finish(), protein_error));
    if out.sources.iter().any(|s| s.state == biomcp_types::SourceState::Stale) {
        for item in &mut items { if item.status == Status::Complete { item.status = Status::Partial; } }
    }
    out.status = status::status(items.iter().map(|i| i.status));
    out.next = next::cards(&items);
    out.body = QueryBody::ProteinCard(ProteinCard { items: items.into() });
    command.finish(deadline.remaining());
    out
}
fn empty_item(input: ProteinInput) -> ProteinCardItem {
    ProteinCardItem { input, status: Status::Failed, header: None,
        candidates: Box::default(), audit: None, sections: None, more: Box::default() }
}
fn ids(leg: &Leg<'_>, input: &ProteinInput, out: &mut Outcome) -> Result<Vec<UniProtIsoform>, RequestError> {
    let ProteinInput::Gene(symbol) = input else {
        return Ok(match input { ProteinInput::Accession(id) => vec![id.clone()], _ => Vec::new() });
    };
    let rows = hgnc::lookup(leg, Find::Exact(KeyKind::Symbol, symbol.as_str()))?;
    status::append(&mut out.problems, rows.problems);
    let mut ids = Vec::new();
    for gene in rows.value.into_iter().filter(|g| g.symbol.as_str() == symbol.as_str()) {
        for id in gene.uniprot {
            if let Ok(id) = UniProtIsoform::new(id.as_str()) && !ids.contains(&id) { ids.push(id); }
        }
    }
    Ok(ids)
}
fn hydrate(leg: &Leg<'_>, ids: Vec<UniProtIsoform>, item: &mut ProteinCardItem,
    out: &mut Outcome, error: &mut Option<RequestError>) {
    let ambiguous = ids.len() > 1;
    item.status = Status::Absent;
    let mut failed = false;
    for id in ids {
        match uniprot::entry(leg, &id) {
            Ok(mapped) => {
                failed |= !mapped.problems.is_empty();
                status::append(&mut out.problems, mapped.problems);
                if let Some(entry) = mapped.value {
                    item.status = Status::Complete;
                    if ambiguous {
                        let mut rows = item.candidates.to_vec(); rows.push(entry.header); item.candidates = rows.into();
                    } else { item.header = Some(entry.header); item.audit = Some(entry.audit); }
                }
            }
            Err(e) => { failed = true; *error = Some(e); out.local = out.local.or(status::local(e)); }
        }
    }
    if failed { item.status = if item.header.is_some() || !item.candidates.is_empty() { Status::Partial } else { Status::Failed }; }
}
