//! Candidate-specific source sections under one command deadline.
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
        for section in [
            VariantSection::Population,
            VariantSection::Gwas,
            VariantSection::Civic,
            VariantSection::Hotspots,
            VariantSection::Oncokb,
        ] {
            if !query.sections().contains(&section)
                && !query.sections().contains(&VariantSection::All)
            {
                continue;
            }
            let candidates = std::slice::from_ref(&*candidate);
            match section {
                VariantSection::Population => sections.population = Some(retain(permitted(cx, section, || super::population::section(cx, query, candidates, deadline)), &mut sources, &mut problems, &mut local)),
                VariantSection::Gwas => sections.gwas = Some(retain(permitted(cx, section, || super::gwas::section(cx, query, candidates, deadline)), &mut sources, &mut problems, &mut local)),
                VariantSection::Civic => {
                    sections.civic = Some(retain(
                        permitted(cx, section, || {
                            super::civic::section(cx, query, candidates, deadline)
                        }),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                VariantSection::Hotspots => {
                    sections.hotspots = Some(retain(
                        permitted(cx, section, || {
                            super::hotspots::section(cx, query, candidates, deadline)
                        }),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ))
                }
                VariantSection::Oncokb => {
                    sections.oncokb = Some(retain(
                        permitted(cx, section, || {
                            super::oncokb::section(cx, query, candidates, deadline)
                        }),
                        &mut sources,
                        &mut problems,
                        &mut local,
                    ));
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
pub(super) fn source(section: VariantSection) -> Option<Source> {
    match section {
        VariantSection::Population => Some(Source::Gnomad),
        VariantSection::Gwas => Some(Source::Gwas),
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
            Section::unavailable(source, Reason::Disabled),
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
