//! Native FDA application discovery; product and submission scope remain separate.
use super::{empty, unsupported};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{drugsfda as source, received_date};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugCard, DrugGet, DrugSection, DrugSource, Request, Source, SourceState, StepKind,
};
mod applications;
mod approvals;
pub(super) use applications::search;
mod history;
mod history_read;
mod history_reply;
mod history_search;
mod history_settle;
mod label_contents;
mod label_plain;
mod label_sections;
#[cfg(test)]
mod label_sections_tests;
mod label_text;
mod letter;
mod purpose;
/// Fail the item. A failed reply whose source still reads ok would name no cause, so the
/// source takes the reason.
fn fail(out: &mut Outcome, error: biomcp_fetch::RequestError) {
    crate::status::item(out, false, true, false);
    out.local = status::local(error);
    if let Some(reason) = status::reason(error)
        && let Some(source) = out
            .sources
            .iter_mut()
            .find(|s| s.source == Source::Drugsfda && s.state == SourceState::Ok)
    {
        source.set_status(biomcp_types::SourceStatus::Failed(Some(reason)));
    }
}
pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = empty_get(query);
    if !query.supports_sections() {
        return unsupported(query.source(), out);
    }
    let id = match query.native_input() {
        Ok(id) => id,
        Err(_) => return super::rejected_input(query.source(), "input", out),
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::get(&leg, &id);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Drugsfda, leg.finish(), error));
    if let Ok(page) = result {
        if let Some(source) = out.sources.first_mut() {
            source.count = Some(page.total);
            source.retrieved_count = Some(page.records.len() as u32);
        }
        out.include_refusals(page.refusals);
        out.problems.extend(page.problems);
        {
            let (found, identity_failed, unavailable) = if page.records.is_empty() {
                (false, false, false)
            } else if out.sources.iter().any(|s| s.state == SourceState::Stale) {
                (true, false, true)
            } else {
                (true, false, false)
            };
            crate::status::item(&mut out, found, identity_failed, unavailable);
        };
        if let Some(row) = page.records.first() {
            label_steps(cx, row, &mut out);
            let metadata = (page.metadata.fetched, page.updated);
            if !read_section(cx, query, row, metadata, deadline, &mut out) {
                if let Some(request) = crate::next::checked(
                    query.for_section(DrugSection::Approvals),
                    biomcp_types::Source::Drugsfda,
                    &mut out.problems,
                ) {
                    out.next.push(crate::next::Step::run(
                        StepKind::Section,
                        &[biomcp_types::Source::Drugsfda],
                        Request::GetDrug(request),
                        cx.fresh,
                    ));
                }
                out.body = (DrugCard {
                    header: row.header(),
                    alternate_names: Box::default(),
                    official_names: Box::default(),
                    cross_references: Box::default(),
                    parents: Box::default(),
                    products: Box::default(),
                    formulations: Box::default(),
                    version: None,
                    updated: None,
                    retrieved: received_date(page.metadata.fetched),
                    available_sections: vec![DrugSection::Approvals].into(),
                    next: Box::default(),
                })
                .with_query(query.clone());
            }
        }
    }
    command.finish(deadline.remaining());
    out
}
/// Read the approvals or label-history section of `row`; false for any other request.
fn read_section(
    cx: &Ctx<'_>,
    query: &DrugGet,
    row: &source::Application,
    metadata: (Option<i64>, Option<biodata::PartialDate>),
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> bool {
    let read = match query.sections() {
        [DrugSection::LabelHistory] => history::select(cx, query, row, deadline, out),
        [DrugSection::Approvals] => approvals::select(cx, query, row, metadata, deadline, out),
        _ => return false,
    };
    if let Err(error) = read {
        fail(out, error);
    }
    true
}
fn empty_get(query: &DrugGet) -> Outcome {
    if query.sections() == [DrugSection::LabelHistory] {
        return history::empty(query);
    }
    let is_approvals = query.sections() == [DrugSection::Approvals];
    empty(if is_approvals {
        biomcp_types::DrugApprovals::into_reply(query.clone(), None)
    } else {
        biomcp_types::DrugCard::into_reply(query.clone(), None)
    })
}

fn label_steps(cx: &Ctx<'_>, row: &source::Application, out: &mut Outcome) {
    for id in row.label_ids() {
        if let Some(request) = crate::next::checked(
            DrugGet::new(id.clone(), DrugSource::OpenfdaLabel),
            biomcp_types::Source::Drugsfda,
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[request.source().source()],
                Request::GetDrug(request),
                cx.fresh,
            ));
        }
    }
}
