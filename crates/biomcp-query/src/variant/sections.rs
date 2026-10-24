//! Candidate-specific section disclosure under one shared command deadline.
use crate::Ctx;
use biodata::{
    Problem, Reason, Section, Source, SourceOutcome, SourceState, VariantGet, VariantHeader,
    VariantSection, VariantSections,
};
use biomcp_fetch::{Deadline, RequestError};
pub(super) type Result<T> = (
    Section<T>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
);
pub(super) fn requested(query: &VariantGet) -> Vec<VariantSection> {
    use VariantSection as S;
    [S::Population, S::Gwas]
        .into_iter()
        .filter(|s| query.sections().contains(s))
        .collect()
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
    for candidate in candidates {
        let mut sections = VariantSections::default();
        for section in requested(query) {
            macro_rules! attach {
                ($field:ident,$read:expr) => {
                    sections.$field = Some(retain(
                        permitted(cx, section, || $read),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                };
            }
            let candidates = std::slice::from_ref(&*candidate);
            use VariantSection as S;
            match section {
                S::Population => attach!(
                    population,
                    super::population::section(cx, query, candidates, deadline)
                ),
                S::Gwas => attach!(gwas, super::gwas::section(cx, query, candidates, deadline)),
                _ => {}
            }
        }
        if !query.sections().is_empty() {
            candidate.sections = Some(Box::new(sections));
        }
    }
    (sources, problems, local)
}
pub(super) fn source(section: VariantSection) -> Option<Source> {
    use VariantSection as S;
    Some(match section {
        S::Population => Source::Gnomad,
        S::Gwas => Source::Gwas,
        _ => return None,
    })
}

fn permitted<T>(
    cx: &Ctx<'_>,
    section: VariantSection,
    read: impl FnOnce() -> Result<T>,
) -> Result<T> {
    if let Some(source) = source(section).filter(|source| cx.fetch.disabled(*source)) {
        return unavailable(source, Reason::Disabled, SourceState::Skipped);
    }
    read()
}
fn unavailable<T>(source: Source, reason: Reason, state: SourceState) -> Result<T> {
    (
        Section::unavailable(source, reason),
        vec![SourceOutcome {
            source,
            state,
            reason: Some(reason),
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
