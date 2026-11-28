//! Orphanet OMIM mappings and epidemiology for an Orphanet code or a concept with Orphanet links.
use super::page::{empty, page_info, run, slice};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::{Curie, DiseaseRelation};
use biomcp_adapters::{mydisease, orphadata};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGet, DiseasePrevalence, DiseaseSection, DiseaseXrefs, OrphanetEpidemiology, Reason,
    Request, Source, SourceNotice, SourceState, SourceStatus, StepKind,
};

pub(super) fn card(
    cx: &Ctx<'_>,
    query: &DiseaseGet,
    id: &Curie,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) {
    let leg = cx.fetch.leg(&orphadata::SPEC, deadline);
    let reply = orphadata::card(&leg, id);
    let error = reply.as_ref().err().copied();
    let mut source = status::source(Source::Orphadata, leg.finish(), error);
    if matches!(source.state, SourceState::Syncing | SourceState::Stale) {
        sync_step(out, Bundle::ORPHADATA_XREFS);
    }
    out.local = error.and_then(status::local);
    let card = match reply {
        Ok(mapped) => {
            out.include_refusals(mapped.refusals);
            out.problems.extend(mapped.problems);
            mapped.value
        }
        Err(_) => {
            out.sources.push(source);
            status::item(out, false, true, false);
            return;
        }
    };
    let found = card.is_some();
    if let Some(card) = card {
        source.notices.push(acknowledgment());
        out.next.extend(prevalence_step(id, &mut out.problems));
        out.body = card.with_query(query.clone());
    }
    out.sources.push(source);
    let failed = !found
        && out
            .sources
            .iter()
            .any(|source| source.state == SourceState::Failed);
    status::item(out, found, failed, false);
}

pub(crate) fn get(cx: &Ctx<'_>, query: &DiseaseGet, id: &Curie) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    if query.section() == Some(DiseaseSection::Xrefs) {
        let out = xrefs(cx, query, id, deadline);
        command.finish(deadline.remaining());
        return out;
    }
    let mut out = empty(body(query, Vec::new(), Vec::new()));
    let Some((codes, links)) = linked(cx, query, id, deadline, &mut out) else {
        command.finish(deadline.remaining());
        return out;
    };
    if codes.is_empty() {
        // The concept exists but MONDO states no Orphanet link, so no file row can be joined.
        out.sources.push(biomcp_adapters::registry::outcome(
            Source::Orphadata,
            SourceStatus::Skipped(Some(Reason::NoJoinId)),
        ));
        status::item(&mut out, true, false, true);
        status::missing(
            &mut out,
            biodata::text!("/result/items"),
            Source::Orphadata,
            Some(Reason::NoJoinId),
        );
        command.finish(deadline.remaining());
        return out;
    }
    let leg = cx.fetch.leg(&orphadata::SPEC, deadline);
    let reply = orphadata::epidemiology(&leg, &codes);
    let error = reply.as_ref().err().copied();
    let source = status::source(Source::Orphadata, leg.finish(), error);
    let sync = matches!(source.state, SourceState::Syncing | SourceState::Stale);
    out.sources.push(source);
    out.local = out.local.or(error.and_then(status::local));
    match reply {
        Ok(mapped) => {
            out.include_refusals(mapped.refusals);
            out.problems.extend(mapped.problems);
            let (items, total) = slice(mapped.value, query);
            page(&mut out, query, items.len() as u32, total);
            let found = total > 0;
            out.body = body(query, items, links);
            status::item(&mut out, found, false, false);
        }
        Err(_) => {
            out.body = body(query, Vec::new(), links);
            status::item(&mut out, false, true, false);
        }
    }
    if sync {
        sync_step(&mut out, Bundle::ORPHADATA);
    }
    command.finish(deadline.remaining());
    out
}

fn xrefs(
    cx: &Ctx<'_>,
    query: &DiseaseGet,
    id: &Curie,
    deadline: biomcp_fetch::Deadline<'_>,
) -> Outcome {
    let mut body = DiseaseXrefs {
        items: Box::default(),
        components: Box::default(),
    };
    let mut out = empty(body.clone().with_query(query.clone()));
    let leg = cx.fetch.leg(&orphadata::SPEC, deadline);
    let reply = orphadata::omim_xrefs(&leg, id);
    let error = reply.as_ref().err().copied();
    let mut source = status::source(Source::Orphadata, leg.finish(), error);
    if matches!(source.state, SourceState::Syncing | SourceState::Stale) {
        sync_step(&mut out, Bundle::ORPHADATA_XREFS);
    }
    out.local = error.and_then(status::local);
    if reply.is_ok() {
        source.notices.push(acknowledgment());
    }
    out.sources.push(source);
    match reply {
        Ok(mapped) => {
            out.include_refusals(mapped.refusals);
            out.problems.extend(mapped.problems);
            let (items, total) = slice(mapped.value, query);
            page(&mut out, query, items.len() as u32, total);
            body.items = items.into_iter().map(Into::into).collect();
            out.body = body.with_query(query.clone());
            status::item(&mut out, total > 0, false, false);
        }
        Err(_) => status::item(&mut out, false, true, false),
    }
    out
}

fn acknowledgment() -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: orphadata::NOTICE.into(),
        retry_after_seconds: None,
    }
}

fn page(out: &mut Outcome, query: &DiseaseGet, returned: u32, total: u64) {
    let offset = query.section_offset().saturating_add(returned);
    out.page = Some(page_info(query.section_offset(), returned, total));
    if returned > 0 && u64::from(offset) < total {
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &[Source::Orphadata],
            run(Request::GetDisease(query.at_section_offset(offset))),
        ));
    }
}

fn sync_step(out: &mut Outcome, bundle: Bundle) {
    out.next.push(crate::next::Step::new(
        StepKind::Sync,
        &[Source::Orphadata],
        Action::Sync { bundle },
    ));
}

/// The Orphanet codes to read: the requested code, or the concept's MONDO-stated Orphanet links.
/// None when the concept itself could not be read.
fn linked(
    cx: &Ctx<'_>,
    query: &DiseaseGet,
    id: &Curie,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Option<(Vec<Curie>, Vec<DiseaseRelation>)> {
    if orphadata::orpha_code(id.as_str()).is_some() {
        return Some((vec![id.clone()], Vec::new()));
    }
    let leg = cx.fetch.leg(&mydisease::SPEC, deadline).fresh(cx.fresh);
    let reply = mydisease::annotation(&leg, id);
    let error = reply.as_ref().err().copied();
    out.sources
        .push(status::source(Source::MyDisease, leg.finish(), error));
    out.local = error.and_then(status::local);
    let mapped = match reply {
        Ok(mapped) => mapped,
        Err(_) => {
            super::page::record_problem(out, query, Source::MyDisease);
            return None;
        }
    };
    out.include_refusals(mapped.refusals);
    out.problems.extend(mapped.problems);
    let links: Vec<DiseaseRelation> = mapped
        .value
        .xrefs
        .into_iter()
        .filter(|xref| orphadata::orpha_code(xref.object.id.as_str()).is_some())
        .collect();
    Some((links.iter().map(|l| l.object.id.clone()).collect(), links))
}

fn body(
    query: &DiseaseGet,
    items: Vec<OrphanetEpidemiology>,
    links: Vec<DiseaseRelation>,
) -> biomcp_types::ReplyData {
    (DiseasePrevalence {
        items: items.into(),
        components: Box::default(),
        links: links.into(),
    })
    .with_query(query.clone())
}

/// A card's next step when MONDO states an Orphanet link for the concept.
pub(crate) fn step(
    id: &Curie,
    xrefs: &[DiseaseRelation],
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<crate::next::Step> {
    if !xrefs
        .iter()
        .any(|xref| orphadata::orpha_code(xref.object.id.as_str()).is_some())
    {
        return None;
    }
    prevalence_step(id, problems)
}

fn prevalence_step(
    id: &Curie,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<crate::next::Step> {
    let query = crate::next::checked(
        DiseaseGet::orphadata_prevalence(id.clone()),
        Source::Orphadata,
        problems,
    )?;
    Some(crate::next::Step::new(
        StepKind::Section,
        &[Source::Orphadata],
        run(Request::GetDisease(query)),
    ))
}
