//! Published-identifier resolution with source-owned allele identity checks.
mod alphagenome;
mod alphagenome_join;
mod alphagenome_page;
pub(crate) mod assertions;
mod civic;
mod components;
mod conservation;
mod continuations;
use continuations::next;
mod cosmic;
mod direct;
mod hotspots;
pub mod normalize;
mod oncokb;
mod predict;
pub mod recurrence;
pub mod search;
mod sections;
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::{allele_registry, mutalyzer, myvariant, variantvalidator};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    QueryBody, Reason, Source, Status, VariantCard, VariantGet, VariantHeader, VariantNamespace,
    VariantResolution,
};
/// Resolve one checked input without guessing a build, allele or preferred candidate.
pub fn get(cx: &Ctx<'_>, query: &VariantGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    if let Some(source) = direct::source(cx, query) {
        let out = direct::get(cx, query, source, deadline);
        command.finish(deadline.remaining());
        return out;
    }
    let spec = source(query);
    let population = population_identity::requested(query) && !cx.fetch.disabled(Source::Gnomad);
    let found = population_identity::read(cx, query, deadline, population);
    deadline = deadline.after_stage(found.elapsed);
    let mut problems = found.problems;
    let mut candidates = found.rows;
    let mut rejection = found.rejection;
    let mut resolution = if found.error.is_some() {
        VariantResolution::Unavailable
    } else if rejection.is_some() {
        VariantResolution::Rejected
    } else if candidates.is_empty() && !problems.is_empty() {
        VariantResolution::Unavailable
    } else {
        classify(candidates.len())
    };
    if matches!(found.error, Some(RequestError::Source(e)) if e.reason == Reason::NotFound) {
        resolution = VariantResolution::Absent;
    }
    let population_reason = if population && resolution == VariantResolution::Candidates {
        Some(Reason::NoJoinId)
    } else if population {
        found.reason
    } else {
        None
    };
    let mut sources = found.sources;
    let mut local = found.local;
    if query.sections().is_empty()
        && spec.source == Source::AlleleRegistry
        && resolution == VariantResolution::Resolved
    {
        let (source, error) = enrich(cx, &mut deadline, &mut candidates, &mut problems);
        sources.push(source);
        local = local.or(error);
    }
    if transcript_gene_mismatch(query, &candidates) {
        candidates.clear();
        resolution = VariantResolution::Rejected;
        rejection = Text::new("transcript_gene_mismatch").ok();
    }
    match_transcripts(query, &mut candidates);
    let page = candidate_page(query, resolution, &mut candidates);
    let (section_sources, section_problems, section_error) =
        sections::apply(cx, query, &mut candidates, &mut deadline, population_reason);
    sources.extend(section_sources);
    problems.extend(section_problems);
    local = local.or(section_error);
    let mut next = next(query, resolution, &candidates, page.as_ref());
    next.extend(oncokb::next(query, &sources));
    next.extend(alphagenome::next(&sources));
    let out = crate::status::finish(
        [overall(resolution, &problems, &sources)],
        crate::status::Parts {
            dropped: false,
            body: QueryBody::VariantCard(VariantCard {
                resolution,
                source: spec.source,
                candidates: candidates.into(),
                rejection,
            }),
            page,
            sources,
            problems,
            next,
            local,
        },
    );
    command.finish(deadline.remaining());
    out
}
fn transcript_gene_mismatch(query: &VariantGet, candidates: &[VariantHeader]) -> bool {
    let Some((reference, gene)) = query.input().reference_gene() else {
        return false;
    };
    candidates.iter().flat_map(|c| &c.transcripts).any(|t| {
        t.transcript.as_str() == reference
            && t.gene
                .as_ref()
                .is_some_and(|source_gene| source_gene.as_str() != gene)
    })
}
fn match_transcripts(query: &VariantGet, candidates: &mut [VariantHeader]) {
    for candidate in candidates {
        for transcript in &mut candidate.transcripts {
            transcript.matched =
                query
                    .input()
                    .as_str()
                    .split_once(':')
                    .is_some_and(|(id, notation)| {
                        transcript.transcript.as_str() == transcript_reference(id)
                            && transcript
                                .hgvs_c
                                .as_ref()
                                .is_some_and(|c| c.as_str() == notation)
                    });
        }
    }
}
fn transcript_reference(reference: &str) -> &str {
    let Some((outer, inner)) = reference.split_once('(') else {
        return reference;
    };
    let inner = inner.strip_suffix(')').unwrap_or(inner);
    if inner.parse::<biodata::Accession>().is_ok() {
        inner
    } else {
        outer
    }
}
fn classify(len: usize) -> VariantResolution {
    match len {
        0 => VariantResolution::Absent,
        1 => VariantResolution::Resolved,
        _ => VariantResolution::Candidates,
    }
}
pub(crate) fn resolve(
    query: &VariantGet,
    leg: &Leg<'_>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    let native = query.input().native_hgvs();
    let input = native.as_ref();
    match query.input().namespace() {
        Ok(VariantNamespace::Rs | VariantNamespace::ClinVar | VariantNamespace::Chromosome) => {
            myvariant_identity(query, leg, problems, false)
        }
        Ok(VariantNamespace::Ca) => car(allele_registry::by_id(leg, input)?, problems),
        _ => match query.resolver() {
            biomcp_types::VariantResolver::Car => car(allele_registry::hgvs(leg, input)?, problems),
            biomcp_types::VariantResolver::Mutalyzer => match mutalyzer::resolve(leg, input)? {
                mutalyzer::Resolution::Found(row) => Ok((vec![row], None)),
                mutalyzer::Resolution::Refused(fields) => {
                    problems.extend(fields);
                    Ok((Vec::new(), None))
                }
                mutalyzer::Resolution::Rejected(codes) => {
                    Ok((Vec::new(), codes.into_iter().next()))
                }
            },
            biomcp_types::VariantResolver::Variantvalidator => {
                let result = variantvalidator::resolve(
                    leg,
                    input,
                    query.build().ok_or(biomcp_fetch::SourceError {
                        source: Source::VariantValidator,
                        reason: Reason::Rejected,
                        status: None,
                        retry_after: None,
                    })?,
                )?;
                problems.extend(result.problems);
                Ok((
                    result.candidates,
                    if result.rejected {
                        Text::new("validation_warning").ok()
                    } else {
                        None
                    },
                ))
            }
        },
    }
}
fn car(
    result: allele_registry::Resolution,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    Ok(match result {
        allele_registry::Resolution::Found(row) => {
            problems.extend(row.problems);
            (vec![row.value], None)
        }
        allele_registry::Resolution::Rejected(code) => (Vec::new(), Some(code)),
        allele_registry::Resolution::Refused(fields) => {
            problems.extend(fields);
            (Vec::new(), None)
        }
    })
}
fn join_id(row: &VariantHeader) -> Option<String> {
    row.identity.representations.iter().find_map(|r| {
        if r.build != Some(biodata::Build::GRCh37) {
            return None;
        }
        let allele = r.allele.as_ref()?;
        let chromosome =
            biodata::Chromosome::from_accession(allele.location().sequence, biodata::Build::GRCh37)
                .ok()?;
        myvariant::genomic_id(&chromosome.name(), r.expression.value())
    })
}
pub(crate) fn source(query: &VariantGet) -> &'static biomcp_fetch::SourceSpec {
    let direct = matches!(
        query.input().namespace(),
        Ok(VariantNamespace::Rs | VariantNamespace::ClinVar | VariantNamespace::Chromosome)
    );
    if direct {
        &myvariant::SPEC
    } else {
        match query.input().namespace() {
            Ok(VariantNamespace::Ca) => &allele_registry::SPEC,
            _ => match query.resolver() {
                biomcp_types::VariantResolver::Car => &allele_registry::SPEC,
                biomcp_types::VariantResolver::Mutalyzer => &mutalyzer::SPEC,
                biomcp_types::VariantResolver::Variantvalidator => &variantvalidator::SPEC,
            },
        }
    }
}

fn enrich(
    cx: &Ctx<'_>,
    deadline: &mut biomcp_fetch::Deadline<'_>,
    candidates: &mut [VariantHeader],
    problems: &mut Vec<biomcp_types::Problem>,
) -> (biomcp_types::SourceOutcome, Option<RequestError>) {
    let mut local = None;
    let leg = cx.fetch.leg(&myvariant::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut join_error = None;
    for candidate in candidates {
        if let Some(id) = join_id(candidate) {
            match myvariant::lookup(&leg, &id, biodata::Build::GRCh37) {
                Ok(found) => {
                    problems.extend(found.problems);
                    for row in found.value {
                        candidate.myvariant = Some(row.identity);
                        candidate.transcripts = row.transcripts;
                        candidate.component_terms = row.component_terms;
                    }
                }
                Err(e) => {
                    join_error = Some(e);
                    local = local.or(status::local(e));
                }
            }
        }
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    (
        status::source(Source::MyVariant, leg.finish(), join_error),
        local,
    )
}

fn candidate_page(
    query: &VariantGet,
    resolution: VariantResolution,
    candidates: &mut Vec<VariantHeader>,
) -> Option<biomcp_types::PageInfo> {
    if resolution != VariantResolution::Candidates {
        return None;
    }
    let total = candidates.len() as u64;
    *candidates = std::mem::take(candidates)
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .collect();
    Some(biomcp_types::PageInfo {
        offset: query.offset(),
        returned: candidates.len() as u32,
        total,
        pool: None,
        truncated: None,
    })
}

pub(crate) fn overall(
    resolution: VariantResolution,
    problems: &[biomcp_types::Problem],
    sources: &[biomcp_types::SourceOutcome],
) -> Status {
    if matches!(
        resolution,
        VariantResolution::Unavailable | VariantResolution::Rejected
    ) {
        crate::status::item_status(false, true, false)
    } else if resolution == VariantResolution::Absent {
        crate::status::item_status(false, false, false)
    } else if !problems.is_empty()
        || sources.iter().any(|s| {
            matches!(
                s.state,
                biomcp_types::SourceState::Failed | biomcp_types::SourceState::Stale
            )
        })
    {
        crate::status::item_status(true, false, true)
    } else {
        crate::status::item_status(true, false, false)
    }
}

mod gwas;
mod population;
pub(crate) mod population_detail;
mod population_identity;
mod population_release;

fn myvariant_identity(
    query: &VariantGet,
    leg: &Leg<'_>,
    problems: &mut Vec<biomcp_types::Problem>,
    population: bool,
) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    let input = query.input().as_str();
    let term = if query.input().namespace().ok() == Some(VariantNamespace::ClinVar) {
        format!(
            "clinvar.variant_id:{}",
            input.strip_prefix("clinvar:").unwrap_or(input)
        )
    } else {
        input.to_owned()
    };
    let build = query.myvariant_build();
    let found = if population {
        myvariant::population_lookup(leg, &term, build)?
    } else {
        myvariant::lookup(leg, &term, build)?
    };
    problems.extend(found.problems);
    Ok((found.value, None))
}
