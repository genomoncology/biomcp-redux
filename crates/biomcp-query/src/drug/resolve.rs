//! Resolve complete source names before the unchanged native-ID path.
use crate::{Ctx, Outcome, status};
use biomcp_types::{DrugGet, DrugHeader, DrugNameMatch, DrugNameResolution, DrugSearch, DrugSource, ReplyData, Step, StepKind, UsageArguments, UsageError};
use biomcp_types::ReplyBody;

type Rank = (u8, u8, String);

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let name = query.input().to_string();
    let search: DrugSearch = match serde_json::from_value(serde_json::json!({"query": name, "source": query.source(), "limit": 50})) {
        Ok(search) => search,
        Err(_) => return status::empty(ReplyData::Usage(UsageError::arguments())),
    };
    let (mut discovery, mut candidates) = if query.source() == DrugSource::Drugsfda {
        fda(cx, &search)
    } else { discover(cx, &search) };
    candidates.sort_by(|a, b| a.0.cmp(&b.0));
    candidates.dedup_by(|a, b| a.1.id == b.1.id);
    let Some((_, selected)) = candidates.first() else {
        if discovery.sources.iter().any(|s| s.state != biomcp_types::SourceState::Ok) { return discovery; }
        discovery.body = ReplyData::Usage(UsageError::Arguments(UsageArguments {
            argument: Some("input".into()),
            next: vec![Step { kind: StepKind::Disambiguate, args: vec!["search".into(), "drug".into(), name.into(), "--source".into(), query.source().as_str().into()].into() }],
            ..Default::default()
        }));
        return discovery;
    };
    let Ok(resolved) = query.with_native_id(selected.id.clone()) else { return discovery; };
    let mut out = super::selected_get(cx, &resolved);
    let resolution = DrugNameResolution {
        input: search.query().cloned().expect("name search has input"),
        total: candidates.len() as u64,
        alternatives: candidates.iter().skip(1).take(5).map(|(_, h)| Step {
            kind: StepKind::Disambiguate,
            args: vec!["get".into(), "drug".into(), h.id.to_string().into(), "--source".into(), query.source().as_str().into()].into(),
        }).collect(),
    };
    annotate(&mut out.body, resolution);
    out.problems.extend(discovery.problems);
    for source in discovery.sources {
        if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source.source) { status::retain_source_state(existing, &source); }
        else { out.sources.push(source); }
    }
    out
}

fn discover(cx: &Ctx<'_>, search: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let mut query = search.clone();
    let mut out = super::search(cx, &query);
    let mut candidates = Vec::new();
    loop {
        if let ReplyData::DrugPage { body: Some(page), .. } = &out.body {
            for header in &page.items {
                let rank = match header.matched_name {
                    Some(DrugNameMatch::Brand | DrugNameMatch::Both) => Some(0),
                    Some(DrugNameMatch::Generic) => Some(1),
                    _ if header.name.as_ref().is_some_and(|n| search.query().is_some_and(|q| n.as_str().eq_ignore_ascii_case(q.as_str()))) => Some(0),
                    _ => None,
                };
                if let Some(rank) = rank { candidates.push(((rank, 0, String::new()), header.clone())); }
            }
        }
        let Some(page) = &out.page else { break; };
        if page.returned == 0 || u64::from(page.offset) + u64::from(page.returned) >= page.total { break; }
        query = query.at_offset(page.offset + page.returned);
        let next = super::search(cx, &query);
        if next.sources.iter().any(|s| s.state != biomcp_types::SourceState::Ok) { out = next; break; }
        out = next;
    }
    (out, candidates)
}

fn fda(cx: &Ctx<'_>, query: &DrugSearch) -> (Outcome, Vec<(Rank, DrugHeader)>) {
    let mut out = status::empty(biomcp_types::DrugCard::into_reply(
        DrugGet::new(query.query().expect("name").to_string().parse().expect("checked input"), query.source()).expect("checked source"), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&biomcp_adapters::drugsfda::SPEC, deadline).fresh(cx.fresh);
    let mut candidates = Vec::new();
    let mut offset = 0;
    let mut error = None;
    loop {
        match biomcp_adapters::drugsfda::search(&leg, query.query().expect("name"), 50, offset) {
            Ok(page) => {
                out.include_refusals(page.refusals);
                out.problems.extend(page.problems);
                for row in &page.records {
                    if let Some((name, application, date)) = row.name_rank(query.query().expect("name").as_str()) {
                        candidates.push(((name, application, date.unwrap_or_else(|| "9999".into())), row.header()));
                    }
                }
                offset += page.records.len() as u32;
                if page.records.is_empty() || u64::from(offset) >= page.total { break; }
            }
            Err(failure) => { error = Some(failure); break; }
        }
    }
    out.local = error.and_then(status::local);
    out.sources.push(status::source(query.source().source(), leg.finish(), error));
    status::search(&mut out);
    command.finish(deadline.remaining());
    (out, candidates)
}

fn annotate(body: &mut ReplyData, resolution: DrugNameResolution) {
    let header = match body {
        ReplyData::DrugCard { body: Some(body), .. } => &mut body.header,
        ReplyData::DrugApprovals { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugLabel { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugRegulatory { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugTargets { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugIndications { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugCivic { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugInteractions { body: Some(body), .. } => &mut body.identity,
        ReplyData::DrugXrefs { body: Some(body), .. } => &mut body.identity,
        _ => return,
    };
    header.name_resolution = Some(resolution);
}
