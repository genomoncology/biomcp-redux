//! Resolve complete source names before the unchanged native-ID path.
use super::name_pages;
use crate::drug::aliases::name_evidence::recovery;
use crate::drug::{fork, label_choice, rejected_input, selected_parts};
use crate::{Ctx, Outcome, status};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugCard, DrugGet, DrugHeader, DrugNameMatch, DrugNameResolution, DrugSearch, DrugSource,
    ReplyData, Request, SourceState, Status, Step, StepKind, UsageArguments, UsageError,
};
use std::cmp::Reverse;

/// Name match, source tier, a label's original packager before repackagers, more numbered label
/// indications first, then a source key.
pub(in crate::drug) type Rank = (u8, u8, u8, Reverse<u32>, String);
pub(in crate::drug) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    get_with_name_state(cx, query).0
}

/// Also report whether MyChem stopped reading before it settled the name.
pub(in crate::drug) fn get_with_name_state(cx: &Ctx<'_>, query: &DrugGet) -> (Outcome, bool) {
    let mut unsettled = false;
    let mut out = resolve_parts(cx, std::slice::from_ref(query), &mut unsettled);
    let out = out
        .pop()
        .unwrap_or_else(|| status::empty(ReplyData::Usage(UsageError::argument("input".into()))));
    (out, unsettled)
}

/// Resolve the name of the first query once and get every query for the chosen record, so
/// label headings of one name share one record read. A name that resolves to no record
/// answers with one outcome for all.
pub(in crate::drug) fn get_parts(cx: &Ctx<'_>, queries: &[DrugGet]) -> Vec<Outcome> {
    resolve_parts(cx, queries, &mut false)
}

fn resolve_parts(cx: &Ctx<'_>, queries: &[DrugGet], unsettled: &mut bool) -> Vec<Outcome> {
    let Some(query) = queries.first() else {
        return Vec::new();
    };
    let deadline = lookup_deadline(cx);
    let name = query.input().to_string();
    let search = match DrugSearch::named(query.input().clone(), query.source(), 50) {
        Ok(search) => search,
        Err(_) => {
            return vec![status::empty(ReplyData::Usage(UsageError::argument(
                "input".into(),
            )))];
        }
    };
    let (mut discovery, mut candidates) = if query.source() == DrugSource::Drugsfda {
        name_pages::fda(cx, query, &search)
    } else if query.source() == DrugSource::OpenfdaLabel {
        name_pages::label(cx, query, &search)
    } else if query.source() == DrugSource::Mychem {
        let (out, candidates, stopped) = name_pages::mychem(cx, query, &search);
        *unsettled = stopped;
        (out, candidates)
    } else if query.source() == DrugSource::Chembl {
        name_pages::chembl(cx, query, &search)
    } else if query.source() == DrugSource::Ema {
        name_pages::ema(cx, query, &search)
    } else {
        name_pages::discover(cx, &search)
    };
    if unanswered(&discovery, candidates.is_empty()) {
        return queries
            .iter()
            .map(|query| failed_name(&discovery, query))
            .collect();
    }
    candidates.sort_by(|a, b| a.0.cmp(&b.0));
    let mut seen = std::collections::BTreeSet::new();
    candidates.retain(|(_, header)| seen.insert(header.id.to_string()));
    let Some((rank, selected)) = candidates.first() else {
        discovery.body = ReplyData::Usage(UsageError::Arguments(UsageArguments {
            argument: Some("input".into()),
            next: vec![Step {
                kind: StepKind::Disambiguate,
                args: vec![
                    "search".into(),
                    "drug".into(),
                    name.into(),
                    "--source".into(),
                    query.source().as_str().into(),
                ]
                .into(),
                options: Default::default(),
            }],
            ..Default::default()
        }));
        return vec![discovery];
    };
    let resolved = match queries
        .iter()
        .map(|query| query.with_native_id(selected.id.clone()))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(resolved) => resolved,
        Err(_) => return vec![rejected_input(query.source(), "input", discovery)],
    };
    selected_parts(&within(cx, deadline.remaining()), &resolved)
        .into_iter()
        .zip(queries)
        .map(|(out, query)| {
            let mut out = settled(
                chosen_name(query, rank.0, &candidates, &discovery, out),
                *unsettled,
            );
            if query.source() == DrugSource::Chembl
                && out.sources.iter().any(|s| s.state == SourceState::Stale)
            {
                recovery(&mut out, Request::GetDrug(query.clone()));
            }
            out
        })
        .collect()
}

/// A failed discovery keeps the caller's requested section and paging.
fn failed_name(discovery: &Outcome, query: &DrugGet) -> Outcome {
    let mut out = fork(discovery, DrugCard::into_reply(query.clone(), None));
    status::item(&mut out, false, true, false);
    recovery(&mut out, Request::GetDrug(query.clone()));
    out
}

/// Whether the name lookup did not answer. Stale rows answer only with a candidate, whose read
/// says they are stale; with none, the name may be missing only from the stale copy (1081).
fn unanswered(discovery: &Outcome, none: bool) -> bool {
    let unread = |s: &biomcp_types::SourceOutcome| {
        s.state != SourceState::Ok && (none || s.state != SourceState::Stale)
    };
    discovery.dropped || discovery.sources.iter().any(unread)
}

/// A product-only answer, while later matches may hold the drug itself, or a record chosen
/// from stale stored rows, is partial.
fn settled(mut out: Outcome, unsettled: bool) -> Outcome {
    let stale = out.sources.iter().any(|s| s.state == SourceState::Stale);
    if (unsettled || stale) && out.status() == Status::Complete {
        status::item(&mut out, true, false, true);
    }
    out
}

fn chosen_name(
    query: &DrugGet,
    rank: u8,
    candidates: &[(Rank, DrugHeader)],
    discovery: &Outcome,
    mut out: Outcome,
) -> Outcome {
    let ranking = (if query.source() == DrugSource::Drugsfda {
        "exact brand before generic; NDA/BLA before ANDA, then earliest approval"
    } else if query.source() == DrugSource::OpenfdaLabel {
        label_choice::RANKING
    } else if query.source() == DrugSource::Ema {
        "exact brand before generic, then earliest marketing authorisation; source search order among equals"
    } else {
        "exact brand before generic; source search order among equals"
    })
    .into();
    let resolution = DrugNameResolution {
        input: query.input().clone(),
        basis: if rank == 0 {
            DrugNameMatch::Brand
        } else {
            DrugNameMatch::Generic
        },
        ranking,
        total: candidates.len() as u64,
        reason: (query.source() == DrugSource::OpenfdaLabel)
            .then(|| label_choice::reason(candidates))
            .flatten(),
        alternatives: candidates
            .iter()
            .skip(1)
            .take(5)
            .map(|(_, h)| Step {
                kind: StepKind::Disambiguate,
                args: vec![
                    "get".into(),
                    "drug".into(),
                    h.id.to_string().into(),
                    "--source".into(),
                    query.source().as_str().into(),
                ]
                .into(),
                options: Default::default(),
            })
            .collect(),
    };
    annotate(&mut out.body, resolution);
    carry(discovery, &mut out);
    out
}

/// Keep the lookup's problems and source reads on the record it chose.
pub(in crate::drug) fn carry(discovery: &Outcome, out: &mut Outcome) {
    out.problems.extend(discovery.problems.iter().cloned());
    for source in &discovery.sources {
        if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source.source) {
            status::retain_source_state(existing, source);
        } else {
            out.sources.push(source.clone());
        }
    }
}

fn annotate(body: &mut ReplyData, resolution: DrugNameResolution) {
    let header = match body {
        ReplyData::DrugCard {
            body: Some(body), ..
        } => &mut body.header,
        ReplyData::DrugApprovals {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugLabel {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugRegulatory {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugTargets {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugIndications {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugCivic {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugInteractions {
            body: Some(body), ..
        } => &mut body.identity,
        ReplyData::DrugXrefs {
            body: Some(body), ..
        } => &mut body.identity,
        _ => return,
    };
    header.name_resolution = Some(resolution);
}

pub(in crate::drug) fn within<'a>(cx: &Ctx<'a>, budget: std::time::Duration) -> Ctx<'a> {
    Ctx {
        fetch: cx.fetch,
        // Drug source credentials remain in the borrowed Fetch; no drug source contacts NCBI.
        ncbi_email: None,
        budget,
        fresh: cx.fresh,
        drug_name_limit: cx.drug_name_limit,
        label_bytes: cx.label_bytes,
        fit_deadline: cx.fit_deadline,
    }
}

/// Track the time spent by nested source commands, which advance their own replay clocks.
#[expect(
    clippy::disallowed_methods,
    reason = "Times nested commands, which charge their own command clocks; it opens no leg (1038)"
)]
pub(in crate::drug) fn lookup_deadline<'a>(cx: &Ctx<'a>) -> biomcp_fetch::Deadline<'a> {
    cx.fetch.deadline(cx.budget)
}
