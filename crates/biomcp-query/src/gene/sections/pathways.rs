use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};

use biomcp_adapters::{
    Mapped,
    reactome::{self, MappingId},
};
use biomcp_fetch::{Deadline, Leg, RequestError};
use biomcp_types::{Capped, GeneSection, PathwayRef, Reason, Section, SourceRecord, SourceStatus};
const LIMIT: usize = 25;

pub(in crate::gene) type Mapping = (HgncId, Result<Vec<MappingId>, RequestError>);

pub(in crate::gene) fn mappings(
    identity: &Leg<'_>,
    genes: &[(HgncId, Gene)],
    hgnc_error: &mut Option<RequestError>,
    local: &mut Option<RequestError>,
) -> Vec<Mapping> {
    genes
        .iter()
        .map(|(id, gene)| {
            let keys = MappingId::for_gene(identity, gene).inspect_err(|error| {
                hgnc_error.get_or_insert(*error);
                *local = local.or_else(|| status::local(*error));
            });
            (*id, keys)
        })
        .collect()
}

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, mappings: &[Mapping]) -> SectionResult {
    let mut ids = Vec::new();
    for id in mappings.iter().flat_map(|(_, keys)| keys.iter().flatten()) {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    let leg = cx.fetch.leg(&reactome::SPEC, deadline).fresh(cx.fresh);
    let results = leg.batch(&ids, |leg, id| reactome::pathways(leg, id));
    let error = results
        .iter()
        .find_map(|result| result.as_ref().err().copied());
    let local = results
        .iter()
        .find_map(|result| result.as_ref().err().copied().and_then(status::local));
    let mut source = status::source(GeneSection::Pathways.source(), leg.finish(), error);
    if ids.is_empty() {
        source.set_status(SourceStatus::Skipped(Some(Reason::NoJoinId)));
    }
    let mut dropped = false;
    let mut problems = Vec::new();
    let reads: Vec<_> = ids.into_iter().zip(results).collect();
    let values = mappings
        .iter()
        .map(|(id, keys)| {
            let section = keys.as_ref().map_or_else(
                |_| Section::unavailable(GeneSection::Pathways.source(), Reason::NoJoinId),
                |keys| section(keys, &reads, &mut dropped, &mut problems),
            );
            (*id, SectionValue::Pathways(section))
        })
        .collect();
    SectionResult {
        dropped,
        values,
        source,
        problems,
        local,
    }
}

type Read<'a> = (&'a MappingId, Result<Mapped<Vec<PathwayRef>>, RequestError>);

fn section(
    keys: &[MappingId],
    reads: &[Read<'_>],
    dropped: &mut bool,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Section<Capped<PathwayRef>> {
    let mut pathways = Vec::new();
    let mut failure = None;
    for key in keys {
        let Some((_, result)) = reads.iter().find(|(id, _)| *id == key) else {
            continue;
        };
        match result {
            Ok(mapped) => {
                *dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                status::append(problems, mapped.problems.clone());
                for pathway in &mapped.value {
                    if !pathways
                        .iter()
                        .any(|item: &biomcp_types::PathwayRef| item.id == pathway.id)
                    {
                        pathways.push(pathway.clone());
                    }
                }
            }
            Err(error) => {
                failure.get_or_insert(
                    crate::status::reason(*error).unwrap_or(biomcp_types::Reason::Unavailable),
                );
            }
        }
    }
    if let Some(reason) = failure {
        Section::unavailable(GeneSection::Pathways.source(), reason)
    } else if pathways.is_empty() {
        Section::Empty {
            source: GeneSection::Pathways.source(),
            record: keys.first().and_then(|key| {
                let (namespace, id) = key.parts();
                SourceRecord::asked(Some(namespace), id)
            }),
        }
    } else {
        let total = u64::try_from(pathways.len()).unwrap_or(u64::MAX);
        pathways.truncate(LIMIT);
        Section::Data {
            source: GeneSection::Pathways.source(),
            data: Capped {
                items: pathways.into(),
                total,
            },
        }
    }
}
