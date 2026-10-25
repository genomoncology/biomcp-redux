use crate::{Action, Outcome};
use biomcp_types::{DiagnosticCandidate, DiagnosticGet, DiagnosticSearch, Request, StepKind};
pub(super) fn push(out: &mut Outcome, kind: StepKind, request: Request, fresh: bool) {
    out.next.push((kind, Action::Run { request, fresh }));
}
pub(super) fn candidates(
    out: &mut Outcome,
    query: &DiagnosticSearch,
    rows: &[DiagnosticCandidate],
    fresh: bool,
) {
    for row in rows {
        if let DiagnosticCandidate::Resolved { header, .. } = row {
            if let Ok(mut get) = serde_json::from_value::<DiagnosticGet>(
                serde_json::json!({"id":header.id,"source":"gtr"}),
            ) {
                if let Some(revision) = query.bundle_revision() {
                    get = get.with_bundle_revision(revision.clone());
                }
                push(
                    out,
                    StepKind::Disambiguate,
                    Request::GetDiagnostic(get),
                    fresh,
                );
            }
        }
    }
}
