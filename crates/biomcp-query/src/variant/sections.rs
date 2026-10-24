//! Candidate-specific source sections under one command deadline.
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
pub(super) fn apply(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &mut [VariantHeader],
    deadline: Deadline<'_>,
) -> (Vec<SourceOutcome>, Vec<Problem>, Option<RequestError>) {
    let (mut sources, mut problems, mut local) = (Vec::new(), Vec::new(), None);
    for candidate in candidates {
        let mut sections = VariantSections::default();
        for section in selected(query) {
            let candidates = std::slice::from_ref(&*candidate);
            macro_rules! attach {
                ($field:ident, $read:expr) => {
                    sections.$field = Some(retain(
                        permitted(cx, section, || $read),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                };
            }
            match section {
                VariantSection::Clinvar => attach!(
                    clinvar,
                    super::assertions::section(
                        cx,
                        query,
                        candidates,
                        deadline,
                        AssertionSource::Clinvar
                    )
                ),
                VariantSection::Erepo => attach!(
                    erepo,
                    super::assertions::section(
                        cx,
                        query,
                        candidates,
                        deadline,
                        AssertionSource::Erepo
                    )
                ),
                VariantSection::Population => attach!(
                    population,
                    super::population::section(cx, query, candidates, deadline)
                ),
                VariantSection::Gwas => {
                    attach!(gwas, super::gwas::section(cx, query, candidates, deadline))
                }
                VariantSection::Civic => attach!(
                    civic,
                    super::civic::section(cx, query, candidates, deadline)
                ),
                VariantSection::Hotspots => attach!(
                    hotspots,
                    super::hotspots::section(cx, query, candidates, deadline)
                ),
                VariantSection::Oncokb => attach!(
                    oncokb,
                    super::oncokb::section(cx, query, candidates, deadline)
                ),
                _ => {}
            }
        }
        if !query.sections().is_empty() {
            candidate.sections = Some(Box::new(sections));
        }
    }
    (sources, problems, local)
}
fn selected(query: &VariantGet) -> impl Iterator<Item = VariantSection> + '_ {
    [
        VariantSection::Population,
        VariantSection::Gwas,
        VariantSection::Clinvar,
        VariantSection::Erepo,
        VariantSection::Civic,
        VariantSection::Hotspots,
        VariantSection::Oncokb,
    ]
    .into_iter()
    .filter(|section| {
        query.sections().contains(section) || query.sections().contains(&VariantSection::All)
    })
}
pub(super) fn source(section: VariantSection) -> Option<Source> {
    match section {
        VariantSection::Population => Some(Source::Gnomad),
        VariantSection::Gwas => Some(Source::Gwas),
        VariantSection::Clinvar => Some(Source::ClinVar),
        VariantSection::Erepo => Some(Source::Erepo),
        VariantSection::Civic => Some(Source::Civic),
        VariantSection::Oncokb => Some(Source::Oncokb),
        VariantSection::Hotspots => Some(Source::CancerHotspots),
        _ => None,
    }
}
fn permitted<T>(
    cx: &Ctx<'_>,
    section: VariantSection,
    read: impl FnOnce() -> Result<T>,
) -> Result<T> {
    if let Some(source) = source(section).filter(|source| cx.fetch.disabled(*source)) {
        return (
            if matches!(section, VariantSection::Clinvar | VariantSection::Erepo) {
                Section::Skipped {
                    source,
                    reason: Reason::Disabled,
                }
            } else {
                Section::unavailable(source, Reason::Disabled)
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
        );
    }
    read()
}
fn retain<T>(
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
