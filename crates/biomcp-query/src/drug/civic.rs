//! Source-local CIViC therapy candidates and independent native claim streams.
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::civic;
use biomcp_types::{
    DrugGet, DrugPage, DrugSearch, DrugSection, DrugSource, PageInfo, QueryBody, Request, Source,
    SourceState, Status, StepKind,
};

pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = super::empty(QueryBody::DrugPage(DrugPage {
        items: Box::default(),
        total: None,
    }));
    let Some(name) = query.query() else {
        return super::unsupported(query.source(), out);
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&civic::SPEC, deadline).fresh(cx.fresh);
    let result = civic::drug_search(&leg, name, query.limit(), query.offset());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Civic, leg.finish(), error));
    if let Ok(mapped) = result {
        out.problems = mapped.problems;
        let (page, consumed) = mapped.value;
        {
        let (found, identity_failed, unavailable) = if page.items.is_empty() {
            (false, false, false)
        } else {
            (true, false, false)
        };
        crate::status::item(&mut out, found, identity_failed, unavailable);
    };
        let returned = page.items.len() as u32;
        out.sources.iter_mut().for_each(|s| {
            s.count = page.total;
            s.retrieved_count = Some(returned);
        });
        out.page = page.total.map(|total| PageInfo {
            offset: query.offset(),
            returned,
            total,
            pool: None,
            truncated: None,
        });
        for item in &page.items {
            if let Ok(get) = DrugGet::new(item.id.clone(), DrugSource::Civic)
                .and_then(|q| q.for_section(DrugSection::Civic))
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
        if let Some(offset) = query
            .offset()
            .checked_add(consumed)
            .filter(|n| consumed > 0 && page.total.is_some_and(|t| u64::from(*n) < t))
        {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::SearchDrug(query.at_offset(offset)),
                    fresh: cx.fresh,
                },
            ));
        }
        out.body = QueryBody::DrugPage(page);
    }
    command.finish(deadline.remaining());
    out
}
fn state(out: &Outcome) -> Status {
    if !out.problems.is_empty() || out.sources.iter().any(|s| s.state == SourceState::Stale) {
        Status::Partial
    } else {
        Status::Complete
    }
}
pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let mut out = super::empty(QueryBody::DrugCivic(None));
    if !matches!(query.sections(), [DrugSection::Civic]) {
        return super::unsupported(query.source(), out);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&civic::SPEC, deadline).fresh(cx.fresh);
    let result = civic::drug_civic(&leg, query);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Civic, leg.finish(), error));
    if let Ok(mapped) = result {
        out.problems = mapped.problems;
        let Some(mut value) = mapped.value else {
            crate::status::item(&mut out, true, false, false);
            command.finish(deadline.remaining());
            return out;
        };
        if let biomcp_types::Section::Data { data, .. } = &value.civic {
            for stream in [&data.evidence, &data.assertions].into_iter().flatten() {
                if let Some(next) = &stream.page.next {
                    out.next.push((
                        StepKind::Page,
                        Action::Run {
                            request: Request::GetDrug(next.clone()),
                            fresh: cx.fresh,
                        },
                    ));
                }
            }
        }
        super::next::civic(&mut value, cx.fresh, &mut out);
        crate::status::item(&mut out, true, false, false);
        out.body = QueryBody::DrugCivic(Some(value));
    }
    command.finish(deadline.remaining());
    out
}
