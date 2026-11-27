use crate::{Ctx, Outcome, gene, status};
use biodata::{PathwayId, PathwaySpecies, UniProtAcc};
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
    let error = resolved.as_ref().err().copied();
    let report = identity.finish();
    // Prefer HGNC when an accession-shaped value also names a gene or alias.
    if (matches!(&resolved, Ok(r) if matches!(r.value, GeneResolution::NotFound { .. }))
        || error.is_some())
        && let GeneInput::Text(text) = input
        && let Ok(id) = UniProtAcc::new(&text.as_str().to_ascii_uppercase())
    {
        return Ok(Mapped {
            refusals: biomcp_adapters::Refusal::None,
            value: vec![MappingId::UniProt(id)],
            problems: Vec::new(),
        });
    }
    out.sources
        .push(status::source(Source::Hgnc, report, error));
    let resolved = resolved.inspect_err(|_| skip_reactome(out))?;
    let mut problems = resolved.problems;
    let value = match resolved.value {
        GeneResolution::Resolved { gene, .. } => MappingId::for_gene(&gene),
        _ => {
            problems.push(Problem {
                source: Source::Hgnc,
                record: input.to_string().parse().ok(),
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
