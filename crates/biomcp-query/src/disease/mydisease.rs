use super::page::{empty, page_info, run, slice};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::mydisease;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGet, DiseaseHierarchy, DiseaseSection, DiseaseSynonyms, DiseaseXrefs, Reason, Request,
    Source, SourceStatus, StepKind,
};

/// Read only the selected exact annotation, then page its native component facts.
pub(crate) fn get(cx: &Ctx<'_>, query: &DiseaseGet, id: &biodata::Curie) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(biomcp_types::DiseaseCard::into_reply(query.clone(), None));
    if id.as_str().starts_with("OMIM:") && omim_gene(cx, id, deadline, &mut out) {
        crate::status::item(&mut out, false, false, false);
        command.finish(deadline.remaining());
        return out;
    }
    let leg = cx.fetch.leg(&mydisease::SPEC, deadline).fresh(cx.fresh);
    let mut reply = mydisease::annotation(&leg, id);
    let mut label_problems = Vec::new();
    let mut label_error = None;
    let hierarchy = matches!(query.section(), Some(DiseaseSection::Hierarchy));
    for mapped in reply.iter_mut().filter(|_| hierarchy) {
        let page = mapped
            .value
            .parents
            .iter_mut()
            .skip(query.section_offset() as usize)
            .take(query.section_limit() as usize);
        label_error = label(&leg, page.collect(), &mut label_problems);
    }
    let error = reply.as_ref().err().copied();
    // A failed parent-name read leaves the rows unnamed, so the source and reply are not complete.
    let mut source = status::source(Source::MyDisease, leg.finish(), error.or(label_error));
    let reason = error.and_then(status::reason);
    let fallback =
        query.orpha_card_fallback() && matches!(reason, Some(Reason::NotFound | Reason::BadReply));
    if fallback && reason == Some(Reason::NotFound) {
        source.set_status(SourceStatus::Skipped(reason));
    }
    out.sources.push(source);
    if fallback {
        super::page::record_problem(&mut out, query, Source::MyDisease);
        super::orphadata::card(cx, query, id, deadline, &mut out);
        command.finish(deadline.remaining());
        return out;
    }
    out.local = error.and_then(status::local);
    if let Ok(mapped) = reply {
        out.include_refusals(mapped.refusals);
        out.problems = mapped.problems;
        out.problems.extend(label_problems);
        notices(&mut out, &mapped.value.card, false);
        apply(&mut out, query, mapped.value);
    }
    if error.is_some() || label_error.is_none() {
        super::page::record_problem(&mut out, query, Source::MyDisease);
    }
    command.finish(deadline.remaining());
    out
}
/// A positive, exact gene entry stops the disease lookup and offers the known gene IDs.
fn omim_gene(
    cx: &Ctx<'_>,
    id: &biodata::Curie,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> bool {
    use biomcp_adapters::hgnc;
    use biomcp_types::{GeneGet, GeneInput, SourceNotice};

    let leg = cx.fetch.leg(&hgnc::OMIM_SPEC, deadline).fresh(cx.fresh);
    let reply = hgnc::omim_genes(&leg, id);
    let error = reply.as_ref().err().copied();
    let mut source = status::source(Source::Hgnc, leg.finish(), error);
    let (is_gene, notice) = match reply {
        Ok(ids) if !ids.is_empty() => {
            for id in ids {
                let query = crate::next::checked(
                    GeneGet::new(vec![GeneInput::HgncId(id)].into()),
                    Source::Hgnc,
                    &mut out.problems,
                );
                out.next.extend(query.map(|query| {
                    crate::next::Step::new(
                        StepKind::Pivot,
                        &[Source::Hgnc],
                        run(Request::GetGene(query)),
                    )
                }));
            }
            (true, Some(format!("HGNC lists {id} as a gene entry.")))
        }
        Ok(_) => (false, None),
        Err(_) => (
            false,
            Some(format!(
                "HGNC could not check whether {id} is a gene entry."
            )),
        ),
    };
    source
        .notices
        .extend(notice.map(|message| SourceNotice::RequestGuidance {
            data_date: None,
            message: message.into(),
            retry_after_seconds: None,
        }));
    out.sources.push(source);
    is_gene
}

/// Name each direct parent on the page from its own record in one batch; an unnamed parent keeps its ID.
/// Returns the lookup failure, which the caller reports on the source.
fn label(
    leg: &biomcp_fetch::Leg<'_>,
    parents: Vec<&mut biodata::DiseaseRelation>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<biomcp_fetch::RequestError> {
    let ids: Vec<_> = parents.iter().map(|row| row.object.id.clone()).collect();
    match mydisease::parent_labels(leg, &ids) {
        Ok(labels) => {
            problems.extend(labels.problems);
            for row in parents {
                if let Some(label) = labels.value.get(&row.object.id.to_string()) {
                    row.object.label = Some(label.clone());
                }
            }
            None
        }
        Err(error) => {
            problems.push(biomcp_types::Problem {
                source: Source::MyDisease,
                record: None,
                field: "parents.label: parent label lookup failed".into(),
                code: biomcp_types::ProblemCode::ValueUnknown,
            });
            Some(error)
        }
    }
}
fn apply(out: &mut Outcome, query: &DiseaseGet, record: mydisease::Annotation) {
    let components = record.card.components.clone();
    match query.section() {
        Some(DiseaseSection::Synonyms) => {
            let (items, total) = slice(record.synonyms, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = (DiseaseSynonyms {
                items: items.into(),
                components,
            })
            .with_query(query.clone());
        }
        Some(DiseaseSection::Xrefs) => {
            let (items, total) = slice(record.xrefs, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = (DiseaseXrefs {
                items: items.into_iter().map(Into::into).collect(),
                components,
            })
            .with_query(query.clone());
        }
        Some(DiseaseSection::Hierarchy) => {
            let (items, total) = slice(record.parents, query);
            out.page = Some(page_info(query.section_offset(), items.len() as u32, total));
            out.body = (DiseaseHierarchy {
                items: items.into(),
                components,
            })
            .with_query(query.clone());
        }
        _ => {
            for section in &record.card.sections {
                out.next.push(crate::next::Step::new(
                    StepKind::Section,
                    &[biomcp_types::Source::MyDisease],
                    run(Request::GetDisease(query.with_section(*section))),
                ));
            }
            // Orphadata reads an Orphanet code itself; any other ID goes through the MONDO record's links.
            let card = &record.card;
            let id = if biomcp_adapters::orphadata::orpha_code(card.requested_id.as_str()).is_some()
            {
                &card.requested_id
            } else {
                &card.returned_id
            };
            let step = super::orphadata::step(id, &record.xrefs, &mut out.problems);
            out.next.extend(step);
            out.body = (record.card).with_query(query.clone());
        }
    }
    if let Some(page) = &out.page {
        let offset = page.offset.saturating_add(page.returned);
        if page.returned > 0 && u64::from(offset) < page.total {
            out.next.push(crate::next::Step::new(
                StepKind::Page,
                &[biomcp_types::Source::MyDisease],
                run(Request::GetDisease(query.at_section_offset(offset))),
            ));
        }
    }
    crate::status::item(out, true, false, false);
}

/// The MONDO record MyDisease reads for a cross-referenced ID, with the MyDisease source and its notices
/// recorded on `out`. None when the read failed or reached no MONDO record.
pub(crate) fn mondo(
    cx: &Ctx<'_>,
    query: &DiseaseGet,
    id: &biodata::Curie,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Option<biodata::Curie> {
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
    let card = mapped.value.card;
    notices(out, &card, true);
    if !card.returned_id.as_str().starts_with("MONDO:") {
        crate::status::item(out, false, false, false);
        return None;
    }
    Some(card.returned_id)
}
/// Plain MyDisease notices: the cross-reference that named a record read for another reply, and a
/// cross-referenced record that MONDO marks obsolete.
fn notices(out: &mut Outcome, card: &biomcp_types::DiseaseCard, mapping: bool) {
    let Some(alias) = &card.alias else {
        return;
    };
    let mut messages = Vec::new();
    if mapping {
        messages.push(format!(
            "{} was read as {}, the MONDO record whose cross-reference {} states {}; a MONDO \
             cross-reference may be broader than the code asked for.",
            card.requested_id, card.returned_id, alias.predicate, alias.object.id
        ));
    }
    if card.obsolete == Some(true) {
        let replacement: Vec<String> = card
            .replaced_by
            .iter()
            .flatten()
            .map(ToString::to_string)
            .collect();
        messages.push(if replacement.is_empty() {
            format!(
                "MONDO marks {} obsolete and names no replacement.",
                card.returned_id
            )
        } else {
            format!(
                "MONDO marks {} obsolete and names {} as its replacement.",
                card.returned_id,
                replacement.join(", ")
            )
        });
    }
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::MyDisease)
    {
        source.notices.extend(messages.into_iter().map(|message| {
            biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            }
        }));
    }
}
