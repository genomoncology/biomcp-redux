use super::{SectionResult, SectionValue};
use crate::{Ctx, status};
use biodata::{Gene, HgncId};

use biomcp_adapters::reactome::{self, MappingId};
use biomcp_fetch::{Deadline, Leg, RequestError};
use biomcp_types::{Capped, GeneSection, Reason, Section, SourceRecord, SourceStatus};
const LIMIT: usize = 25;

pub(in crate::gene) type Mappings = Vec<(HgncId, Result<Vec<MappingId>, RequestError>)>;

pub(in crate::gene) fn mappings(identity: &Leg<'_>, genes: &[(HgncId, Gene)]) -> Mappings {
    genes
        .iter()
        .map(|(id, gene)| (*id, MappingId::for_gene(identity, gene)))
        .collect()
}

pub(super) fn run(cx: &Ctx<'_>, deadline: Deadline<'_>, mappings: &Mappings) -> SectionResult {
    let mut ids = Vec::new();
    for id in mappings
        .iter()
        .filter_map(|(_, keys)| keys.as_ref().ok())
        .flatten()
    {
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
    let mut values = Vec::new();
    for (id, keys) in mappings {
        let Ok(keys) = keys else {
            values.push((
                *id,
                SectionValue::Pathways(Section::unavailable(
                    GeneSection::Pathways.source(),
                    Reason::NoJoinId,
                )),
            ));
            continue;
        };
        let mut pathways = Vec::new();
        let mut failure = None;
        for key in keys {
            let Some(index) = ids.iter().position(|id| *id == key) else {
                continue;
            };
            let Some(result) = results.get(index) else {
                continue;
            };
            match result {
                Ok(mapped) => {
                    dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                    status::append(&mut problems, mapped.problems.clone());
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
        let section = if let Some(reason) = failure {
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
        };
        values.push((*id, SectionValue::Pathways(section)));
    }
    SectionResult {
        dropped,
        values,
        source,
        problems,
        local,
    }
}
