//! Candidate-specific section disclosure under one shared command deadline.
use crate::Ctx;
use biodata::{
    AssertionSource, Problem, Section, SourceOutcome, VariantGet, VariantHeader, VariantSection,
    VariantSections,
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
        macro_rules! attach {
            ($field:ident,$read:expr) => {
                sections.$field = Some(retain($read, &mut sources, &mut problems, &mut local))
            };
        }
        for section in requested(query) {
            let candidates = std::slice::from_ref(&*candidate);
            use VariantSection as S;
            match section {
                S::Clinvar | S::Erepo => {
                    let source = if section == S::Clinvar {
                        AssertionSource::Clinvar
                    } else {
                        AssertionSource::Erepo
                    };
                    let result = retain(
                        super::assertions::section(cx, query, candidates, deadline, source),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    );
                    if section == S::Clinvar {
                        sections.clinvar = Some(result)
                    } else {
                        sections.erepo = Some(result)
                    }
                }
                S::Population => attach!(
                    population,
                    super::population::section(cx, query, candidates, deadline)
                ),
                S::Gwas => attach!(gwas, super::gwas::section(cx, query, candidates, deadline)),
                S::Civic => attach!(
                    civic,
                    super::civic::section(cx, query, candidates, deadline)
                ),
                S::Hotspots => attach!(
                    hotspots,
                    super::hotspots::section(cx, query, candidates, deadline)
                ),
                S::Predict => attach!(
                    predict,
                    super::predict::section(cx, query, candidates, deadline)
                ),
                S::Conservation => attach!(
                    conservation,
                    super::conservation::section(cx, query, candidates, deadline)
                ),
                S::Cosmic => attach!(
                    cosmic,
                    super::cosmic::section(cx, query, candidates, deadline)
                ),
                S::Oncokb => attach!(
                    oncokb,
                    super::oncokb::section(cx, query, candidates, deadline)
                ),
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
