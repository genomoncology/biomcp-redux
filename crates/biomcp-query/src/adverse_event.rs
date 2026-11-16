//! Explicit adverse-event sources; FAERS widens drug names by default, MAUDE and VAERS stay literal.
use biomcp_types::Paged;
use biomcp_types::ReplyBody;
mod counts;
mod get;
mod names;
mod vaers;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{faers, maude};
use biomcp_types::{
    AdverseEventPage, AdverseEventSearch, PageInfo, ReplyData, ReportUnit, Request, Source,
    StepKind,
};
/// Search report headers through the declared source and its reviewed component admission.
pub(crate) fn search_command(cx: &Ctx<'_>, query: &AdverseEventSearch) -> Outcome {
    if query.source() == biomcp_types::AdverseEventSource::Vaers {
        return vaers::search(cx, query);
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let out = if query.widens_drug_names()
        && !cx.fetch.disabled(Source::Faers)
        && !(cx.fetch.disabled(Source::Meddra)
            && (query.reaction().is_some()
                || query
                    .aggregate()
                    .is_some_and(|aggregate| aggregate.required_component().is_some())))
    {
        names::search(cx, query, &mut deadline)
    } else {
        search(cx, query, query, deadline)
    };
    command.finish(deadline.remaining());
    disclose(out, query)
}
/// Name each typed openFDA value sent as other words (0841), and whether it was searched in an
/// OR group: FAERS ORs the search text with every --drug, MAUDE ORs every --product-code (0976).
fn disclose(mut out: Outcome, query: &AdverseEventSearch) -> Outcome {
    let faers = query.source() == biomcp_types::AdverseEventSource::Faers;
    let source = if faers { Source::Faers } else { Source::Maude };
    let names = usize::from(query.text().is_some()) + query.drug().len();
    let codes = query.product_code().len() > 1;
    let named = [
        ("The search text", query.text(), faers && names > 1),
        ("--manufacturer", query.manufacturer(), false),
        ("--problem", query.problem(), false),
        ("--reaction", query.reaction(), false),
    ];
    let listed = (query.drug().iter().map(|v| ("--drug", Some(v), names > 1))).chain(
        query
            .product_code()
            .iter()
            .map(|v| ("--product-code", Some(v), codes)),
    );
    crate::openfda_text::disclose(&mut out, source, named.into_iter().chain(listed));
    counts_notes(&mut out, query, names);
    out
}

/// What a FAERS count counts (1012): every name goes in one query, so a report naming several
/// counts once; seriousnessdeath marks the report, not a reaction, so a death count with
/// --reaction is reports flagged death that list that reaction. Said only when FAERS answered.
fn counts_notes(out: &mut Outcome, query: &AdverseEventSearch, typed: usize) {
    if query.source() != biomcp_types::AdverseEventSource::Faers {
        return;
    }
    let Some(report) = out.sources.iter_mut().find(|report| {
        report.source == Source::Faers
            && matches!(
                report.state,
                biomcp_types::SourceState::Ok | biomcp_types::SourceState::Stale
            )
    }) else {
        return;
    };
    let said = report.notices.iter().any(|notice| {
        matches!(notice, biomcp_types::SourceNotice::DrugNameSearch { message, .. }
            if message.contains(names::COUNTS_ONCE))
    });
    let mut notes = Vec::new();
    if typed > 1 && !said {
        notes.push(format!("The {typed} names went in {}.", names::COUNTS_ONCE));
    }
    if query.serious() == Some(biomcp_types::AdverseEventSerious::Death)
        && query.reaction().is_some()
    {
        notes.push(
            "Counts reports flagged death that list this reaction, not deaths caused by it.".into(),
        );
    }
    report.notices.extend(
        notes
            .into_iter()
            .map(|message| biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            }),
    );
}

fn search<'a>(
    cx: &Ctx<'a>,
    query: &AdverseEventSearch,
    navigation_query: &AdverseEventSearch,
    deadline: biomcp_fetch::Deadline<'a>,
) -> Outcome {
    if query
        .aggregate()
        .is_some_and(|aggregate| aggregate.is_received())
    {
        return counts::received(cx, query, navigation_query, deadline);
    }
    if query.aggregate().is_some() {
        return counts::search(cx, query, navigation_query, deadline);
    }
    let source = query.source().source();
    let spec = if source == Source::Faers {
        &faers::SPEC
    } else {
        &maude::SPEC
    };
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let selector = if source == Source::Faers {
        faers::expression(query)
    } else {
        maude::expression(query)
    };
    let reply = if source == Source::Faers {
        faers::search(&leg, query)
    } else {
        maude::search(&leg, query)
    };
    let error = reply.as_ref().err().copied();
    let mut out = crate::status::finish(
        [crate::status::item_status(false, true, false)],
        crate::status::Parts {
            dropped: false,
            body: (AdverseEventPage {
                reaction_counts: None,
                received_counts: None,
                reaction_match: query
                    .reaction()
                    .map(|_| biomcp_types::AdverseEventReactionMatch::PhraseMatch),
                source_query: selector.into_boxed_str(),
                source_updated: None,
                fetched: None,
                items: Box::default(),
                unit: ReportUnit::Reports,
                components: Box::default(),
            })
            .with_query(navigation_query.clone()),
            page: None,
            sources: vec![status::source(source, leg.finish(), error)],
            problems: Vec::new(),
            next: Vec::new(),
            local: error.and_then(status::local),
        },
    );
    if let Ok(page) = reply {
        let returned = page.items.len() as u32;
        navigation(navigation_query, &page, &mut out);
        out.include_refusals(page.refusals);
        out.problems.extend(page.problems);
        out.page = Some(PageInfo {
            offset: query.offset(),
            returned,
            total: page.total,
            pool: None,
            truncated: None,
        });
        crate::status::search(&mut out);
        if let ReplyData::AdverseEventPage {
            body: Some(body), ..
        } = &mut out.body
        {
            body.items = page.items.into();
            body.components = page.components;
            body.source_updated = page.updated.map(Into::into);
            body.fetched = page
                .metadata
                .fetched
                .and_then(|value| u64::try_from(value).ok());
        }
    }
    out
}

/// An unimplemented report get has no source effects until its owning slice lands.
pub(crate) fn unsupported() -> Outcome {
    crate::status::empty(ReplyData::Usage(
        biomcp_types::UsageError::UnsupportedAdverseEvent,
    ))
}

fn navigation(query: &AdverseEventSearch, page: &faers::Page, out: &mut Outcome) {
    let returned = page.items.len() as u32;
    for row in &page.items {
        if let Some(get) = crate::next::checked(
            biomcp_types::AdverseEventGet::new(row.id.clone(), query.source()),
            query.source().source(),
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[query.source().source()],
                Request::GetAdverseEvent(get),
                false,
            ));
        }
    }
    let offset = query.offset().saturating_add(returned);
    if returned > 0 && u64::from(offset) < page.total {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[query.source().source()],
            Request::SearchAdverseEvent(query.at_offset(offset)),
            false,
        ));
    }
}

/// Read an exact FAERS report and its independent retained sections.
pub(crate) fn get(cx: &Ctx<'_>, query: &biomcp_types::AdverseEventGet) -> Outcome {
    if query.source() != biomcp_types::AdverseEventSource::Faers {
        return unsupported();
    }
    get::get(cx, query)
}
