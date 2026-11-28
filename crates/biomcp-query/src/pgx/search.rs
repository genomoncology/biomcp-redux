//! PGx discovery keeps CPIC pairs and ClinPGx annotations in separate replies.
use super::{finish, next, page_info};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{clinpgx, cpic};
use biomcp_fetch::Leg;
use biomcp_types::{
    PgxDiscovery, PgxHeader, PgxPage, PgxSearch, PgxSearchHeader, PgxSource, PgxTotalKind,
    ReplyBody, Request,
};

pub(crate) fn search(cx: &Ctx<'_>, query: &PgxSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let source = query.source();
    let spec = match source {
        PgxSource::Cpic => &cpic::SPEC,
        PgxSource::Clinpgx => &clinpgx::SPEC,
    };
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let (mut out, consumed) = match source {
        PgxSource::Cpic => pairs(leg, query),
        PgxSource::Clinpgx => clinical(leg, query),
    };
    out.next = next::steps_with(&Request::SearchPgx(query.clone()), &mut out, Some(consumed));
    command.finish(deadline.remaining());
    out
}

fn header(query: &PgxSearch) -> Option<PgxSearchHeader> {
    let source = query.source();
    if let Some(drug) = query.drug() {
        Some(PgxSearchHeader::Drug { drug: drug.clone(), source })
    } else {
        query.variant().map(|variant| PgxSearchHeader::Variant { variant: *variant, source })
    }
}

fn pairs(leg: Leg<'_>, query: &PgxSearch) -> (Outcome, u32) {
    let gene = query.gene().zip(query.gene_context());
    let header = header(query);
    let body = |items: Vec<_>, total| {
        let page = page_info(query.offset(), items.len(), total);
        let items = items.into();
        if let Some((symbol, context)) = &gene {
            PgxPage {
                header: PgxHeader { gene: (*symbol).clone(), source: query.source() },
                page,
                items,
            }.with_query(context.clone())
        } else {
            PgxDiscovery::into_reply(query.clone(), header.clone().map(|header| {
                PgxDiscovery::Pairs { header, page, items }
            }))
        }
    };
    let mut consumed = 0;
    let mut problems = Vec::new();
    let mut refusals = biomcp_adapters::Refusal::None;
    let reply = cpic::search(&leg, query).map(|page| {
        refusals = page.refusals;
        problems = page.problems;
        consumed = page.consumed;
        body(page.items, page.total)
    });
    let mut out = finish(leg, reply, body(vec![], 0), query.source().source());
    out.include_refusals(refusals);
    status::append(&mut out.problems, problems);
    (out, consumed)
}

fn clinical(leg: Leg<'_>, query: &PgxSearch) -> (Outcome, u32) {
    let header = header(query);
    let body = |items: Vec<_>, total| {
        let mut page = page_info(query.offset(), items.len(), total);
        page.total_kind = PgxTotalKind::Observed;
        PgxDiscovery::into_reply(query.clone(), header.clone().map(|header| {
            PgxDiscovery::Clinical { header, page, items: items.into() }
        }))
    };
    let mut consumed = 0;
    let mut problems = Vec::new();
    let mut refusals = biomcp_adapters::Refusal::None;
    let reply = clinpgx::search(&leg, query).map(|rows| {
        refusals = rows.refusals;
        problems = rows.problems;
        let total = rows.value.len() as u64;
        let items: Vec<_> = rows.value.into_iter()
            .skip(query.offset() as usize).take(query.limit() as usize).collect();
        consumed = items.len() as u32;
        body(items, total)
    });
    let mut out = finish(leg, reply, body(vec![], 0), query.source().source());
    out.include_refusals(refusals);
    status::append(&mut out.problems, problems);
    (out, consumed)
}
