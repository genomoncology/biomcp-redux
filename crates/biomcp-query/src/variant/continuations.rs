//! Assertion pages preserve their native source and selected record/version.
use crate::Action;
use biodata::{Request, Section, StepKind, VariantHeader};
pub(super) fn collect(candidates: &[VariantHeader], next: &mut Vec<(StepKind, Action)>) {
    for candidate in candidates {
        let Some(sections) = candidate.sections.as_deref() else {
            continue;
        };
        for section in [&sections.clinvar, &sections.erepo].into_iter().flatten() {
            if let Section::Data { data, .. } = section {
                if let Some(query) = &data.next {
                    let action = Action::Run {
                        request: Request::VariantAssertions(query.clone()),
                        fresh: false,
                    };
                    if !next.iter().any(|(_, old)| old == &action) {
                        next.push((StepKind::Page, action));
                    }
                }
            }
        }
    }
}
