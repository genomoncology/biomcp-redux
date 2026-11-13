//! Orphanet epidemiology for an Orphanet code, or a MONDO or DOID concept through its own Orphanet links.
use super::page::{empty, page_info, run, slice};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::{Curie, DiseaseRelation};
use biomcp_adapters::{mydisease, orphadata};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGet, DiseasePrevalence, OrphanetEpidemiology, Reason, Request, Source, SourceState,
    SourceStatus, StepKind,
};

pub(crate) fn get(cx: &Ctx<'_>, query: &DiseaseGet, id: &Curie) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
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
        out.page = Some(page_info(query.section_offset(), 0, 0));
        status::item(&mut out, false, false, false);
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
            let returned = items.len() as u32;
            out.page = Some(page_info(query.section_offset(), returned, total));
            let offset = query.section_offset().saturating_add(returned);
            if returned > 0 && u64::from(offset) < total {
                out.next.push(crate::next::Step::new(
                    StepKind::Page,
                    &[Source::Orphadata],
                    run(Request::GetDisease(query.at_section_offset(offset))),
                ));
            }
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
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[Source::Orphadata],
            Action::Sync {
                bundle: Bundle::ORPHADATA,
            },
        ));
    }
    command.finish(deadline.remaining());
    out
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
    if orphadata::orpha_code(id).is_some() {
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
        .filter(|xref| orphadata::orpha_code(&xref.object.id).is_some())
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
        .any(|xref| orphadata::orpha_code(&xref.object.id).is_some())
    {
        return None;
    }
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
