use super::SectionResult;
use crate::{Ctx, status};
use biodata::{Gene, HgncId, LocusGroup};

use biomcp_adapters::gnomad;
use biomcp_fetch::Deadline;
use biomcp_types::{ConstraintRelease, GeneSection, Reason, Section, SourceNotice, SourceRecord};

pub(super) fn applicable(gene: &Gene) -> bool {
    gene.locus_group == Some(LocusGroup::ProteinCoding) && gene.ensembl_id.is_some()
}
pub(super) fn run(
    cx: &Ctx<'_>,
    deadline: Deadline<'_>,
    genes: &[(HgncId, Gene)],
    release: Option<ConstraintRelease>,
) -> SectionResult {
    if let Some(release) = release {
        return from_release(cx, deadline, genes, release);
    }
    let ids: Vec<_> = genes
        .iter()
        .filter(|(_, gene)| applicable(gene))
        .filter_map(|(_, gene)| gene.ensembl_id.clone())
        .collect();
    let leg = cx.fetch.leg(&gnomad::SPEC, deadline).fresh(cx.fresh);
    let result = gnomad::constraints(&leg, &ids);
    let error = result.as_ref().err().copied();
    let source = status::source(GeneSection::Constraint.source(), leg.finish(), error);
    let mut dropped = false;
    let mut problems = Vec::new();
    let mut sections = Vec::new();
    for (id, gene) in genes.iter().filter(|(_, gene)| applicable(gene)) {
        let section = match &result {
            Ok(values) => {
                let value = values.iter().find(|value| {
                    gene.ensembl_id
                        .as_ref()
                        .is_some_and(|id| id.base() == value.id.base())
                });
                match value {
                    Some(value) => {
                        dropped |= value.constraint.refusals == biomcp_adapters::Refusal::Refused;
                        status::append(&mut problems, value.constraint.problems.clone());
                        if value
                            .constraint
                            .problems
                            .iter()
                            .any(|problem| problem.field.as_ref() == "gene_id")
                        {
                            Section::unavailable(GeneSection::Constraint.source(), Reason::BadReply)
                        } else if let Some(data) = &value.constraint.value {
                            Section::Data {
                                source: GeneSection::Constraint.source(),
                                data: data.clone(),
                            }
                        } else {
                            Section::Empty {
                                source: GeneSection::Constraint.source(),
                                record: gene
                                    .ensembl_id
                                    .as_ref()
                                    .and_then(|id| SourceRecord::asked(Some("Ensembl"), id.base())),
                            }
                        }
                    }
                    None => {
                        Section::unavailable(GeneSection::Constraint.source(), Reason::BadReply)
                    }
                }
            }
            Err(error) => Section::unavailable(
                GeneSection::Constraint.source(),
                crate::status::reason(*error).unwrap_or(biomcp_types::Reason::Unavailable),
            ),
        };
        sections.push((*id, super::SectionValue::Constraint(section)));
    }
    SectionResult {
        dropped,
        values: sections,
        source,
        problems,
        local: error.and_then(status::local),
    }
}

fn from_release(
    cx: &Ctx<'_>,
    deadline: Deadline<'_>,
    genes: &[(HgncId, Gene)],
    release: ConstraintRelease,
) -> SectionResult {
    let spec = &gnomad::release::SPEC;
    let leg = cx.fetch.leg(spec, deadline);
    let genes: Vec<_> = genes.iter().filter(|(_, gene)| applicable(gene)).collect();
    let asked: Vec<_> = genes.iter().map(|(_, gene)| gene).collect();
    let result = gnomad::release::constraints(&leg, release, &asked);
    let error = result.as_ref().err().copied();
    let mut source = status::source(spec.source, leg.finish(), error);
    let values = match result {
        Ok(rows) => {
            let missing = genes
                .iter()
                .zip(&rows)
                .filter(|(_, row)| row.is_none())
                .map(|((_, gene), _)| gene.symbol.as_str())
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                source.notices.push(SourceNotice::RequestGuidance {
                    data_date: None,
                    message: format!(
                        "gnomAD {release} has no unique MANE Select constraint row for {}.",
                        missing.join(", ")
                    )
                    .into(),
                    retry_after_seconds: None,
                });
            }
            genes
                .iter()
                .zip(rows)
                .map(|((id, _), row)| {
                    let section = row.map_or_else(
                        || Section::unavailable(spec.source, Reason::NotStated),
                        |data| Section::Data {
                            source: spec.source,
                            data,
                        },
                    );
                    (*id, super::SectionValue::Constraint(section))
                })
                .collect()
        }
        Err(error) => {
            let reason = status::reason(error).unwrap_or(Reason::Unavailable);
            if matches!(
                reason,
                Reason::TooLarge | Reason::Syncing | Reason::NotFound | Reason::Offline
            ) {
                let bundle = crate::Bundle::gnomad_constraint(release);
                source.notices.push(SourceNotice::RequestGuidance {
                    data_date: None,
                    message: format!("gnomAD {release} constraint table is unavailable locally. Run biomcp sync {}.", bundle.name()).into(),
                    retry_after_seconds: None,
                });
            }
            genes
                .iter()
                .map(|(id, _)| {
                    (
                        *id,
                        super::SectionValue::Constraint(Section::unavailable(spec.source, reason)),
                    )
                })
                .collect()
        }
    };
    SectionResult {
        dropped: false,
        values,
        source,
        problems: Vec::new(),
        local: error.and_then(status::local),
    }
}
