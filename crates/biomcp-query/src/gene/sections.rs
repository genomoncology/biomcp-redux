mod constraint;
mod expression;
use crate::Ctx;
use biodata::{
    Constraint, Expression, Gene, GeneGet, GeneItem, GeneResolution, GeneSection, GeneSectionWord,
    GeneSections, HgncId, Problem, Section, SourceOutcome, SourceState, Status,
};
use biomcp_fetch::{Deadline, RequestError};
pub(super) enum SectionValue {
    Constraint(Section<Constraint>),
    Expression(Section<Expression>),
}
pub(super) struct SectionResult {
    pub values: Vec<(HgncId, SectionValue)>,
    pub source: SourceOutcome,
    pub problems: Vec<Problem>,
    pub local: Option<RequestError>,
}
pub(super) fn requested(query: &GeneGet) -> Vec<GeneSection> {
    [GeneSection::Constraint, GeneSection::Expression]
        .into_iter()
        .filter(|section| {
            query.sections().iter().any(|word| match word {
                GeneSectionWord::All => true,
                GeneSectionWord::Constraint => *section == GeneSection::Constraint,
                GeneSectionWord::Expression => *section == GeneSection::Expression,
                _ => false,
            })
        })
        .collect()
}

fn applicable(section: GeneSection, gene: &Gene) -> bool {
    match section {
        GeneSection::Constraint => constraint::applicable(gene),
        GeneSection::Expression => expression::applicable(gene),
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
        item.more = [GeneSection::Constraint, GeneSection::Expression]
            .into_iter()
            .filter(|section| !requested.contains(section) && applicable(*section, gene))
            .collect();
        if !requested.is_empty() {
            item.sections = Some(GeneSections {
                expression: requested
                    .contains(&GeneSection::Expression)
                    .then_some(Section::NotApplicable),
                constraint: requested
                    .contains(&GeneSection::Constraint)
                    .then_some(Section::NotApplicable),
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
    if requested(query).contains(&GeneSection::Expression)
        && genes.iter().any(|(_, gene)| expression::applicable(gene))
    {
        jobs.push(Box::new(move || {
            super::get::Job::Section(expression::run(cx, deadline, genes))
        }));
    }
    jobs
}
pub(super) fn join(items: &mut [GeneItem], result: &SectionResult) {
    for item in items {
        let Some(GeneResolution::Resolved { gene, .. }) = &item.answer else {
            continue;
        };
        let Some((_, value)) = result.values.iter().find(|(id, _)| *id == gene.hgnc_id) else {
            continue;
        };
        let unavailable = match value {
            SectionValue::Constraint(value) => matches!(value, Section::Unavailable { .. }),
            SectionValue::Expression(value) => matches!(value, Section::Unavailable { .. }),
        };
        if let Some(sections) = &mut item.sections {
            match value {
                SectionValue::Constraint(value) => sections.constraint = Some(value.clone()),
                SectionValue::Expression(value) => sections.expression = Some(value.clone()),
            }
        }
        if unavailable
            || result.source.state == SourceState::Stale
            || result.problems.iter().any(|problem| {
                problem.record.as_ref().is_some_and(|record| {
                    gene.ensembl_id
                        .as_ref()
                        .is_some_and(|id| record.as_str() == id.base())
                })
            })
        {
            item.status = Status::Partial;
        }
    }
}