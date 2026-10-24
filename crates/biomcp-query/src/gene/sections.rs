mod civic;
mod constraint;
use crate::Ctx;
use biodata::{
    Constraint, Gene, GeneGet, GeneItem, GeneResolution, GeneSection, GeneSectionWord,
    GeneSections, HgncId, Problem, Section, SourceOutcome, SourceState, Status,
};
use biomcp_fetch::{Deadline, RequestError};
pub(super) struct SectionResult {
    pub section: GeneSection,
    pub values: Vec<(HgncId, SectionValue)>,
    pub source: SourceOutcome,
    pub problems: Vec<Problem>,
    pub local: Option<RequestError>,
}
pub(super) enum SectionValue {
    Constraint(Section<Constraint>),
    Civic(Section<biodata::CivicGene>),
}
pub(super) fn requested(query: &GeneGet) -> Vec<GeneSection> {
    [GeneSection::Constraint, GeneSection::Civic]
        .into_iter()
        .filter(|section| {
            query.sections().iter().any(|word| match word {
                GeneSectionWord::All => true,
                GeneSectionWord::Constraint => *section == GeneSection::Constraint,
                GeneSectionWord::Civic => *section == GeneSection::Civic,
                _ => false,
            })
        })
        .collect()
}

fn applicable(section: GeneSection, gene: &Gene) -> bool {
    match section {
        GeneSection::Constraint => constraint::applicable(gene),
        GeneSection::Civic => true,
        _ => false,
    }
}
pub(super) fn prepare(query: &GeneGet, items: &mut [GeneItem]) -> Vec<(HgncId, Gene)> {
    let requested = requested(query);
    let mut genes = Vec::new();
    for item in items {
        let Some(GeneResolution::Resolved { gene, .. }) = &item.answer else {
            continue;
        };
        item.more = [GeneSection::Constraint, GeneSection::Civic]
            .into_iter()
            .filter(|section| !requested.contains(section) && applicable(*section, gene))
            .collect();
        if !requested.is_empty() {
            item.sections = Some(GeneSections {
                constraint: requested
                    .contains(&GeneSection::Constraint)
                    .then_some(Section::NotApplicable),
                civic: requested
                    .contains(&GeneSection::Civic)
                    .then_some(Section::Empty {
                        source: biodata::Source::Civic,
                        record: None,
                    }),
            });
        }
        if !genes.iter().any(|(id, _)| *id == gene.hgnc_id) {
            genes.push((gene.hgnc_id, gene.as_ref().clone()));
        }
    }
    genes
}
pub(super) fn jobs<'a>(
    cx: &'a Ctx<'a>,
    deadline: Deadline<'a>,
    query: &GeneGet,
    genes: &'a [(HgncId, Gene)],
) -> Vec<Box<dyn FnOnce() -> super::get::Job + Send + 'a>> {
    let mut jobs: Vec<Box<dyn FnOnce() -> super::get::Job + Send + 'a>> = Vec::new();
    if requested(query).contains(&GeneSection::Constraint)
        && genes.iter().any(|(_, gene)| constraint::applicable(gene))
    {
        jobs.push(Box::new(move || {
            super::get::Job::Section(constraint::run(cx, deadline, genes))
        }));
    }
    if requested(query).contains(&GeneSection::Civic) && !genes.is_empty() {
        jobs.push(Box::new(move || {
            super::get::Job::Section(civic::run(cx, deadline, genes))
        }));
    }
    jobs
}
pub(super) fn join(items: &mut [GeneItem], result: &SectionResult) {
    for item in items {
        let Some(GeneResolution::Resolved { gene, .. }) = &item.answer else {
            continue;
        };
        let Some((_, section)) = result.values.iter().find(|(id, _)| *id == gene.hgnc_id) else {
            continue;
        };
        if matches!(
            section,
            SectionValue::Constraint(Section::Unavailable { .. })
                | SectionValue::Civic(Section::Unavailable { .. })
        ) || result.source.state == SourceState::Stale
            || result.problems.iter().any(|problem| {
                problem.record.as_ref().is_some_and(|record| {
                    (matches!(section, SectionValue::Civic(_))
                        && gene
                            .entrez_id
                            .is_some_and(|id| record.as_str() == id.to_string()))
                        || gene
                            .ensembl_id
                            .as_ref()
                            .is_some_and(|id| record.as_str() == id.base())
                })
            })
        {
            item.status = Status::Partial;
        }
        if let Some(sections) = &mut item.sections {
            match section {
                SectionValue::Constraint(section) if result.section == GeneSection::Constraint => {
                    sections.constraint = Some(section.clone())
                }
                SectionValue::Civic(section) if result.section == GeneSection::Civic => {
                    sections.civic = Some(section.clone())
                }
                _ => {}
            }
        }
    }
}
