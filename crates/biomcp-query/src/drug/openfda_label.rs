//! Native SPL identities and unchanged explicit label headings.
use crate::{Action, Ctx, Outcome, status};
use biomcp_adapters::{openfda_label as source, received_date};
use biomcp_fetch::RequestError;
use biomcp_types::{
    DrugGet, DrugLabel, DrugPage, DrugSearch, DrugSection, DrugSource, PageInfo, QueryBody, Reason,
    Request, Section, Source, SourceState, Status, StepKind,
};
const SOURCE: Source = Source::OpenfdaLabel;
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
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::search(&leg, name, query.limit(), query.offset()).and_then(|page| {
        Ok((
            page.records
                .iter()
                .map(|r| {
                    let mut header = r.header()?;
                    header.matched_name = r.name_match(name);
                    Ok(header)
                })
                .collect::<Result<Vec<_>, RequestError>>()?,
            page.total,
            page.scanned,
        ))
    });
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    if let Ok((items, total, scanned)) = result {
        let returned = items.len() as u32;
        out.status = if returned == 0 {
            Status::Absent
        } else {
            state(&out)
        };
        if let Some(source) = out.sources.first_mut() {
            source.count = Some(total);
            source.retrieved_count = Some(returned);
        }
        out.page = Some(page(query.offset(), returned, total));
        for item in &items {
            if let Ok(get) = DrugGet::new(item.id.clone(), DrugSource::OpenfdaLabel) {
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
            .checked_add(scanned)
            .filter(|n| scanned > 0 && u64::from(*n) < total)
        {
            out.next.push((
                StepKind::Page,
                Action::Run {
                    request: Request::SearchDrug(query.at_offset(offset)),
                    fresh: cx.fresh,
                },
            ));
        }
        out.body = QueryBody::DrugPage(DrugPage {
            items: items.into(),
            total: Some(total),
        });
    }
    command.finish(deadline.remaining());
    out
}
pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let label = matches!(query.sections(), [DrugSection::Label]);
    let mut out = super::empty(if label {
        QueryBody::DrugLabel(None)
    } else {
        QueryBody::DrugCard(None)
    });
    if !matches!(query.sections(), [] | [DrugSection::Label])
        || label
            && !query
                .label_section()
                .is_some_and(|h| source::HEADINGS.contains(&h.as_str()))
    {
        return super::unsupported(DrugSource::OpenfdaLabel, out);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::get(&leg, query.input());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    if let Ok(page) = result {
        out.status = state(&out);
        if let Some(source) = out.sources.first_mut() {
            source.count = Some(page.total);
            source.retrieved_count = Some(page.records.len() as u32);
        }
        if let Some(record) = page.records.first() {
            if let Err(error) = select(cx, query, record, page.metadata.fetched, &mut out) {
                out.status = Status::Failed;
                out.local = status::local(error);
            }
        } else {
            out.status = Status::Absent;
        }
    }
    command.finish(deadline.remaining());
    out
}
fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    record: &source::Record,
    fetched: Option<i64>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    if query.sections().is_empty() {
        let card = record.card(received_date(fetched))?;
        for heading in source::HEADINGS
            .into_iter()
            .filter(|h| record.parts(h).is_some())
        {
            if let Ok(heading) = heading.parse()
                && let Ok(request) = query.for_label(heading)
            {
                out.next.push((
                    StepKind::Section,
                    Action::Run {
                        request: Request::GetDrug(request),
                        fresh: cx.fresh,
                    },
                ));
            }
        }
        out.body = QueryBody::DrugCard(Some(card));
        return Ok(());
    }
    let heading = query.label_section().map_or("", |h| h.as_str());
    let offset = query.section_offset().unwrap_or(0);
    let label = match record.label(heading, offset, query.section_limit().unwrap_or(1))? {
        Some(mut data) => {
            data.retrieved = received_date(fetched);
            let returned = data.fragments.len() as u32;
            let total = data.total.unwrap_or(0);
            out.page = Some(page(offset, returned, total));
            if let Some(next) = offset
                .checked_add(returned)
                .filter(|n| returned > 0 && u64::from(*n) < total)
                && let Ok(request) = query.at_section_offset(next)
            {
                out.next.push((
                    StepKind::Page,
                    Action::Run {
                        request: Request::GetDrug(request),
                        fresh: cx.fresh,
                    },
                ));
            }
            Section::Data {
                source: SOURCE,
                data,
            }
        }
        None => {
            out.status = Status::Partial;
            Section::unavailable(SOURCE, Reason::NotFound)
        }
    };
    out.body = QueryBody::DrugLabel(Some(DrugLabel {
        identity: record.header()?,
        label,
    }));
    Ok(())
}
fn state(out: &Outcome) -> Status {
    if out.sources.iter().any(|s| s.state == SourceState::Stale) {
        Status::Partial
    } else {
        Status::Complete
    }
}
fn page(offset: u32, returned: u32, total: u64) -> PageInfo {
    PageInfo {
        offset,
        returned,
        total,
        pool: None,
        truncated: None,
    }
}
