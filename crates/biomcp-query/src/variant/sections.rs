//! Candidate-specific source sections under one command deadline.
use crate::Ctx;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    AssertionSource, Problem, Reason, Section, Source, SourceOutcome, SourceState, VariantGet,
    VariantHeader, VariantSection, VariantSections,
};
pub(super) type Result<T> = (
    Section<T>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
    bool,
);
pub(super) fn apply(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &mut [VariantHeader],
    deadline: &mut Deadline<'_>,
    population_reason: Option<Reason>,
) -> (Vec<SourceOutcome>, Vec<Problem>, Option<RequestError>, bool) {
    let mut collected = Collected::default();
    for candidate in candidates {
        let mut sections = VariantSections::default();
        for section in selected(query) {
            let candidates = std::slice::from_ref(&*candidate);
            macro_rules! attach {
                ($field:ident, $read:expr) => {
                    sections.$field = Some(collected.keep(permitted(cx, section, || $read)))
                };
            }
            match section {
                VariantSection::Clinvar => attach!(
                    clinvar,
                    super::assertions::section(
                        cx,
                        query,
                        candidates,
                        *deadline,
                        AssertionSource::Clinvar
                    )
                ),
                VariantSection::Erepo => attach!(
                    erepo,
                    super::assertions::section(
                        cx,
                        query,
                        candidates,
                        *deadline,
                        AssertionSource::Erepo
                    )
                ),
                VariantSection::Predict => attach!(
                    predict,
                    super::predict::section(cx, query, candidates, deadline)
                ),
                VariantSection::Conservation => attach!(
                    conservation,
                    super::conservation::section(cx, query, candidates, deadline)
                ),
                VariantSection::Cosmic => attach!(
                    cosmic,
                    super::cosmic::section(cx, query, candidates, deadline)
                ),
                VariantSection::Population => attach!(
                    population,
                    super::population::section(cx, query, candidates, *deadline, population_reason)
                ),
                VariantSection::Gwas => {
                    attach!(gwas, super::gwas::section(cx, query, candidates, *deadline))
                }
                VariantSection::Civic => attach!(
                    civic,
                    super::civic::section(cx, query, candidates, *deadline)
                ),
                VariantSection::Hotspots => attach!(
                    hotspots,
                    super::hotspots::section(cx, query, candidates, *deadline)
                ),
                VariantSection::AlphaGenome => attach!(
                    alphagenome,
                    super::alphagenome::section(cx, query, candidates, *deadline)
                ),
                VariantSection::Oncokb => attach!(
                    oncokb,
                    super::oncokb::section(cx, query, candidates, *deadline)
                ),
                _ => {}
            }
        }
        if !query.sections().is_empty() {
            candidate.sections = Some(Box::new(sections));
        }
    }
    (collected.sources, collected.problems, collected.local, collected.incomplete)
}
#[derive(Default)]
struct Collected {
    sources: Vec<SourceOutcome>,
    problems: Vec<Problem>,
    local: Option<RequestError>,
    incomplete: bool,
}
impl Collected {
    fn keep<T>(&mut self, value: Result<T>) -> Section<T> {
        retain(
            value,
            &mut self.sources,
            &mut self.problems,
            &mut self.local,
            &mut self.incomplete,
        )
    }
}
pub(super) fn source(section: VariantSection) -> Option<Source> {
    match section {
        VariantSection::Clinvar => Some(Source::ClinVar),
        VariantSection::Erepo => Some(Source::Erepo),
        VariantSection::Population => Some(Source::Gnomad),
        VariantSection::Gwas => Some(Source::Gwas),
        VariantSection::Civic => Some(Source::Civic),
        VariantSection::Oncokb => Some(Source::Oncokb),
        VariantSection::AlphaGenome => Some(Source::AlphaGenome),
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
            vec![biomcp_adapters::registry::outcome(
                source,
                SourceState::Skipped,
                Some(Reason::Disabled),
            )],
            Vec::new(),
            None,
            false,
        );
    }
    read()
}
pub(super) fn retain<T>(
    value: Result<T>,
    sources: &mut Vec<SourceOutcome>,
    problems: &mut Vec<Problem>,
    local: &mut Option<RequestError>,
    incomplete: &mut bool,
) -> Section<T> {
    let (section, more, issues, error, dropped) = value;
    *incomplete |= dropped || matches!(section, Section::Unavailable { .. });
    sources.extend(more);
    problems.extend(issues);
    *local = local.or(error);
    section
}

fn selected(query: &VariantGet) -> impl Iterator<Item = VariantSection> + '_ {
    [
        VariantSection::Clinvar,
        VariantSection::Erepo,
        VariantSection::Population,
        VariantSection::Gwas,
        VariantSection::Civic,
        VariantSection::Hotspots,
        VariantSection::Oncokb,
        VariantSection::AlphaGenome,
        VariantSection::Predict,
        VariantSection::Conservation,
        VariantSection::Cosmic,
    ]
    .into_iter()
    .filter(|section| {
        query.sections().contains(section) || query.sections().contains(&VariantSection::All)
    })
}
