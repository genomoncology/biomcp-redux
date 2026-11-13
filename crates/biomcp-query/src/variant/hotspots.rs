use crate::{Ctx, status};

use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    PageInfo, Problem, ProblemCode, Reason, Section, Source, SourceOutcome, VariantGet,
    VariantHeader, VariantHotspots,
};

pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    mut deadline: Deadline<'_>,
) -> (
    Section<VariantHotspots>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
    bool,
) {
    let disabled = cx.fetch.disabled(Source::CancerHotspots);
    let mut genes = symbols(candidates);
    let mut sources = Vec::new();
    let mut problems = Vec::new();
    let mut dropped = false;
    let mut error = disabled.then_some(failure(Reason::Disabled));
    if genes.is_empty()
        && !disabled
        && let Some(join) = registry_genes(cx, candidates, &mut deadline)
    {
        sources.push(join.source);
        problems.extend(join.problems);
        dropped |= join.dropped;
        error = join.error;
        genes = join.genes;
    }
    let leg = cx
        .fetch
        .leg(&biomcp_adapters::hotspots::SPEC, deadline)
        .fresh(cx.fresh);
    let mut rows = Vec::new();
    for gene in &genes {
        if error.is_some() {
            break;
        }
        match biomcp_adapters::hotspots::gene(&leg, gene) {
            Ok(mapped) => {
                dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
                rows.extend(mapped.value);
                status::append(&mut problems, mapped.problems);
            }
            Err(e) => {
                error = Some(e);
                break;
            }
        }
    }
    if genes.is_empty() && error.is_none() {
        error = Some(failure(Reason::NoJoinId));
    }
    if rows.is_empty() && dropped && error.is_none() {
        error = Some(failure(Reason::BadReply));
    }
    sources.push(status::source(Source::CancerHotspots, leg.finish(), error));
    let value = disclose(query, rows, error, &mut problems);
    (
        value,
        sources,
        problems,
        error.and_then(status::local),
        dropped,
    )
}

fn symbols(candidates: &[VariantHeader]) -> Vec<biodata::GeneSymbol> {
    candidates
        .iter()
        .flat_map(|c| {
            c.identity
                .genes
                .iter()
                .chain(c.myvariant.iter().flat_map(|v| v.genes.iter()))
        })
        .filter_map(|gene| gene.label().as_str().parse::<biodata::GeneSymbol>().ok())
        .fold(Vec::new(), |mut genes, gene| {
            if !genes.contains(&gene) {
                genes.push(gene);
            }
            genes
        })
}

struct Join {
    genes: Vec<biodata::GeneSymbol>,
    source: SourceOutcome,
    problems: Vec<Problem>,
    error: Option<RequestError>,
    dropped: bool,
}

/// A registry candidate (a CAid) names no gene; read the gene from MyVariant's
/// record for the candidate's own genomic allele, the join the MyVariant sections use.
fn registry_genes(
    cx: &Ctx<'_>,
    candidates: &[VariantHeader],
    deadline: &mut Deadline<'_>,
) -> Option<Join> {
    use biomcp_adapters::myvariant;
    if cx.fetch.disabled(Source::MyVariant)
        || candidates.iter().any(|c| c.source == Source::MyVariant)
    {
        return None;
    }
    let (id, build) = super::components::target(candidates)?;
    let leg = cx.fetch.leg(&myvariant::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let result = myvariant::lookup(&leg, &id, build);
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let error = result.as_ref().err().copied();
    let source = status::source(Source::MyVariant, leg.finish(), error);
    let mut join = Join {
        genes: Vec::new(),
        source,
        problems: Vec::new(),
        error,
        dropped: false,
    };
    if let Ok(found) = result {
        join.dropped = found.refusals == biomcp_adapters::Refusal::Refused;
        join.problems = found.problems;
        join.genes = symbols(&found.value);
    }
    Some(join)
}

fn disclose(
    query: &VariantGet,
    rows: Vec<biomcp_types::HotspotRecord>,
    error: Option<RequestError>,
    problems: &mut Vec<Problem>,
) -> Section<VariantHotspots> {
    if let Some(error) = error {
        Section::unavailable(
            Source::CancerHotspots,
            crate::status::reason(error).unwrap_or(biomcp_types::Reason::Unavailable),
        )
    } else if rows.is_empty() {
        Section::Empty {
            source: Source::CancerHotspots,
            record: None,
        }
    } else {
        problems.push(Problem {
            source: Source::CancerHotspots,
            record: None,
            field: "transcript_consequence_join".into(),
            code: ProblemCode::ValueUnknown,
        });
        let total = rows.len() as u64;
        let records = rows
            .into_iter()
            .skip(query.section_offset() as usize)
            .take(query.section_limit() as usize)
            .collect::<Vec<_>>();
        let returned = records.len() as u32;
        let next = if (query.section_offset() as u64 + returned as u64) < total {
            Some(query.at_section_offset(query.section_offset() + returned))
        } else {
            None
        };
        Section::Data {
            source: Source::CancerHotspots,
            data: VariantHotspots::new(
                records.into(),
                PageInfo {
                    offset: query.section_offset(),
                    returned,
                    total,
                    pool: None,
                    truncated: None,
                },
                next,
            ),
        }
    }
}

fn failure(reason: Reason) -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::CancerHotspots,
        reason,
        status: None,
        tls_cause: None,
        retry_after: None,
    }
    .into()
}
