//! Native FDA application discovery; product and submission scope remain separate.
use super::{empty, searched, unsupported};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::{drugsfda as source, received_date};
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugCard, DrugFdaPage, DrugGet, DrugPage, DrugSearch, DrugSection, DrugSource, Request, Source,
    SourceState, StepKind,
};
mod approvals;
mod letter;
mod purpose;
const SEARCHED: &str = "Searched these names in one Drugs@FDA query, at the exact or word step \
    the typed name reaches; the total is openFDA's own count of matching applications, not a sum \
    per name.";

pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(
        (DrugFdaPage {
            items: Box::default(),
            total: 0,
            source_updated: None,
        })
        .with_query(query.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let fits = |names: &[Text]| source::names_fit(names, query.applicant());
    let mut widened = match super::widen::begin(cx, query, &mut deadline, fits, &mut out) {
        super::widen::Start::Typed => None,
        super::widen::Start::Widened(saved) => Some(saved),
        super::widen::Start::Answered => return out,
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = match (widened.as_mut(), query.query()) {
        (Some(saved), Some(typed)) => widened_page(&leg, query, typed, saved),
        _ => source::search(
            &leg,
            query.query(),
            query.applicant(),
            query.limit(),
            query.offset(),
        ),
    };
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Drugsfda, leg.finish(), error));
    if let Ok(page) = result {
        out.include_refusals(page.refusals);
        // Later pages keep this page's names, so the snapshot is kept only for an answer.
        let sent = super::widen::retain(cx, query, widened.as_ref(), deadline, true, &mut out);
        out.problems.extend(page.problems);
        searched(
            &sent,
            cx.fresh,
            DrugPage {
                items: page.records.iter().map(|r| r.header()).collect(),
                total: Some(page.total),
            },
            &mut out,
        );
        out.body = (DrugFdaPage {
            items: page
                .records
                .iter()
                .map(source::Application::candidate)
                .collect(),
            total: page.total,
            source_updated: page.updated.map(Into::into),
        })
        .with_query(sent.clone());
    }
    if let Some(applicant) = query.applicant()
        && let Some(report) = out.sources.first_mut()
    {
        report
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: format!(
                    "--applicant matches sponsor names starting with {:?}, ignoring letter case.",
                    source::applicant_prefix(applicant)
                )
                .into(),
                retry_after_seconds: None,
            });
    }
    if let Some(saved) = widened {
        super::widen::finish(query, saved, SEARCHED, &mut out);
    }
    command.finish(deadline.remaining());
    out
}

/// Search the name group at its step; after a refusal, search the typed name alone and keep it.
fn widened_page(
    leg: &biomcp_fetch::Leg<'_>,
    query: &DrugSearch,
    typed: &Text,
    saved: &mut super::widen::Saved<source::NameStep>,
) -> Result<source::Page, biomcp_fetch::RequestError> {
    let page = (query.limit(), query.offset());
    let (result, step) = source::search_widened(
        leg,
        (&saved.names, typed),
        query.applicant(),
        page,
        saved.step,
    );
    saved.step = Some(step);
    if saved.names.len() < 2 || !super::search_names::rejected(result.as_ref().err()) {
        return result;
    }
    saved.reject(query);
    let (result, step) =
        source::search_widened(leg, (&saved.names, typed), query.applicant(), page, None);
    saved.step = Some(step);
    result
}
pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let (is_approvals, mut out) = empty_get(query);
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
            if is_approvals {
                if let Err(error) = approvals::select(
                    cx,
                    query,
                    row,
                    (page.metadata.fetched, page.updated),
                    deadline,
                    &mut out,
                ) {
                    crate::status::item(&mut out, false, true, false);
                    out.local = status::local(error);
                }
            } else {
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
fn empty_get(query: &DrugGet) -> (bool, Outcome) {
    let is_approvals = query.sections() == [DrugSection::Approvals];
    let out = empty(if is_approvals {
        biomcp_types::DrugApprovals::into_reply(query.clone(), None)
    } else {
        biomcp_types::DrugCard::into_reply(query.clone(), None)
    });
    (is_approvals, out)
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
