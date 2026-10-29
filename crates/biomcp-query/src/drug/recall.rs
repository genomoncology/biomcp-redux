//! Source-native drug recall search and exact retrieval.
use super::empty;
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::fda_drug_recall as source;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DrugGet, DrugRecallPage, DrugSearch, DrugSource, PageInfo, QueryBody, Request, Source,
    SourceState, Status, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &DrugSearch) -> Outcome {
    let mut out = empty(QueryBody::DrugRecallPage(DrugRecallPage {
        count_unit: biomcp_types::DrugRecallCountUnit::RecallRecords,
        items: Box::default(),
        total: 0,
    }));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::search(&leg, q);
    let error = result.as_ref().err().copied();
    if let Ok(page) = result {
        let returned = page.items.len() as u32;
        out.status = if returned == 0 {
            Status::Absent
        } else {
            Status::Complete
        };
        out.problems = page.problems;
        for item in &page.items {
            if let Ok(id) = item.recall_number.to_string().parse()
                && let Ok(get) = DrugGet::new(id, DrugSource::FdaDrugRecall)
            {
                out.next.push((
                    StepKind::Disambiguate,
                    Action::Run {
                        request: Request::GetDrug(get),
                        fresh: cx.fresh,
                    },
                ));
            }
        }
        if let Some(offset) = q
            .offset()
            .checked_add(returned)
            .filter(|n| returned > 0 && u64::from(*n) < page.total)
        {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::SearchDrug(q.at_offset(offset)),
                    fresh: cx.fresh,
                },
            ));
        }
        out.page = Some(PageInfo {
            offset: q.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        out.body = QueryBody::DrugRecallPage(DrugRecallPage {
            count_unit: biomcp_types::DrugRecallCountUnit::RecallRecords,
            items: page.items,
            total: page.total,
        });
    }
    finish(&mut out, leg, error);
    command.finish(deadline.remaining());
    out
}
pub(super) fn get(cx: &Ctx<'_>, q: &DrugGet) -> Outcome {
    let mut out = empty(QueryBody::DrugRecallCard(None));
    let Ok(id) = q.input().to_string().parse() else {
        return out;
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let mut error = None;
    match source::get(&leg, &id) {
        Ok(Some(row)) => {
            out.status = Status::Complete;
            out.problems = row.problems;
            out.body = QueryBody::DrugRecallCard(Some(row.value));
        }
        Ok(None) => out.status = Status::Absent,
        Err(failure) => {
            error = Some(failure.error);
            if let Some(field) = failure.identity_field {
                out.problems.push(biomcp_types::Problem {
                    source: Source::FdaDrugRecall,
                    record: q.input().to_string().parse().ok(),
                    field: field.into(),
                    code: biomcp_types::ProblemCode::IdentityConflict,
                });
            }
        }
    }
    finish(&mut out, leg, error);
    command.finish(deadline.remaining());
    out
}
fn finish(out: &mut Outcome, leg: Leg<'_>, error: Option<RequestError>) {
    out.local = error.and_then(status::local);
    let source = status::source(Source::FdaDrugRecall, leg.finish(), error);
    if out.status == Status::Complete
        && (source.state == SourceState::Stale || !out.problems.is_empty())
    {
        out.status = Status::Partial;
    }
    out.sources.push(source);
}
