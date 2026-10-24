//! Published-identifier resolution with source-owned allele identity checks.
pub(crate) mod assertions;
mod continuations;
mod direct;
pub mod normalize;
pub mod recurrence;
pub mod search;
mod sections;
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    QueryBody, Reason, Request, Source, Status, StepKind, Text, VariantCard, VariantGet,
    VariantHeader, VariantNamespace, VariantResolution,
};
use biomcp_adapters::{allele_registry, mutalyzer, myvariant, variantvalidator};
use biomcp_fetch::{Leg, RequestError};
/// Resolve one checked input without guessing a build, allele or preferred candidate.
pub fn get(cx: &Ctx<'_>, query: &VariantGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    if let Some(source) = direct::source(cx, query) {
        let out = direct::get(cx, query, source, deadline);
        command.finish(deadline.remaining());
        return out;
    }
    let spec = source(query);
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let mut problems = Vec::new();
    let result = resolve(query, &leg, &mut problems);
    let error = result.as_ref().err().copied();
    let mut main_source = status::source(spec.source, leg.finish(), error);
    let mut candidates = Vec::new();
    let mut rejection = None;
    let mut resolution = VariantResolution::Unavailable;
    if let Ok((rows, rejected)) = result {
        candidates = rows;
        rejection = rejected;
        resolution = if rejection.is_some() {
            VariantResolution::Rejected
        } else if candidates.is_empty() && !problems.is_empty() {
            VariantResolution::Unavailable
        } else {
            classify(candidates.len())
        };
    }
    if matches!(error, Some(RequestError::Source(e)) if e.reason == Reason::NotFound) {
        resolution = VariantResolution::Absent;
    }
    if rejection.is_some() {
        main_source.state = biodata::SourceState::Failed;
        main_source.reason = Some(Reason::Rejected);
    }
    let mut sources = vec![main_source];
    let mut local = error.and_then(status::local);
    if query.sections().is_empty()
        && spec.source == Source::AlleleRegistry
        && resolution == VariantResolution::Resolved
    {
        let (source, error) = enrich(cx, deadline, &mut candidates, &mut problems);
        sources.push(source);
        local = local.or(error);
    }
    let page = candidate_page(query, resolution, &mut candidates);
    let (section_sources, section_problems, section_error) =
        sections::apply(cx, query, &mut candidates, deadline);
    sources.extend(section_sources);
    problems.extend(section_problems);
    local = local.or(section_error);
    let next = next(query, resolution, &candidates, page.as_ref());
    let out = Outcome {
        status: overall(resolution, &problems, &sources),
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
    };
    command.finish(deadline.remaining());
    out
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
    problems: &mut Vec<biodata::Problem>,
) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    let input = query.input().as_str();
    match query.input().namespace() {
        Ok(VariantNamespace::Rs | VariantNamespace::ClinVar | VariantNamespace::Chromosome) => {
            let term = if query.input().namespace().ok() == Some(VariantNamespace::ClinVar) {
                format!(
                    "clinvar.variant_id:{}",
                    input.strip_prefix("clinvar:").unwrap_or(input)
                )
            } else {
                input.to_owned()
            };
            let found =
                myvariant::lookup(leg, &term, query.build().unwrap_or(biodata::Build::Grch37))?;
            problems.extend(found.problems);
            Ok((found.value, None))
        }
        Ok(VariantNamespace::Ca) => car(allele_registry::by_id(leg, input)?, problems),
        _ => match query.resolver() {
            biodata::VariantResolver::Car => car(allele_registry::hgvs(leg, input)?, problems),
            biodata::VariantResolver::Mutalyzer => match mutalyzer::resolve(leg, input)? {
                mutalyzer::Resolution::Found(row) => Ok((vec![row], None)),
                mutalyzer::Resolution::Rejected(codes) => {
                    Ok((Vec::new(), codes.into_iter().next()))
                }
            },
            biodata::VariantResolver::Variantvalidator => {
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
    problems: &mut Vec<biodata::Problem>,
) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    Ok(match result {
        allele_registry::Resolution::Found(row) => {
            problems.extend(row.problems);
            (vec![row.value], None)
        }
        allele_registry::Resolution::Rejected(code) => (Vec::new(), Some(code)),
    })
}
fn join_id(row: &VariantHeader) -> Option<String> {
    row.identity.representations.iter().find_map(|r| {
        if r.build != Some(biodata::Build::Grch37) {
            return None;
        }
        let allele = r.allele.as_ref()?;
        let chromosome =
            biodata::Chromosome::from_accession(allele.location().sequence, biodata::Build::Grch37)
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
                biodata::VariantResolver::Car => &allele_registry::SPEC,
                biodata::VariantResolver::Mutalyzer => &mutalyzer::SPEC,
                biodata::VariantResolver::Variantvalidator => &variantvalidator::SPEC,
            },
        }
    }
}

fn enrich(
    cx: &Ctx<'_>,
    deadline: biomcp_fetch::Deadline<'_>,
    candidates: &mut [VariantHeader],
    problems: &mut Vec<biodata::Problem>,
) -> (biodata::SourceOutcome, Option<RequestError>) {
    let mut local = None;
    let leg = cx.fetch.leg(&myvariant::SPEC, deadline).fresh(cx.fresh);
    let mut join_error = None;
    for candidate in candidates {
        if let Some(id) = join_id(candidate) {
            match myvariant::lookup(&leg, &id, biodata::Build::Grch37) {
                Ok(found) => {
                    problems.extend(found.problems);
                    for row in found.value {
                        candidate.myvariant = Some(row.identity);
                    }
                }
                Err(e) => {
                    join_error = Some(e);
                    local = local.or(status::local(e));
                }
            }
        }
    }
    (
        status::source(Source::MyVariant, leg.finish(), join_error),
        local,
    )
}

fn next(
    query: &VariantGet,
    resolution: VariantResolution,
    candidates: &[VariantHeader],
    page: Option<&biodata::PageInfo>,
) -> Vec<(StepKind, Action)> {
    let mut next = Vec::new();
    if resolution == VariantResolution::Candidates {
        for row in candidates {
            if let Ok(input) = row.record.id.as_str().parse()
                && let Ok(q) = VariantGet::new(
                    input,
                    query.resolver(),
                    row.identity
                        .representations
                        .iter()
                        .find_map(|r| r.build)
                        .or(query.build()),
                )
            {
                next.push((
                    StepKind::Disambiguate,
                    Action::Run {
                        request: Request::GetVariant(q),
                        fresh: false,
                    },
                ));
            }
        }
    }
    if let Some(page) = page
        && (page.offset as u64 + page.returned as u64) < page.total
    {
        next.push((
            StepKind::Page,
            Action::Run {
                request: Request::GetVariant(query.at_offset(page.offset + page.returned)),
                fresh: false,
            },
        ));
    }
    continuations::collect(candidates, &mut next);
    next
}

fn candidate_page(
    query: &VariantGet,
    resolution: VariantResolution,
    candidates: &mut Vec<VariantHeader>,
) -> Option<biodata::PageInfo> {
    if resolution != VariantResolution::Candidates {
        return None;
    }
    let total = candidates.len() as u64;
    *candidates = std::mem::take(candidates)
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .collect();
    Some(biodata::PageInfo {
        offset: query.offset(),
        returned: candidates.len() as u32,
        total,
        pool: None,
        truncated: None,
    })
}
