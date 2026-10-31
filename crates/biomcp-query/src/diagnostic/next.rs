use crate::{Action, Outcome};
use biomcp_types::{DiagnosticCandidate, DiagnosticGet, DiagnosticSearch, Request, StepKind};
pub(super) fn push(
    out: &mut Outcome,
    kind: StepKind,
    sources: &[biomcp_types::Source],
    request: Request,
    fresh: bool,
) {
    out.next.push(crate::next::Step::new(
        kind,
        sources,
        Action::Run { request, fresh },
    ));
}
pub(super) fn candidates(
    out: &mut Outcome,
    query: &DiagnosticSearch,
    rows: &[DiagnosticCandidate],
    fresh: bool,
) {
    for row in rows {
        let (id, source) = match row {
            DiagnosticCandidate::DeviceRecall { header } => (
                header.recall_number.to_string(),
                biomcp_types::DiagnosticGetSource::FdaDeviceRecall,
            ),
            DiagnosticCandidate::Resolved { header, .. } => (
                header.id.to_string(),
                biomcp_types::DiagnosticGetSource::Gtr,
            ),
            DiagnosticCandidate::WhoIvd { id, .. } => {
                (id.to_string(), biomcp_types::DiagnosticGetSource::WhoIvd)
            }
            _ => continue,
        };
        let get = id
            .parse()
            .map_err(|_| biomcp_types::QueryInvalid::new(&["id"]))
            .and_then(|id| DiagnosticGet::new(id, source, None));
        if let Some(mut get) = crate::next::checked(get, source.source(), &mut out.problems) {
            if let Some(revision) = query.bundle_revision() {
                get = get.with_bundle_revision(revision.clone());
            }
            push(
                out,
                StepKind::Disambiguate,
                &[get.source().source()],
                Request::GetDiagnostic(get),
                fresh,
            );
        }
    }
}
