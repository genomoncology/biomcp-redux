//! Resolve complete source names before the unchanged native-ID path.
use crate::{Ctx, Outcome, status};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugGet, DrugHeader, DrugNameMatch, DrugNameResolution, DrugSearch, DrugSource, ReplyData,
    Step, StepKind, UsageArguments, UsageError,
};

type Rank = (u8, u8, String);

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let name = query.input().to_string();
    let search: DrugSearch = match serde_json::from_value(
        serde_json::json!({"query": name, "source": query.source(), "limit": 50}),
    ) {
        Ok(search) => search,
        Err(_) => return status::empty(ReplyData::Usage(UsageError::arguments())),
    };
    let (mut discovery, mut candidates) = if query.source() == DrugSource::Drugsfda {
        fda(cx, query, &search)
    } else if query.source() == DrugSource::OpenfdaLabel {
        label(cx, query, &search)
    } else if query.source() == DrugSource::Mychem {
        mychem(cx, query, &search)
    } else if query.source() == DrugSource::Chembl {
        chembl(cx, query, &search)
    } else {
        discover(cx, &search)
    };
    if discovery
        .sources
        .iter()
        .any(|s| s.state != biomcp_types::SourceState::Ok)
    {
        return discovery;
    }
    candidates.sort_by(|a, b| a.0.cmp(&b.0));
    let mut seen = std::collections::BTreeSet::new();
    candidates.retain(|(_, header)| seen.insert(header.id.to_string()));
    let Some((_, selected)) = candidates.first() else {
        if discovery
            .sources
            .iter()
            .any(|s| s.state != biomcp_types::SourceState::Ok)
        {
            return discovery;
        }
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
            }],
            ..Default::default()
        }));
        return discovery;
    };
    let Ok(resolved) = query.with_native_id(selected.id.clone()) else {
        return discovery;
    };
    let mut out = super::selected_get(cx, &resolved);
    let Ok(ranking) = (if query.source() == DrugSource::Drugsfda {
        "exact brand before generic; NDA/BLA before ANDA, then earliest approval"
    } else {
        "exact brand before generic; source search order among equals"
    })
    .parse() else {
        return discovery;
    };
    let resolution = DrugNameResolution {
        input: query.input().clone(),
        ranking,
        total: candidates.len() as u64,
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
            })
            .collect(),
    };
    annotate(&mut out.body, resolution);
    out.problems.extend(discovery.problems);
    for source in discovery.sources {
        if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source.source) {
            status::retain_source_state(existing, &source);
        } else {
            out.sources.push(source);
        }
    }
    out
}

fn discover(cx: &Ctx<'_>, search: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let mut query = search.clone();
    let mut out = super::search(cx, &query);
    let mut candidates = Vec::new();
    loop {
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
                            biomcp_types::DrugNameMatch::same_name(n.as_str(), q.as_str())
                        })
                    }) =>
                    {
                        Some(0)
                    }
                    _ => None,
                };
                if let Some(rank) = rank {
                    candidates.push(((rank, 0, String::new()), header.clone()));
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
        let next = super::search(cx, &query);
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
    (out, candidates)
}

struct NativePage {
    candidates: Vec<(Rank, DrugHeader)>,
    next_offset: Option<u32>,
    problems: Vec<biomcp_types::Problem>,
    refusals: biomcp_adapters::Refusal,
}
fn native_names(
    cx: &Ctx<'_>,
    query: &DrugSearch,
    spec: &biomcp_fetch::SourceSpec,
    read: impl Fn(&biomcp_fetch::Leg<'_>, u32) -> Result<NativePage, biomcp_fetch::RequestError>,
) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let mut out = status::empty(biomcp_types::DrugPage::into_reply(query.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let mut candidates = Vec::new();
    let mut offset = 0;
    let mut error = None;
    loop {
        match read(&leg, offset) {
            Ok(page) => {
                out.include_refusals(page.refusals);
                out.problems.extend(page.problems);
                candidates.extend(page.candidates);
                let Some(next) = page.next_offset else {
                    break;
                };
                if next <= offset {
                    error = Some(biomcp_fetch::InputError::Serialization.into());
                    break;
                }
                offset = next;
            }
            Err(failure) => {
                error = Some(failure);
                break;
            }
        }
    }
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(query.source().source(), leg.finish(), error));
    status::search(&mut out);
    command.finish(deadline.remaining());
    (out, candidates)
}
fn fda(cx: &Ctx<'_>, get: &DrugGet, query: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    native_names(
        cx,
        query,
        &biomcp_adapters::drugsfda::SPEC,
        |leg, offset| {
            let page = biomcp_adapters::drugsfda::search(leg, get.input(), 50, offset)?;
            let next = offset + page.records.len() as u32;
            Ok(NativePage {
                candidates: page
                    .records
                    .iter()
                    .filter_map(|row| {
                        row.name_rank(get.input().as_str())
                            .map(|(name, app, date)| {
                                (
                                    (name, app, date.unwrap_or_else(|| "9999".into())),
                                    row.header(),
                                )
                            })
                    })
                    .collect(),
                next_offset: (next > offset && u64::from(next) < page.total).then_some(next),
                problems: page.problems,
                refusals: page.refusals,
            })
        },
    )
}
fn label(cx: &Ctx<'_>, get: &DrugGet, query: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    native_names(
        cx,
        query,
        &biomcp_adapters::openfda_label::SPEC,
        |leg, offset| {
            let page = biomcp_adapters::openfda_label::search(leg, get.input(), 50, offset).page?;
            let next_offset = page.window.next_offset();
            let mut candidates = Vec::new();
            for row in &page.window.rows {
                if let Some(matched) = row.exact_name_match(get.input()) {
                    candidates.push((
                        (
                            u8::from(matched == DrugNameMatch::Generic),
                            0,
                            String::new(),
                        ),
                        row.header()?,
                    ));
                }
            }
            Ok(NativePage {
                candidates,
                next_offset,
                problems: page.problems,
                refusals: page.window.refusals(),
            })
        },
    )
}

fn mychem(cx: &Ctx<'_>, get: &DrugGet, query: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    native_names(cx, query, &biomcp_adapters::mychem::SPEC, |leg, offset| {
        let (page, received) =
            biomcp_adapters::mychem::search_names(leg, get.input().as_str(), 50, offset)?;
        let next = offset + received;
        Ok(NativePage {
            candidates: page
                .value
                .items
                .iter()
                .filter_map(|header| {
                    header.matched_name.map(|matched| {
                        (
                            (
                                u8::from(matched == DrugNameMatch::Generic),
                                0,
                                String::new(),
                            ),
                            header.clone(),
                        )
                    })
                })
                .collect(),
            next_offset: (received > 0
                && page
                    .value
                    .total
                    .is_some_and(|total| u64::from(next) < total))
            .then_some(next),
            problems: page.problems,
            refusals: page.refusals,
        })
    })
}
fn chembl(cx: &Ctx<'_>, get: &DrugGet, query: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let mut all = Vec::new();
    let mut outcome = None;
    for selector in [
        biomcp_types::DrugNameSelector::Preferred,
        biomcp_types::DrugNameSelector::Synonym,
    ] {
        let (out, candidates) =
            native_names(cx, query, &biomcp_adapters::chembl::SPEC, |leg, offset| {
                let (page, _) = biomcp_adapters::chembl::search(
                    leg,
                    get.input().as_str(),
                    selector,
                    50,
                    offset,
                )?;
                let next_offset = page.window.next_offset();
                let refusals = page.window.refusals();
                let mut problems = page.problems;
                let mut candidates = Vec::new();
                for row in &page.window.rows {
                    if let Some(matched) = row.name_match(get.input().as_str()) {
                        let header = biomcp_adapters::chembl::molecule_header(row)
                            .map_err(|_| biomcp_fetch::InputError::Serialization)?;
                        problems.extend(header.problems);
                        candidates.push((
                            (
                                u8::from(matched == DrugNameMatch::Generic),
                                0,
                                String::new(),
                            ),
                            header.value,
                        ));
                    }
                }
                Ok(NativePage {
                    candidates,
                    next_offset,
                    problems,
                    refusals,
                })
            });
        if out
            .sources
            .iter()
            .any(|s| s.state != biomcp_types::SourceState::Ok)
        {
            return (out, Vec::new());
        }
        all.extend(candidates);
        outcome = Some(out);
    }
    (
        outcome.unwrap_or_else(|| {
            status::empty(biomcp_types::DrugPage::into_reply(query.clone(), None))
        }),
        all,
    )
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
