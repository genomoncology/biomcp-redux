use super::{empty, searched, unsupported};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biomcp_adapters::ddinter;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugGet, DrugInteractions, DrugPage, DrugSearch, DrugSection, DrugSource, PageInfo, Request,
    Section, Source, SourceState, StepKind,
};

pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = empty(
        (DrugPage {
            items: Box::default(),
            total: None,
        })
        .with_query(query.clone()),
    );
    let Some(name) = query.query() else {
        return super::unsupported(query.source(), out);
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&ddinter::SPEC, deadline).fresh(cx.fresh);
    let result = ddinter::search(&leg, name.as_str());
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Ddinter, leg.finish(), error));
    if let Ok(mapped) = result {
        out.include_refusals(mapped.refusals);
        out.problems.extend(mapped.problems);
        let rows = mapped.value;
        let total = rows.len() as u64;
        let page = DrugPage {
            items: rows
                .into_iter()
                .skip(query.offset() as usize)
                .take(query.limit() as usize)
                .collect(),
            total: Some(total),
        };
        searched(query, cx.fresh, page, &mut out);
    }
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}

pub(super) fn get(cx: &Ctx<'_>, query: &DrugGet) -> Outcome {
    let interactions = query.sections() == [DrugSection::Interactions];
    let mut out = empty(if interactions {
        biomcp_types::DrugInteractions::into_reply(query.clone(), None)
    } else {
        biomcp_types::DrugCard::into_reply(query.clone(), None)
    });
    if !query.supports_sections() {
        return unsupported(query.source(), out);
    }
    let id = match query.native_input() {
        Ok(id) => id,
        Err(_) => return super::rejected_input(query.source(), "input", out),
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&ddinter::SPEC, deadline).fresh(cx.fresh);
    let mut aliased = None;
    let result = if interactions {
        read_interactions(cx, query, &leg, &id, &mut out).map(|(found, alias)| {
            aliased = alias;
            found
        })
    } else {
        ddinter::card(&leg, &id).map(|card| {
            out.include_refusals(card.refusals);
            out.problems.extend(card.problems);
            let card = card.value;
            if card.is_some()
                && let Some(get) = crate::next::checked(
                    query.for_section(DrugSection::Interactions),
                    biomcp_types::Source::Ddinter,
                    &mut out.problems,
                )
            {
                out.next.push(crate::next::Step::run(
                    StepKind::Section,
                    &[biomcp_types::Source::Ddinter],
                    Request::GetDrug(get),
                    cx.fresh,
                ));
            }
            let found = card.is_some();
            out.body = biomcp_types::DrugCard::into_reply(query.clone(), card);
            found
        })
    };
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(Source::Ddinter, leg.finish(), error));
    if let Some(aliased) = aliased {
        aliased.append(&mut out);
    }
    if let Ok(found) = result {
        status::item(&mut out, found, false, false);
    }
    sync_step(&mut out);
    command.finish(deadline.remaining());
    out
}

/// No DDInter identity answers the partner and none was refused.
fn unnamed(body: &biomcp_adapters::Mapped<Option<DrugInteractions>>) -> bool {
    body.refusals == biomcp_adapters::Refusal::None
        && body
            .value
            .as_ref()
            .is_some_and(|body| body.partners.is_empty())
}

/// Read one interactions page. A partner name DDInter does not use is retried under the names
/// another source states for it, as the drug itself is.
fn read_interactions(
    cx: &Ctx<'_>,
    query: &DrugGet,
    leg: &biomcp_fetch::Leg<'_>,
    id: &biodata::DrugId,
    out: &mut Outcome,
) -> Result<(bool, Option<super::aliases::Aliased>), biomcp_fetch::RequestError> {
    let read = |partner: Option<&str>| {
        ddinter::interactions(
            leg,
            id,
            partner,
            query.section_limit().unwrap_or(5),
            query.section_offset().unwrap_or(0),
        )
    };
    let mut body = read(query.partner().map(|p| p.as_str()))?;
    let mut aliased = None;
    if let Some(partner) = query.partner()
        && unnamed(&body)
        && !partner
            .as_str()
            .parse::<biodata::DrugId>()
            .is_ok_and(|id| DrugSource::Ddinter.accepts(&id))
    {
        let mut failure = None;
        let (found, alias) =
            super::aliases::partner(cx, partner, |name| match read(Some(name.as_str())) {
                Ok(found) => (!unnamed(&found)).then_some(found),
                Err(error) => {
                    failure.get_or_insert(error);
                    None
                }
            });
        aliased = Some(alias);
        match (found, failure) {
            (Some(found), _) => body = found,
            (None, Some(error)) => return Err(error),
            (None, None) => {}
        }
    }
    let unanswered = unnamed(&body);
    out.include_refusals(body.refusals);
    out.problems.extend(body.problems);
    let body = body.value;
    let found = body.is_some();
    if unanswered {
        partner_search(query, cx.fresh, out);
    }
    out.body = biomcp_types::DrugInteractions::into_reply(
        query.clone(),
        body.map(|body| page(query, cx.fresh, body, out)),
    );
    Ok((found, aliased))
}
/// The DDInter name search for a partner DDInter's files do not name.
fn partner_search(query: &DrugGet, fresh: bool, out: &mut Outcome) {
    if let Some(partner) = query.partner()
        && let Some(search) = crate::next::checked(
            DrugSearch::named(partner.clone(), DrugSource::Ddinter, 10),
            Source::Ddinter,
            &mut out.problems,
        )
    {
        out.next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &[biomcp_types::Source::Ddinter],
            Request::SearchDrug(search),
            fresh,
        ));
    }
}
fn page(
    query: &DrugGet,
    fresh: bool,
    body: DrugInteractions,
    out: &mut Outcome,
) -> DrugInteractions {
    if let Section::Data { data, .. } = &body.interactions {
        let returned = data.records.len() as u32;
        let total = data.total.unwrap_or(0);
        out.page = Some(PageInfo {
            offset: data.offset,
            returned,
            total,
            pool: None,
            truncated: None,
        });
        if let Some(offset) = data
            .offset
            .checked_add(returned)
            .filter(|n| returned > 0 && u64::from(*n) < total)
            && let Some(get) = crate::next::checked(
                query.at_section_offset(offset),
                biomcp_types::Source::Ddinter,
                &mut out.problems,
            )
        {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[biomcp_types::Source::Ddinter],
                Request::GetDrug(get),
                fresh,
            ));
        }
    }
    body
}
fn sync_step(out: &mut Outcome) {
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Syncing | SourceState::Stale))
    {
        out.next.push(crate::next::Step::new(
            StepKind::Sync,
            &[biomcp_types::Source::Ddinter],
            Action::Sync {
                bundle: Bundle::DDINTER,
            },
        ));
    }
}
