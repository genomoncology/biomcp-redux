//! Page sources that expose name candidates through their ordinary search.
use super::{NativePage, Rank, lookup_deadline, native_names, plain, within};
use crate::{Ctx, Outcome};
use biomcp_types::{DrugGet, DrugHeader, DrugNameMatch, DrugSearch, ReplyData, StepKind};

pub(in crate::drug) fn discover(
    cx: &Ctx<'_>,
    search: &DrugSearch,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let deadline = lookup_deadline(cx);
    let mut query = search.clone();
    let mut out = super::super::super::search(&within(cx, deadline.remaining()), &query);
    let mut candidates = Vec::new();
    let mut incomplete = false;
    let mut problems = Vec::new();
    loop {
        incomplete |= out.dropped;
        problems.append(&mut out.problems);
        if let ReplyData::DrugPage {
            body: Some(page), ..
        } = &out.body
        {
            for header in &page.items {
                let rank = match header.matched_name {
                    Some(DrugNameMatch::Brand | DrugNameMatch::Both) => Some(0),
                    Some(DrugNameMatch::Generic) => Some(1),
                    _ if header.name.as_ref().is_some_and(|n| {
                        search.query().is_some_and(|q| {
                            biomcp_types::DrugNameMatch::generic_name(n.as_str(), q.as_str())
                        })
                    }) =>
                    {
                        Some(1)
                    }
                    _ => None,
                };
                if let Some(rank) = rank {
                    candidates.push((plain(rank, 0, String::new()), header.clone()));
                }
            }
        }
        let Some(next_query) = out.next.iter().find_map(|step| match &step.1 {
            crate::Action::Run {
                request: biomcp_types::Request::SearchDrug(next),
                ..
            } if step.0 == StepKind::Page => Some(next.clone()),
            _ => None,
        }) else {
            break;
        };
        query = next_query;
        let next = super::super::super::search(&within(cx, deadline.remaining()), &query);
        if next
            .sources
            .iter()
            .any(|s| s.state != biomcp_types::SourceState::Ok)
        {
            out = next;
            break;
        }
        out = next;
    }
    out.dropped |= incomplete;
    out.problems.extend(problems);
    (out, candidates)
}

pub(in crate::drug) fn ema(
    cx: &Ctx<'_>,
    get: &DrugGet,
    query: &DrugSearch,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    native_names(cx, query, &biomcp_adapters::ema::SPEC, |leg, _| {
        let rows = biomcp_adapters::ema::search_names(leg, get.input().as_str())?;
        let candidates = rows
            .value
            .into_iter()
            .filter_map(|(header, authorised)| {
                header.matched_name.map(|matched| {
                    let rank = u8::from(matched == DrugNameMatch::Generic);
                    let date = authorised.map_or_else(|| "9999".into(), |date| date.to_string());
                    (plain(rank, 0, date), header)
                })
            })
            .collect();
        Ok(NativePage {
            read: None,
            candidates,
            next_offset: None,
            problems: rows.problems,
            refusals: rows.refusals,
        })
    })
}
