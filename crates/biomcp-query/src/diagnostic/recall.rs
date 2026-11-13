use super::{empty, next};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::fda_device_recall as adapter;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiagnosticCard, DiagnosticCountUnit, DiagnosticGet, DiagnosticPage, DiagnosticSearch, PageInfo,
    Problem, ProblemCode, Request, Source, SourceNotice, SourceState, Status, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &DiagnosticSearch) -> Outcome {
    let source_query = || Some(adapter::expression(q).into_boxed_str()).filter(|s| !s.is_empty());
    let mut out = empty(
        (DiagnosticPage {
            items: Box::default(),
            count_unit: Some(DiagnosticCountUnit::RecallRecords),
            distinct_id_total: None,
            source_query: source_query(),
        })
        .with_query(q.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&adapter::SPEC, deadline).fresh(cx.fresh);
    let mut unmatched = (0, 0);
    let result = adapter::search(&leg, q);
    let mut error = result.as_ref().err().copied();
    if let Ok(page) = result {
        let returned = page.items.len() as u32;
        {
            let found = page.total != 0;
            crate::status::item(&mut out, found, false, false);
        };
        out.problems = page.problems;
        unmatched = (page.unmatched, returned);
        if let Some(failure) = page.join_failure {
            error = Some(failed(&mut out, None, failure));
        }
        next::candidates(&mut out, q, &page.items, cx.fresh);
        if u64::from(q.offset()) + u64::from(returned) < page.total && returned > 0 {
            next::push(
                &mut out,
                StepKind::Page,
                &[q.source().source()],
                Request::SearchDiagnostic(q.at_offset(q.offset() + returned)),
                cx.fresh,
            );
        }
        out.page = Some(PageInfo {
            offset: q.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        out.body = (DiagnosticPage {
            items: page.items,
            count_unit: Some(DiagnosticCountUnit::RecallRecords),
            distinct_id_total: None,
            source_query: source_query(),
        })
        .with_query(q.clone());
    }
    let rows_survive = error.is_some() && out.page.is_some();
    finish(&mut out, leg, error);
    if rows_survive {
        crate::status::item(&mut out, true, false, true);
    } else {
        status::search(&mut out);
    }
    if let Some(source) = out.sources.last_mut() {
        if q.classification().is_some() {
            source.notices.push(guidance(CLASS_NOTICE.into()));
        }
        if q.name().is_some() {
            source.notices.push(guidance(NAME_NOTICE.into()));
        }
        if unmatched.0 > 0 {
            source
                .notices
                .push(unmatched_notice(q, unmatched.0, unmatched.1));
        }
    }
    command.finish(deadline.remaining());
    out
}
const CLASS_NOTICE: &str = "--classification selects openFDA device enforcement records: the total counts \
    enforcement records, and --manufacturer and the initiation dates match enforcement fields.";
const NAME_NOTICE: &str = "--name matched the words of openFDA's product_description as one phrase, \
    ignoring case and punctuation, without plural or synonym expansion: the total counts recall records \
    whose description holds the phrase, a different set from a --product-code search.";
/// Count rows the exact join left without their other openFDA record; nothing is inferred for them.
fn unmatched_notice(q: &DiagnosticSearch, rows: u32, returned: u32) -> SourceNotice {
    let (missing, shown) = if q.classification().is_some() {
        ("recall", "only the recall number and classification")
    } else {
        ("enforcement", "no classification")
    };
    guidance(format!(
        "{rows} of {returned} returned rows have no exact openFDA device {missing} record and show {shown}."
    ))
}
fn guidance(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}
pub(super) fn get(cx: &Ctx<'_>, q: &DiagnosticGet, id: &biodata::DeviceRecallNumber) -> Outcome {
    let mut out = empty(biomcp_types::DiagnosticCard::into_reply(q.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&adapter::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    match adapter::get(&leg, id) {
        Ok(Some(mut mapped)) => {
            crate::status::item(&mut out, true, false, false);
            out.include_refusals(mapped.refusals);
            out.problems.append(&mut mapped.problems);
            match adapter::enforcement(&leg, id, &mut mapped.value) {
                Ok(mut problems) => out.problems.append(&mut problems),
                Err(failure) => {
                    error = Some(failed(&mut out, Some(q.record().clone()), failure));
                    crate::status::item(&mut out, true, false, true);
                }
            }
            if let Some(code) = &mapped.value.header.product_code
                && let Some(search) = crate::next::checked(
                    biomcp_types::AdverseEventSearch::maude_product(code.clone()),
                    Source::Maude,
                    &mut out.problems,
                )
            {
                next::push(
                    &mut out,
                    StepKind::Pivot,
                    &[biomcp_types::Source::Maude],
                    Request::SearchAdverseEvent(search),
                    cx.fresh,
                );
            }
            if !cx.fresh {
                next::push(
                    &mut out,
                    StepKind::Fresh,
                    &[q.source().source()],
                    Request::GetDiagnostic(q.clone()),
                    true,
                );
            }
            out.body = (DiagnosticCard::DeviceRecall(Box::new(mapped.value))).with_query(q.clone());
        }
        Ok(None) => crate::status::item(&mut out, false, false, false),
        Err(failure) => error = Some(failed(&mut out, Some(q.record().clone()), failure)),
    }
    finish(&mut out, leg, error);
    command.finish(deadline.remaining());
    out
}
fn failed(
    out: &mut Outcome,
    record: Option<biodata::Text>,
    failure: adapter::Failure,
) -> RequestError {
    if let Some(field) = failure.identity_field {
        out.problems.push(Problem {
            source: Source::FdaDeviceRecall,
            record,
            field: field.into(),
            code: ProblemCode::IdentityConflict,
        });
    }
    failure.error
}
fn finish(out: &mut Outcome, leg: Leg<'_>, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    let source = status::source(Source::FdaDeviceRecall, leg.finish(), error);
    if out.status() == Status::Complete && (source.state == SourceState::Stale || out.dropped) {
        crate::status::item(out, true, false, true);
    }
    out.sources.push(source);
}
