//! Expose source-owned catalog continuations without changing their exact query.
use crate::Action;
use biodata::{Request, Section, StepKind, VariantHeader};
pub(super) fn collect(candidates: &[VariantHeader], next: &mut Vec<(StepKind, Action)>) {
    for candidate in candidates {
        let Some(Section::Data { data, .. }) =
            candidate.sections.as_deref().and_then(|s| s.gwas.as_ref())
        else {
            continue;
        };
        if let Some(query) = &data.continuation {
            let action = Action::Run {
                request: Request::GetVariant(query.clone()),
                fresh: false,
            };
            if !next.iter().any(|(_, old)| old == &action) {
                next.push((StepKind::Page, action));
            }
        }
    }
}
