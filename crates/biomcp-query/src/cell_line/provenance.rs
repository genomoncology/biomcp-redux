use super::{Ctx, Outcome, QueryBody, Source, Status, cellosaurus, status};
/// Release metadata is an independent cacheable read, with its own failure state.
pub(super) fn attach(cx: &Ctx<'_>, deadline: biomcp_fetch::Deadline<'_>, out: &mut Outcome) {
    let leg = cx.fetch.leg(&cellosaurus::SPEC, deadline).fresh(cx.fresh);
    let release = cellosaurus::release(&leg);
    let error = release.as_ref().err().copied();
    let incoming = status::source(Source::Cellosaurus, leg.finish(), error);
    if let Some(source) = out.sources.first_mut() {
        status::retain_source_state(source, &incoming);
    }
    if error.is_some() {
        out.status = Status::Partial;
        out.local = error.and_then(status::local);
        out.problems.push(biomcp_types::Problem {
            source: Source::Cellosaurus,
            record: None,
            field: "release".into(),
            code: biomcp_types::ProblemCode::ValueUnknown,
        });
    }
    match &mut out.body {
        QueryBody::CellLinePage(page) => page.release = release.ok(),
        QueryBody::CellLineCard(card) => card.release = release.ok(),
        _ => {}
    }
    if out
        .sources
        .iter()
        .any(|s| s.state == biomcp_types::SourceState::Stale)
    {
        out.status = Status::Partial;
    }
}
