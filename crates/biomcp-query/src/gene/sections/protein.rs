use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId, UniProtAcc};
use biomcp_adapters::{Mapped, uniprot};
use biomcp_fetch::{Deadline, Leg, RequestError};
use biomcp_types::{GeneSection, Protein, Section, SourceRecord};

pub(super) fn applicable(gene: &Gene) -> bool {
    !gene.uniprot.is_empty() || crate::gene::needs_uniprot_lookup(gene)
}

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, genes: &[(HgncId, Gene)]) -> SectionResult {
    let ids: Vec<_> = genes
        .iter()
        .flat_map(|(_, gene)| gene.uniprot.iter().cloned())
        .collect();
    let leg = cx.fetch.leg(&uniprot::SPEC, deadline).fresh(cx.fresh);
    let linked = uniprot::proteins(&leg, &ids);
    let mut error = linked.as_ref().err().copied();
    let mut local = error.and_then(status::local);
    let mut dropped = false;
    let mut problems = Vec::new();
    if let Ok(mapped) = &linked {
        dropped = mapped.refusals == biomcp_adapters::Refusal::Refused;
        status::append(&mut problems, mapped.problems.clone());
    }
    let values = genes
        .iter()
        .filter(|(_, gene)| applicable(gene))
        .map(|(id, gene)| {
            let section = if crate::gene::needs_uniprot_lookup(gene) {
                match find(&leg, gene, &mut problems) {
                    Ok(mapped) => {
                        dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                        status::append(&mut problems, mapped.problems);
                        rows(mapped.value, None)
                    }
                    Err(failure) => {
                        error.get_or_insert(failure);
                        local = local.or(status::local(failure));
                        unavailable(failure)
                    }
                }
            } else {
                match &linked {
                    Ok(mapped) => rows(
                        gene.uniprot
                            .iter()
                            .filter_map(|id| {
                                mapped
                                    .value
                                    .iter()
                                    .find(|protein| protein.accession == *id)
                                    .cloned()
                            })
                            .collect(),
                        gene.uniprot.first(),
                    ),
                    Err(failure) => unavailable(*failure),
                }
            };
            (*id, SectionValue::Protein(section))
        })
        .collect();
    SectionResult {
        dropped,
        values,
        source: status::source(GeneSection::Protein.source(), leg.finish(), error),
        problems,
        local,
    }
}

fn find(
    leg: &Leg<'_>,
    gene: &Gene,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<Mapped<Vec<Protein>>, RequestError> {
    let candidates = uniprot::gene_candidates(leg, &gene.symbol)?;
    status::append(problems, candidates.problems);
    uniprot::proteins(leg, &candidates.value)
}

fn rows(data: Vec<Protein>, asked: Option<&UniProtAcc>) -> Section<Box<[Protein]>> {
    let source = GeneSection::Protein.source();
    if data.is_empty() {
        Section::Empty {
            source,
            record: asked.and_then(|id| SourceRecord::asked(Some("UniProt"), id)),
        }
    } else {
        Section::Data {
            source,
            data: data.into_boxed_slice(),
        }
    }
}

fn unavailable(error: RequestError) -> Section<Box<[Protein]>> {
    Section::unavailable(
        GeneSection::Protein.source(),
        status::reason(error).unwrap_or(biomcp_types::Reason::Unavailable),
    )
}
