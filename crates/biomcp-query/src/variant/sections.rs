//! Explicit source assertion selection under the command's shared deadline.
use crate::Ctx;
use biodata::{
    AssertionSource, Problem, Reason, Section, Source, SourceOutcome, SourceState, VariantGet,
    VariantHeader, VariantSection, VariantSections,
};
use biomcp_fetch::{Deadline, RequestError};
pub(super) type Result<T> = (
    Section<T>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
);
pub(super) fn source(section: VariantSection) -> Option<Source> {
    match section {
        VariantSection::Clinvar => Some(Source::ClinVar),
        VariantSection::Erepo => Some(Source::Erepo),
        _ => None,
    }
}
pub(super) fn apply(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &mut [VariantHeader],
    deadline: Deadline<'_>,
) -> (Vec<SourceOutcome>, Vec<Problem>, Option<RequestError>) {
    let mut sources = Vec::new();
    let mut problems = Vec::new();
    let mut local = None;
    if query.sections().is_empty() {
        return (sources, problems, local);
    }
    for candidate in candidates {
        let mut sections = VariantSections::default();
        for (selector, assertion_source) in [
            (VariantSection::Clinvar, AssertionSource::Clinvar),
            (VariantSection::Erepo, AssertionSource::Erepo),
        ] {
            if !query.sections().contains(&selector)
                && !query.sections().contains(&VariantSection::All)
            {
                continue;
            }
            let value = permitted(cx, assertion_source.source(), || {
                super::assertions::section(
                    cx,
                    query,
                    std::slice::from_ref(candidate),
                    deadline,
                    assertion_source,
                )
            });
            let section = retain(value, &mut sources, &mut problems, &mut local);
            if selector == VariantSection::Clinvar {
                sections.clinvar = Some(section)
            } else {
                sections.erepo = Some(section)
            }
        }
        candidate.sections = Some(Box::new(sections));
    }
    (sources, problems, local)
}
fn permitted<T>(cx: &Ctx<'_>, source: Source, read: impl FnOnce() -> Result<T>) -> Result<T> {
    if !cx.fetch.disabled(source) {
        return read();
    }
    (
        Section::Skipped {
            source,
            reason: Reason::Disabled,
        },
        vec![SourceOutcome {
            source,
            state: SourceState::Skipped,
            reason: Some(Reason::Disabled),
            term: None,
            count: None,
            fetched: None,
            notices: Vec::new(),
        }],
        Vec::new(),
        None,
    )
}
pub(super) fn retain<T>(
    value: Result<T>,
    sources: &mut Vec<SourceOutcome>,
    problems: &mut Vec<Problem>,
    local: &mut Option<RequestError>,
) -> Section<T> {
    let (section, more, issues, error) = value;
    sources.extend(more);
    problems.extend(issues);
    *local = local.or(error);
    section
}
