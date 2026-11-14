//! Explicit provider normalization with independent source outcomes.
use crate::{Ctx, Outcome, status};
use biomcp_types::Paged;
use biomcp_types::ReplyBody;

use biomcp_adapters::{allele_registry, mutalyzer, variantvalidator};
use biomcp_fetch::{Deadline, RequestError, SourceSpec};
use biomcp_types::{
    NormalizationResult, NormalizationSource, Request, SourceOutcome, StepKind,
    VariantNormalization, VariantNormalize, VariantResolution,
};
type Row = (
    biomcp_adapters::Mapped<NormalizationResult>,
    SourceOutcome,
    Option<RequestError>,
);
/// Call each explicitly selected service independently under the shared deadline.
pub(crate) fn normalize(cx: &Ctx<'_>, query: &VariantNormalize) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let jobs = selected(query.source())
        .into_iter()
        .map(|spec| {
            Box::new(move || service(cx, query, spec, deadline)) as Box<dyn FnOnce() -> Row + Send>
        })
        .collect();
    let mut rows = crate::fan_out::run(jobs);
    let mut calls: Vec<SourceOutcome> = rows.iter().map(|(_, source, _)| source.clone()).collect();
    let mut next = Vec::new();
    transcript_fit(cx, query, deadline, &mut rows, &mut calls, &mut next);
    let mut results = Vec::new();
    let mut local = None;
    let mut more = false;
    let mut states = Vec::new();
    let mut problems = Vec::new();
    for (mapped, _, error) in rows {
        problems.extend(mapped.problems);
        let mut result = mapped.value;
        states.push(
            if matches!(
                result.outcome,
                VariantResolution::Rejected | VariantResolution::Unavailable
            ) {
                crate::status::item_status(false, true, false)
            } else {
                crate::status::item_status(true, false, false)
            },
        );
        more |= page(&mut result, query);
        results.push(result);
        local = local.or(error);
    }
    // One entry per source; a failed call's state stays.
    let mut sources: Vec<SourceOutcome> = Vec::new();
    for call in calls {
        match sources.iter_mut().find(|seen| seen.source == call.source) {
            Some(seen) => status::retain_source_state(seen, &call),
            None => sources.push(call),
        }
    }
    if more {
        next.push(crate::next::Step::run(
            StepKind::Page,
            &selected(query.source())
                .iter()
                .map(|spec| spec.source)
                .collect::<Vec<_>>(),
            Request::VariantNormalize(
                query.at_offset(query.offset().saturating_add(query.limit())),
            ),
            false,
        ));
    }
    let outcome = crate::status::finish(
        [status::status(states.into_iter())],
        crate::status::Parts {
            dropped: false,
            body: (VariantNormalization {
                results: results.into(),
            })
            .with_query(query.clone()),
            sources,
            problems,
            next,
            page: None,
            local,
        },
    );
    command.finish(deadline.remaining());
    outcome
}
/// Page one service's messages and mappings; true when rows remain past this page.
fn page(result: &mut NormalizationResult, query: &VariantNormalize) -> bool {
    result.total_messages = result.messages.len() as u32;
    let skip = query.offset() as usize;
    let take = query.limit() as usize;
    result.messages = std::mem::take(&mut result.messages)
        .into_vec()
        .into_iter()
        .skip(skip)
        .take(take)
        .collect();
    result.mappings = std::mem::take(&mut result.mappings)
        .into_vec()
        .into_iter()
        .skip(skip)
        .take(take)
        .collect();
    // A row resolved on a fitting transcript pages through its own next step.
    result.hgvs.is_none()
        && result.total_messages.max(result.total_mappings) as u64
            > query.offset() as u64 + query.limit() as u64
}
fn selected(source: NormalizationSource) -> Vec<&'static SourceSpec> {
    match source {
        NormalizationSource::Car => vec![&allele_registry::SPEC],
        NormalizationSource::Mutalyzer => vec![&mutalyzer::SPEC],
        NormalizationSource::Variantvalidator => vec![&variantvalidator::SPEC],
        NormalizationSource::All => vec![
            &allele_registry::SPEC,
            &mutalyzer::SPEC,
            &variantvalidator::SPEC,
        ],
    }
}
fn service(
    cx: &Ctx<'_>,
    query: &VariantNormalize,
    spec: &'static SourceSpec,
    deadline: Deadline<'_>,
) -> Row {
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let result = match spec.source {
        biomcp_types::Source::AlleleRegistry => {
            allele_registry::normalize(&leg, query.hgvs(), query.build())
        }
        biomcp_types::Source::Mutalyzer => mutalyzer::normalize(&leg, query.hgvs()),
        _ => query
            .build()
            .ok_or(
                biomcp_fetch::SourceError {
                    source: spec.source,
                    reason: biomcp_types::Reason::Rejected,
                    status: None,
                    tls_cause: None,
                    retry_after: None,
                }
                .into(),
            )
            .and_then(|build| variantvalidator::normalize(&leg, query.hgvs(), build)),
    };
    let error = result.as_ref().err().copied();
    let mut source = status::source(spec.source, leg.finish(), error);
    let result = result.unwrap_or_else(|_| biomcp_adapters::Mapped {
        refusals: biomcp_adapters::Refusal::None,
        value: NormalizationResult::unavailable(spec.source),
        problems: Vec::new(),
    });
    if result.value.outcome == VariantResolution::Rejected {
        source.set_status(biomcp_types::SourceStatus::Failed(Some(
            biomcp_types::Reason::Rejected,
        )));
    }
    (result, source, error.and_then(status::local))
}

/// The most alleles resolved from one mismatch; more stay listed as fits.
const FIT_ALLELES: usize = 3;

/// Transcripts sent to VariantValidator in one fit query. Its time grows with the count:
/// CAMK2D's 47 took 25.2 s and ANK2's 73 took 11.6 s at once (job j39842, 2026-11-13), so
/// a gene's transcripts go in groups of this size, each under its own deadline. Groups of
/// 20 from CAMK2D, ANK2 and TPM1 took 1.2 to 3.6 s the same day.
const FIT_GROUP: usize = 20;

/// A service's own reference-mismatch report, by its native code or message.
fn mismatched(result: &NormalizationResult) -> bool {
    result.outcome == VariantResolution::Rejected
        && result.messages.iter().any(|message| {
            message.code.as_ref().is_some_and(|code| {
                matches!(
                    code.as_str(),
                    "IncorrectReferenceAllele" | "ESEQUENCEMISMATCH"
                )
            }) || message.text.as_str().starts_with("ReferenceMismatchError:")
        })
}

/// After a reference mismatch, VariantValidator names the gene's transcripts where the
/// given base fits. Each distinct allele among them is resolved by the same services at
/// once, and the next step reads that allele's ClinVar record; the mismatch itself stays
/// as the service reported it. Each call's source state joins `calls`.
fn transcript_fit(
    cx: &Ctx<'_>,
    query: &VariantNormalize,
    deadline: Deadline<'_>,
    rows: &mut Vec<Row>,
    calls: &mut Vec<SourceOutcome>,
    next: &mut Vec<crate::next::Step>,
) {
    if query.offset() != 0
        || !variantvalidator::fits_apply(query.hgvs())
        || cx.fetch.disabled(variantvalidator::SPEC.source)
        || !rows.iter().any(|(mapped, _, _)| mismatched(&mapped.value))
    {
        return;
    }
    let mut problems = Vec::new();
    let mut fits = find_fits(cx, query, deadline, calls, &mut problems);
    // Lowest accession number first: the longest-standing RefSeq record of the allele.
    fits.sort_by_key(|fit| accession_order(fit.hgvs.as_str()));
    let mut alleles: Vec<&biomcp_types::TranscriptFit> = Vec::new();
    for fit in &fits {
        if !alleles
            .iter()
            .any(|seen| seen.genomic.is_some() && seen.genomic == fit.genomic)
        {
            alleles.push(fit);
        }
    }
    let retried: Vec<VariantNormalize> = alleles
        .iter()
        .take(FIT_ALLELES)
        .filter_map(|fit| query.with_hgvs(fit.hgvs.as_str()))
        .collect();
    if !fits.is_empty() {
        retry(cx, deadline, &fits, &retried, rows, calls);
    }
    lead_on(cx, rows, &retried, next, &mut problems);
    if let Some((mapped, _, _)) = rows.first_mut() {
        mapped.problems.extend(problems);
    }
}

/// Ask for the gene's transcripts, then for fits in groups of [`FIT_GROUP`], each under
/// its own `transcript_fit_seconds`, stopping at the first group that fits or fails.
fn find_fits(
    cx: &Ctx<'_>,
    query: &VariantNormalize,
    deadline: Deadline<'_>,
    calls: &mut Vec<SourceOutcome>,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<biomcp_types::TranscriptFit> {
    let build = query.build().unwrap_or(biodata::Build::GRCh38);
    let leg = || {
        cx.fetch
            .leg(&variantvalidator::SPEC, deadline.child(cx.fit_deadline))
            .fresh(cx.fresh)
    };
    let listing = leg();
    let listed = variantvalidator::fit_query(&listing, query.hgvs(), build);
    let error = listed.as_ref().err().copied();
    calls.push(status::source(
        variantvalidator::SPEC.source,
        listing.finish(),
        error,
    ));
    let fit = match listed {
        Ok(listed) => {
            problems.extend(listed.problems);
            listed.value
        }
        // The failure is already in the call's source state.
        Err(_) => return Vec::new(),
    };
    let Some(fit) = fit else {
        return Vec::new();
    };
    for group in fit.transcripts.chunks(FIT_GROUP) {
        let asking = leg();
        let found = variantvalidator::transcript_fits(&asking, &fit, group, build);
        let error = found.as_ref().err().copied();
        calls.push(status::source(
            variantvalidator::SPEC.source,
            asking.finish(),
            error,
        ));
        match found {
            Ok(found) if !found.value.is_empty() => {
                problems.extend(found.problems);
                return found.value;
            }
            Ok(found) => problems.extend(found.problems),
            // The failure is already in the call's source state.
            Err(_) => break,
        }
    }
    Vec::new()
}

/// List the fits on each mismatched row and resolve every fitting allele with that row's
/// service, all at once.
fn retry(
    cx: &Ctx<'_>,
    deadline: Deadline<'_>,
    fits: &[biomcp_types::TranscriptFit],
    retried: &[VariantNormalize],
    rows: &mut Vec<Row>,
    calls: &mut Vec<SourceOutcome>,
) {
    let mut jobs: Vec<Box<dyn FnOnce() -> Row + Send + '_>> = Vec::new();
    for (mapped, _, _) in rows.iter_mut() {
        if !mismatched(&mapped.value) {
            continue;
        }
        mapped.value.fits = fits.to_vec().into();
        let Some(spec) = selected(NormalizationSource::All)
            .into_iter()
            .find(|spec| spec.source == mapped.value.source)
        else {
            continue;
        };
        for retry in retried {
            jobs.push(Box::new(move || {
                let mut row = service(cx, retry, spec, deadline);
                row.0.value.hgvs = Some(retry.hgvs().into());
                row
            }));
        }
    }
    let extra = crate::fan_out::run(jobs);
    calls.extend(extra.iter().map(|(_, source, _)| source.clone()));
    rows.extend(extra);
}

/// For each fitting allele, the next step reads its ClinVar record: by the Allele Registry
/// ID when that service resolved it, else by the fitting HGVS unless CAR failed on it.
fn lead_on(
    cx: &Ctx<'_>,
    rows: &[Row],
    retried: &[VariantNormalize],
    next: &mut Vec<crate::next::Step>,
    problems: &mut Vec<biomcp_types::Problem>,
) {
    for retry in retried {
        let car = rows.iter().find(|(mapped, _, _)| {
            mapped.value.source == biomcp_types::Source::AlleleRegistry
                && mapped
                    .value
                    .hgvs
                    .as_ref()
                    .is_some_and(|row| row.as_str() == retry.hgvs())
        });
        let input = match car {
            Some((mapped, _, _)) => match registry_id(&mapped.value) {
                Some(id) => id,
                // CAR could not resolve this allele, so a lookup through it would fail too.
                None => continue,
            },
            None => retry.hgvs(),
        };
        let source = variantvalidator::SPEC.source;
        let Some(input) = crate::next::checked(input.parse(), source, problems) else {
            continue;
        };
        let get = biomcp_types::VariantGet::new(input, biomcp_types::VariantResolver::Car, None);
        let Some(get) = crate::next::checked(get, source, problems) else {
            continue;
        };
        let get = get.with_sections(vec![biomcp_types::VariantSection::Clinvar].into());
        next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &super::next_sources(cx.fetch, &get),
            Request::GetVariant(get),
            false,
        ));
    }
}

/// The ClinGen Allele Registry ID of a resolved CAR result, from its `@id` mapping.
fn registry_id(result: &NormalizationResult) -> Option<&str> {
    if result.outcome != VariantResolution::Resolved {
        return None;
    }
    result
        .mappings
        .iter()
        .filter(|mapping| mapping.field.as_str() == "@id")
        .find_map(|mapping| mapping.value.as_str().rsplit_once("/allele/"))
        .map(|(_, id)| id)
        .filter(|id| id.starts_with("CA"))
}

fn accession_order(hgvs: &str) -> (u64, u32) {
    let reference = hgvs
        .split_once(':')
        .map_or(hgvs, |(reference, _)| reference);
    let (base, version) = reference.split_once('.').unwrap_or((reference, "0"));
    (
        base.trim_start_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .unwrap_or(u64::MAX),
        version.parse().unwrap_or(0),
    )
}
