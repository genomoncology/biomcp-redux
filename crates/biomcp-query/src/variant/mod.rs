//! Published-identifier resolution with source-owned allele identity checks.
use crate::{Action, Ctx, Outcome, status};
use biodata::{QueryBody, Reason, Request, Source, Status, StepKind, Text, VariantCard, VariantGet, VariantHeader, VariantNamespace, VariantResolution};
use biomcp_adapters::{allele_registry, mutalyzer, myvariant, variantvalidator};
use biomcp_fetch::{Leg, RequestError};
/// Resolve one checked input without guessing a build, allele or preferred candidate.
pub fn get(cx: &Ctx<'_>, query: &VariantGet) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let direct = matches!(query.input().namespace(), Ok(VariantNamespace::Rs | VariantNamespace::ClinVar | VariantNamespace::Chromosome));
    let spec = if direct { &myvariant::SPEC } else { match query.input().namespace() { Ok(VariantNamespace::Ca) => &allele_registry::SPEC, _ => match query.resolver() { biodata::VariantResolver::Car => &allele_registry::SPEC, biodata::VariantResolver::Mutalyzer => &mutalyzer::SPEC, biodata::VariantResolver::Variantvalidator => &variantvalidator::SPEC } } };
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let mut problems = Vec::new();
    let result = resolve(query, &leg, &mut problems);
    let error = result.as_ref().err().copied();
    let mut sources = vec![status::source(spec.source, leg.finish(), error)];
    let mut candidates = Vec::new();
    let mut rejection = None;
    let mut resolution = VariantResolution::Unavailable;
    if let Ok((rows, rejected)) = result { candidates = rows; rejection = rejected; resolution = if rejection.is_some() { VariantResolution::Rejected } else { classify(candidates.len()) }; }
    if matches!(error, Some(RequestError::Source(e)) if e.reason == Reason::NotFound) { resolution = VariantResolution::Absent; }
    if rejection.is_some() { sources[0].state = biodata::SourceState::Failed; sources[0].reason = Some(Reason::Rejected); }
    let mut local = error.and_then(status::local);
    // Only CAR's explicit GRCh37 genomic representation may join MyVariant's documented hg19 IDs.
    if spec.source == Source::AlleleRegistry && resolution == VariantResolution::Resolved {
        let leg = cx.fetch.leg(&myvariant::SPEC, deadline).fresh(cx.fresh);
        let mut join_error = None;
        for candidate in &mut candidates {
            if let Some(id) = join_id(candidate) {
                match myvariant::lookup(&leg, &id) {
                    Ok(found) => { problems.extend(found.problems); for row in found.value { let mut facts = candidate.facts.to_vec(); facts.extend(row.facts); candidate.facts = facts.into(); } }
                    Err(e) => { join_error = Some(e); local = local.or(status::local(e)); }
                }
            }
        }
        sources.push(status::source(Source::MyVariant, leg.finish(), join_error));
    }
    let mut next = Vec::new();
    if resolution == VariantResolution::Candidates {
        for row in &candidates {
            if let Ok(input) = row.identity.key.as_str().parse() && let Ok(q) = VariantGet::new(input, query.resolver(), Some(biodata::Build::Grch37)) {
                next.push((StepKind::Disambiguate, Action::Run { request: Request::GetVariant(q), fresh: false }));
            }
        }
    }
    let overall = if resolution == VariantResolution::Unavailable { Status::Failed } else if resolution == VariantResolution::Absent { Status::Absent } else if !problems.is_empty() || sources.iter().any(|s| matches!(s.state, biodata::SourceState::Failed | biodata::SourceState::Stale)) && resolution != VariantResolution::Rejected { Status::Partial } else { Status::Complete };
    let out = Outcome { status: overall, body: QueryBody::VariantCard(VariantCard { resolution, source: spec.source, candidates: candidates.into(), rejection }), page: None, sources, problems, next, local };
    command.finish(deadline.remaining());
    out
}
fn classify(len: usize) -> VariantResolution { match len { 0 => VariantResolution::Absent, 1 => VariantResolution::Resolved, _ => VariantResolution::Candidates } }
fn resolve(query: &VariantGet, leg: &Leg<'_>, problems: &mut Vec<biodata::Problem>) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    let input = query.input().as_str();
    match query.input().namespace() {
        Ok(VariantNamespace::Rs | VariantNamespace::ClinVar | VariantNamespace::Chromosome) => {
            let term = if query.input().namespace().ok() == Some(VariantNamespace::ClinVar) { format!("clinvar.variant_id:{}", input.strip_prefix("ClinVar:").unwrap_or(input)) } else { input.to_owned() };
            let found = myvariant::lookup(leg, &term)?;
            problems.extend(found.problems);
            Ok((found.value, None))
        }
        Ok(VariantNamespace::Ca) => car(allele_registry::by_id(leg, input)?),
        _ => match query.resolver() {
            biodata::VariantResolver::Car => car(allele_registry::hgvs(leg, input)?),
            biodata::VariantResolver::Mutalyzer => match mutalyzer::resolve(leg, input)? { mutalyzer::Resolution::Found(row) => Ok((vec![row], None)), mutalyzer::Resolution::Rejected(codes) => Ok((Vec::new(), codes.into_iter().next())) },
            biodata::VariantResolver::Variantvalidator => {
                let result = variantvalidator::resolve(leg, input, query.build().ok_or_else(|| biomcp_fetch::SourceError { source: Source::VariantValidator, reason: Reason::Rejected, status: None, retry_after: None })?)?;
                Ok((result.candidates, if result.rejected { Text::new("validation_warning").ok() } else { None }))
            }
        }
    }
}
fn car(result: allele_registry::Resolution) -> Result<(Vec<VariantHeader>, Option<Text>), RequestError> {
    Ok(match result { allele_registry::Resolution::Found(row) => (vec![row], None), allele_registry::Resolution::Rejected(code) => (Vec::new(), Some(code)) })
}
fn join_id(row: &VariantHeader) -> Option<String> {
    row.identity.representations.iter().find_map(|r| {
        if r.assembly.as_ref().map(Text::as_str) != Some("GRCh37") { return None; }
        let coordinate = r.coordinate.as_ref()?;
        if !r.notation.as_str().starts_with("NC_") { return None; }
        myvariant::genomic_id(coordinate.contig.as_str(), &r.notation)
    })
}
