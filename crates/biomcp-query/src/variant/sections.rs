//! Candidate-specific section disclosure under one shared command deadline.
use crate::Ctx;
use biodata::{
    Problem, Section, SourceOutcome, VariantGet, VariantHeader, VariantSection, VariantSections,
};
use biomcp_fetch::{Deadline, RequestError};
pub(super) type Result<T> = (
    Section<T>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
);
pub(super) fn apply(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &mut [VariantHeader],
    deadline: Deadline<'_>,
) -> (Vec<SourceOutcome>, Vec<Problem>, Option<RequestError>) {
    let mut sources = Vec::new();
    let mut problems = Vec::new();
    let mut local = None;
    for candidate in candidates {
        let mut sections = VariantSections::default();
        for section in query.sections() {
            let candidates = std::slice::from_ref(&*candidate);
            use VariantSection as S;
            match section {
                S::Predict => {
                    sections.predict = Some(retain(
                        super::predict::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Conservation => {
                    sections.conservation = Some(retain(
                        super::conservation::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Cosmic => {
                    sections.cosmic = Some(retain(
                        super::cosmic::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                _ => {}
            }
        }
        if !query.sections().is_empty() {
            candidate.sections = Some(Box::new(sections));
        }
    }
    (sources, problems, local)
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
