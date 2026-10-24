//! Candidate-specific section disclosure under one shared command deadline.
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
pub(super) fn requested(query: &VariantGet) -> Vec<VariantSection> {
    use VariantSection as S;
    [
        S::Clinvar,
        S::Erepo,
        S::Population,
        S::Gwas,
        S::Civic,
        S::Hotspots,
        S::Predict,
        S::Conservation,
        S::Cosmic,
        S::Oncokb,
    ]
    .into_iter()
    .filter(|s| query.sections().contains(s) || query.sections().contains(&S::All))
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
            let candidates = std::slice::from_ref(&*candidate);
            use VariantSection as S;
            match section {
                S::Clinvar => {
                    sections.clinvar = Some(retain(
                        super::assertions::section(
                            cx,
                            query,
                            candidates,
                            deadline,
                            AssertionSource::Clinvar,
                        ),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Erepo => {
                    sections.erepo = Some(retain(
                        super::assertions::section(
                            cx,
                            query,
                            candidates,
                            deadline,
                            AssertionSource::Erepo,
                        ),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Population => {
                    sections.population = Some(retain(
                        super::population::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Gwas => {
                    sections.gwas = Some(retain(
                        super::gwas::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Civic => {
                    sections.civic = Some(retain(
                        super::civic::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Hotspots => {
                    sections.hotspots = Some(retain(
                        super::hotspots::section(cx, query, candidates, deadline),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                S::Predict => sections.predict = Some(pending(Source::MyVariant, &mut sources)),
                S::Conservation => {
                    sections.conservation = Some(pending(Source::MyVariant, &mut sources))
                }
                S::Cosmic => sections.cosmic = Some(pending(Source::MyVariant, &mut sources)),
                S::Oncokb => sections.oncokb = Some(pending(Source::Oncokb, &mut sources)),
                S::All => {}
                _ => {}
            }
        }
        if !query.sections().is_empty() {
            candidate.sections = Some(Box::new(sections));
        }
    }
    (sources, problems, local)
}
fn pending<T>(source: Source, sources: &mut Vec<SourceOutcome>) -> Section<T> {
    sources.push(SourceOutcome {
        source,
        state: SourceState::Failed,
        reason: Some(Reason::Unavailable),
        term: None,
        count: None,
        fetched: None,
        notices: Vec::new(),
    });
    Section::unavailable(source, Reason::Unavailable)
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
