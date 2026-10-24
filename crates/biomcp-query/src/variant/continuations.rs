//! Spell each provider's typed continuation without changing source identity or page selectors.
use crate::Action;
use biodata::{Request, Section, StepKind, VariantAssertions, VariantHeader};
pub(super) fn collect(candidates: &[VariantHeader], next: &mut Vec<(StepKind, Action)>) {
    for candidate in candidates {
        let Some(sections) = candidate.sections.as_deref() else {
            continue;
        };
        for query in [
            data(&sections.gwas).and_then(|d| d.continuation.as_ref()),
            data(&sections.civic).and_then(|d| d.next.as_ref()),
            data(&sections.hotspots).and_then(|d| d.next.as_ref()),
            data(&sections.cosmic).and_then(|d| d.next.as_ref()),
            data(&sections.oncokb).and_then(|d| d.next.as_ref()),
        ]
        .into_iter()
        .flatten()
        {
            push(next, StepKind::Page, Request::GetVariant(query.clone()));
        }
        if let Some(civic) = data(&sections.civic) {
            for profile in &civic.profiles {
                for query in [&profile.evidence_page.next, &profile.assertion_page.next]
                    .into_iter()
                    .flatten()
                {
                    push(next, StepKind::Page, Request::GetVariant(query.clone()));
                }
            }
        }
        for assertions in [data(&sections.clinvar), data(&sections.erepo)]
            .into_iter()
            .flatten()
        {
            assertion_next(assertions, next);
        }
    }
}
fn data<T>(section: &Option<Section<T>>) -> Option<&T> {
    match section.as_ref()? {
        Section::Data { data, .. } => Some(data),
        _ => None,
    }
}
fn assertion_next(assertions: &VariantAssertions, next: &mut Vec<(StepKind, Action)>) {
    if let Some(query) = &assertions.next {
        push(
            next,
            StepKind::Page,
            Request::VariantAssertions(query.clone()),
        );
    }
}
fn push(next: &mut Vec<(StepKind, Action)>, kind: StepKind, request: Request) {
    let action = Action::Run {
        request,
        fresh: false,
    };
    if !next.iter().any(|(_, old)| old == &action) {
        next.push((kind, action));
    }
}
