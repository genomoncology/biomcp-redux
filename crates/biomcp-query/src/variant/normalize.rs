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
    let mut next = Vec::new();
    let fit_source = transcript_fit(cx, query, deadline, &mut rows, &mut next);
    let mut sources = Vec::new();
    let mut results = Vec::new();
    let mut local = None;
    let mut more = false;
    let mut states = Vec::new();
    let mut problems = Vec::new();
    for (mapped, source, error) in rows {
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
        sources.push(source);
        local = local.or(error);
    }
    sources.extend(fit_source);
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
/// given base fits. Each distinct allele among them is resolved by the same service and
/// offered as the next step; the mismatch itself stays as the service reported it.
fn transcript_fit(
    cx: &Ctx<'_>,
    query: &VariantNormalize,
    deadline: Deadline<'_>,
    rows: &mut Vec<Row>,
    next: &mut Vec<crate::next::Step>,
) -> Option<SourceOutcome> {
    if query.offset() != 0
        || !variantvalidator::fits_apply(query.hgvs())
        || cx.fetch.disabled(variantvalidator::SPEC.source)
        || !rows.iter().any(|(mapped, _, _)| mismatched(&mapped.value))
    {
        return None;
    }
    let build = query.build().unwrap_or(biodata::Build::GRCh38);
    let leg = cx
        .fetch
        .leg(&variantvalidator::SPEC, deadline)
        .fresh(cx.fresh);
    let result = variantvalidator::transcript_fits(&leg, query.hgvs(), build);
    let error = result.as_ref().err().copied();
    let outcome = status::source(variantvalidator::SPEC.source, leg.finish(), error);
    let Ok(found) = result else {
        return Some(outcome);
    };
    let mut fits = found.value;
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
    let mut extra = Vec::new();
    for (mapped, _, _) in rows.iter_mut() {
        if !mismatched(&mapped.value) {
            continue;
        }
        mapped.value.fits = fits.clone().into();
        let spec = selected(NormalizationSource::All)
            .into_iter()
            .find(|spec| spec.source == mapped.value.source);
        for retry in &retried {
            if let Some(spec) = spec {
                let mut row = service(cx, retry, spec, deadline);
                row.0.value.hgvs = Some(retry.hgvs().into());
                extra.push(row);
            }
        }
    }
    rows.extend(extra);
    if let Some((mapped, _, _)) = rows.first_mut() {
        mapped.problems.extend(found.problems);
    }
    for retry in retried {
        next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &selected(query.source())
                .iter()
                .map(|spec| spec.source)
                .collect::<Vec<_>>(),
            Request::VariantNormalize(retry),
            false,
        ));
    }
    Some(outcome)
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
