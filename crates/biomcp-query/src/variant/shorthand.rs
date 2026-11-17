//! Candidate discovery for a gene/protein phrase on the established variant-card surface.
use crate::{Action, Ctx, Outcome, next::Step, status};
use biomcp_fetch::Deadline;
use biomcp_types::{
    Paged, ReplyData, Request, Source, Status, StepKind, VariantCard, VariantGet,
    VariantResolution, VariantSearch,
};

pub(super) fn get<'a>(
    cx: &Ctx<'a>,
    query: &VariantGet,
    search: &VariantSearch,
    deadline: &mut Deadline<'a>,
) -> Outcome {
    let mut out = super::search::search(cx, search, deadline);
    let ReplyData::VariantPage { body, .. } = &mut out.body else {
        return out;
    };
    let Some(body) = body.take() else {
        return out;
    };
    let mut actions = followups(cx, query, &body.items, &mut out);
    let mut resolution = classify(&body, &out, &actions);
    if resolution == VariantResolution::Resolved && !query.sections().is_empty() {
        if let Some(candidate) = exact_choice(&actions)
            && let Some(exact) = attach(cx, query, candidate, deadline, &mut out)
        {
            return exact;
        }
        resolution = VariantResolution::Candidates;
    }
    if resolution == VariantResolution::Unavailable {
        actions.push(Step::run(
            StepKind::Fresh,
            &[Source::MyVariant],
            Request::GetVariant(query.clone()),
            true,
        ));
    }
    out.next = actions;
    status::item(
        &mut out,
        !body.items.is_empty(),
        resolution == VariantResolution::Unavailable,
        resolution == VariantResolution::Candidates && !query.sections().is_empty(),
    );
    out.body = super::reply(
        query,
        VariantCard {
            source: Source::MyVariant,
            resolution,
            candidates: body.items,
            rejection: None,
        },
        &out.sources,
    );
    out
}

fn classify(body: &biomcp_types::VariantPage, out: &Outcome, steps: &[Step]) -> VariantResolution {
    let total = out.page.as_ref().map(|page| page.total);
    if body.items.is_empty() {
        if out.status() == Status::Complete && total == Some(0) {
            return VariantResolution::Absent;
        }
        return VariantResolution::Unavailable;
    }
    // A display limit of one and a source total of one are different claims.
    if out
        .page
        .as_ref()
        .is_some_and(|page| page.total == 1 && page.offset == 0)
        && out.status() == Status::Complete
        && body.excluded.is_none()
        && matches!(body.items.as_ref(), [row] if row.transcripts.iter().any(|row| row.matched))
        && exact_choice(steps).is_some()
    {
        VariantResolution::Resolved
    } else {
        VariantResolution::Candidates
    }
}

fn exact_choice(steps: &[Step]) -> Option<&VariantGet> {
    let [
        Step(
            StepKind::Disambiguate,
            Action::Run {
                request: Request::GetVariant(candidate),
                ..
            },
            _,
        ),
    ] = steps
    else {
        return None;
    };
    Some(candidate)
}

fn followups(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[biomcp_types::VariantHeader],
    out: &mut Outcome,
) -> Vec<Step> {
    let mut actions = Vec::new();
    for row in candidates {
        if let Some((request, source)) = super::identity::choice(row, &mut out.problems) {
            let (request, sources) = preserve(cx, query, request, source, &mut out.problems);
            actions.push(Step::run(StepKind::Disambiguate, &sources, request, false));
        }
    }
    for step in &out.next {
        if let Action::Run {
            request: Request::SearchVariant(page),
            fresh,
        } = &step.1
        {
            actions.push(Step::run(
                StepKind::Page,
                &step.2,
                Request::GetVariant(query.at_offset(page.offset())),
                *fresh,
            ));
        }
    }
    actions
}

fn attach<'a>(
    cx: &Ctx<'a>,
    query: &VariantGet,
    candidate: &VariantGet,
    deadline: &mut Deadline<'a>,
    discovery: &mut Outcome,
) -> Option<Outcome> {
    let mut exact = super::get(cx, candidate, deadline);
    if let ReplyData::VariantCard {
        body: Some(card), ..
    } = exact.body
        && matches!(
            card.resolution,
            VariantResolution::Resolved | VariantResolution::Unresolved
        )
        && card.candidates.len() == 1
    {
        merge_sources(&mut exact.sources, std::mem::take(&mut discovery.sources));
        status::append(&mut exact.problems, std::mem::take(&mut discovery.problems));
        exact.body = super::reply(query, card, &exact.sources);
        return Some(exact);
    }
    // Keep the discoverable candidate and the exact request if attaching a section failed.
    discovery.local = discovery.local.or(exact.local);
    merge_sources(&mut discovery.sources, exact.sources);
    status::append(&mut discovery.problems, exact.problems);
    None
}

fn preserve(
    cx: &Ctx<'_>,
    query: &VariantGet,
    request: Request,
    source: Source,
    problems: &mut Vec<biomcp_types::Problem>,
) -> (Request, Vec<Source>) {
    if let Request::GetVariant(candidate) = &request {
        let with_build = match candidate.build() {
            Some(build) => query.with_build(build),
            None => Ok(query.clone()),
        };
        let selection = with_build.and_then(|q| q.for_candidate(candidate.input().clone()));
        if let Some(q) = crate::next::checked(selection, source, problems) {
            let sources = super::next_sources(cx.fetch, &q);
            return (Request::GetVariant(q), sources);
        }
    }
    (request, vec![source])
}

fn merge_sources(
    sources: &mut Vec<biomcp_types::SourceOutcome>,
    other: Vec<biomcp_types::SourceOutcome>,
) {
    for source in other {
        if let Some(existing) = sources
            .iter_mut()
            .find(|current| current.source == source.source)
        {
            status::retain_source_state(existing, &source);
        } else {
            sources.push(source);
        }
    }
}
