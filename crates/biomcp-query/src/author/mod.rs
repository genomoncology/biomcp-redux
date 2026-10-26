//! Provider-record author search and independent exact cards.
mod get;
mod search;
use crate::{Action, Outcome};
use biomcp_types::{
    Problem, ProblemCode, QueryBody, Reason, Request, Source, SourceState, Status, StepKind,
};
pub use get::get;
pub use search::search;
fn empty(body: QueryBody) -> Outcome {
    Outcome {
        status: Status::Failed,
        body,
        page: None,
        sources: vec![],
        problems: vec![],
        next: vec![],
        local: None,
    }
}
fn run(request: Request) -> Action {
    Action::Run {
        request,
        fresh: false,
    }
}
fn recovery(cx: &crate::Ctx<'_>, out: &mut Outcome, request: Request) {
    if out
        .sources
        .iter()
        .any(|s| s.reason == Some(Reason::BadReply))
    {
        out.problems.push(Problem {
            source: Source::SemanticScholar,
            record: None,
            field: "author".into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    if out
        .sources
        .iter()
        .any(|s| matches!(s.state, SourceState::Failed | SourceState::Stale))
    {
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request,
                fresh: true,
            },
        ));
    }
    if out
        .sources
        .iter()
        .any(|s| s.reason == Some(Reason::RateLimited))
        && !cx
            .fetch
            .has_key(biomcp_fetch::SecretName::SemanticScholarApiKey)
    {
        out.next
            .push((StepKind::Key, Action::KeyHelp(Source::SemanticScholar)));
    }
}
