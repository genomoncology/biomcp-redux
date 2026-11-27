use crate::{Ctx, Outcome, gene, status};
use biodata::{PathwayId, PathwaySpecies, Text, UniProtAcc};
use biomcp_adapters::{
    Mapped, hgnc,
    reactome::{self, MappingId},
    registry,
};
use biomcp_fetch::{Deadline, InputError, RequestError};
use biomcp_types::{
    GeneInput, GeneResolution, PathwayHeader, PathwaySearch, Problem, ProblemCode, Reason, Source,
    SourceStatus,
};

pub(super) fn headers(
    cx: &Ctx<'_>,
    q: &PathwaySearch,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Mapped<Vec<PathwayHeader>>, RequestError> {
    let input = q.gene().ok_or(InputError::Serialization)?;
    let fresh = cx.fresh || q.fresh();
    let mappings = mapping_ids(cx, input, deadline, fresh, out)?;
    let mut mapped = Mapped {
        refusals: mappings.refusals,
        value: Vec::new(),
        problems: mappings.problems,
    };
    if mappings.value.is_empty() {
        skip_reactome(out);
        return Ok(mapped);
    }
    let leg = cx.fetch.leg(&reactome::SPEC, deadline).fresh(fresh);
    let replies = leg.batch(&mappings.value, reactome::pathways);
    let error = replies.iter().find_map(|r| r.as_ref().err().copied());
    out.sources
        .push(status::source(Source::Reactome, leg.finish(), error));
    for reply in replies {
        let rows = reply?;
        mapped.refusals.include(rows.refusals);
        mapped.problems.extend(rows.problems);
        for row in rows.value {
            let id = PathwayId::Reactome(row.id);
            if !mapped.value.iter().any(|r: &PathwayHeader| r.id == id) {
                mapped.value.push(PathwayHeader {
                    id,
                    name: row.name,
                    species: PathwaySpecies::Human,
                    version: None,
                    revision: None,
                });
            }
        }
    }
    Ok(mapped)
}

fn mapping_ids(
    cx: &Ctx<'_>,
    input: &GeneInput,
    deadline: Deadline<'_>,
    fresh: bool,
    out: &mut Outcome,
) -> Result<Mapped<Vec<MappingId>>, RequestError> {
    let identity = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(fresh);
    let resolved = gene::resolve(&identity, input);
    let unresolved = match &resolved {
        Ok(mapped) => matches!(mapped.value, GeneResolution::NotFound { .. }),
        Err(_) => true,
    };
    // Prefer HGNC when an accession-shaped value also names a gene or alias.
    let accession = match input {
        GeneInput::Text(text) => Some(UniProtAcc::new(&text.as_str().to_ascii_uppercase())),
        _ => None,
    };
    match accession {
        Some(Ok(id)) if unresolved => Ok(Mapped {
            refusals: biomcp_adapters::Refusal::None,
            value: vec![MappingId::UniProt(id)],
            problems: Vec::new(),
        }),
        _ => {
            let mapped = resolved.and_then(|resolved| gene_mappings(&identity, input, resolved));
            out.sources.push(status::source(
                Source::Hgnc,
                identity.finish(),
                mapped.as_ref().err().copied(),
            ));
            mapped.inspect_err(|_| skip_reactome(out))
        }
    }
}

fn gene_mappings(
    identity: &biomcp_fetch::Leg<'_>,
    input: &GeneInput,
    resolved: Mapped<GeneResolution>,
) -> Result<Mapped<Vec<MappingId>>, RequestError> {
    let mut problems = resolved.problems;
    let value = match resolved.value {
        GeneResolution::Resolved { gene, .. } => MappingId::for_gene(identity, &gene)?,
        _ => {
            problems.push(Problem {
                source: Source::Hgnc,
                record: Some(Text::new(&input.to_string()).map_err(|_| InputError::Serialization)?),
                field: "gene".into(),
                code: ProblemCode::ValueUnknown,
            });
            Vec::new()
        }
    };
    Ok(Mapped {
        refusals: resolved.refusals,
        problems,
        value,
    })
}

fn skip_reactome(out: &mut Outcome) {
    out.sources.push(registry::outcome(
        Source::Reactome,
        SourceStatus::Skipped(Some(Reason::NoJoinId)),
    ));
}
