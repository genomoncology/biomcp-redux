//! Declared MyVariant searches keep native candidates, totals and page selectors.
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    PageInfo, QueryBody, Request, Source, Status, StepKind, VariantGet, VariantPage,
    VariantResolver, VariantSearch,
};
use biomcp_adapters::myvariant;
/// Discover one source-native page; a zero result remains distinct from a failed source.
pub fn search(cx: &Ctx<'_>, query: &VariantSearch) -> Outcome {
    if query.raw_query() && !cx.fetch.disabled(Source::MyVariant)
        && myvariant::components::restricted(cx.fetch) {
        return policy_error();
    }
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&myvariant::SPEC, deadline).fresh(cx.fresh);
    let (terms, reply) = match filter_terms(query) {
        Ok(terms) => (terms, fetch_page(cx, &leg, query)),
        Err(error) => (None, Err(error)),
    };
    let error = reply.as_ref().err().copied();
    let source = status::source(Source::MyVariant, leg.finish(), error);
    let mut out = Outcome {
        status: Status::Failed,
        body: QueryBody::VariantPage(VariantPage {
            filter_terms: terms.clone(),
            source_query: query.source_query(),
            build: query.build(),
            items: Box::default(),
        }),
        page: None,
        sources: vec![source],
        problems: Vec::new(),
        next: Vec::new(),
        local: error.and_then(status::local),
    };
    if let Ok(reply) = reply {
        out.status = if !reply.rows.problems.is_empty()
            || reply.total > u64::from(VariantSearch::MAX_RESULTS)
            || out
                .sources
                .iter()
                .any(|s| s.state == biodata::SourceState::Stale)
        {
            Status::Partial
        } else if reply.total == 0 {
            Status::Absent
        } else {
            Status::Complete
        };
        let returned = reply.rows.value.len() as u32;
        out.next = next(query, &reply.rows.value, returned, reply.total);
        out.page = Some(PageInfo {
            offset: query.offset(),
            returned,
            total: reply.total,
            pool: None,
            truncated: (reply.total > u64::from(VariantSearch::MAX_RESULTS)).then_some(true),
        });
        out.problems = reply.rows.problems;
        out.body = QueryBody::VariantPage(VariantPage {
            filter_terms: terms,
            source_query: query.source_query(),
            build: query.build(),
            items: reply.rows.value.into(),
        });
    }
    command.finish(deadline.remaining());
    out
}
fn fetch_page(cx: &Ctx<'_>, leg: &biomcp_fetch::Leg<'_>, query: &VariantSearch) -> Result<myvariant::SearchPage, biomcp_fetch::RequestError> {
    if query.uses_clinvar_filters() && myvariant::components::disabled(cx.fetch, "clinvar") {
        return Err(biomcp_fetch::SourceError {
            source: Source::MyVariant,
            reason: biodata::Reason::Disabled,
            status: None,
            retry_after: None,
        }.into());
    }
    myvariant::search(leg, query)
}
fn filter_terms(query: &VariantSearch) -> Result<Option<biodata::ComponentTerms>, biomcp_fetch::RequestError> {
    if !query.uses_clinvar_filters() { return Ok(None); }
    let terms = myvariant::components::terms("clinvar")
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    Ok(Some(myvariant::components::label(terms)?))
}
fn policy_error() -> Outcome {
    Outcome {
        status: Status::Failed,
        body: QueryBody::VariantQueryPolicyError,
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    }
}
fn next(
    query: &VariantSearch,
    rows: &[biodata::VariantHeader],
    returned: u32,
    total: u64,
) -> Vec<(StepKind, Action)> {
    let mut next = Vec::new();
    for row in rows {
        if let Ok(input) = row.record.id.as_str().parse()
            && let Ok(q) = VariantGet::new(input, VariantResolver::Car, Some(query.build()))
        {
            next.push((
                StepKind::Disambiguate,
                Action::Run {
                    request: Request::GetVariant(q),
                    fresh: false,
                },
            ));
        }
    }
    let offset = query.offset() + returned;
    if u64::from(offset) < total && offset < VariantSearch::MAX_RESULTS && returned > 0 {
        next.push((
            StepKind::Page,
            Action::Run {
                request: Request::SearchVariant(query.at_offset(offset)),
                fresh: false,
            },
        ));
    }
    next
}
