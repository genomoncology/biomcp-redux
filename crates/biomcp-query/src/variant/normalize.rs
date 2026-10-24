use crate::{Action, Ctx, Outcome, status};
use biodata::{
    NormalizationSource, QueryBody, Request, SourceState, Status, StepKind, VariantNormalization,
    VariantNormalize, VariantResolution,
};
use biomcp_adapters::{allele_registry, mutalyzer, variantvalidator};
/// Call each explicitly selected service independently under the shared deadline.
pub fn normalize(cx: &Ctx<'_>, query: &VariantNormalize) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let selected = match query.source() {
        NormalizationSource::Car => vec![&allele_registry::SPEC],
        NormalizationSource::Mutalyzer => vec![&mutalyzer::SPEC],
        NormalizationSource::Variantvalidator => vec![&variantvalidator::SPEC],
        NormalizationSource::All => vec![
            &allele_registry::SPEC,
            &mutalyzer::SPEC,
            &variantvalidator::SPEC,
        ],
    };
    let jobs = selected
        .into_iter()
        .map(|spec| {
            Box::new(move || {
                let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
                let result = match spec.source {
                    biodata::Source::AlleleRegistry => {
                        allele_registry::normalize(&leg, query.hgvs().as_str())
                    }
                    biodata::Source::Mutalyzer => mutalyzer::normalize(&leg, query.hgvs().as_str()),
                    _ => query
                        .build()
                        .ok_or(
                            biomcp_fetch::SourceError {
                                source: spec.source,
                                reason: biodata::Reason::Rejected,
                                status: None,
                                retry_after: None,
                            }
                            .into(),
                        )
                        .and_then(|build| {
                            variantvalidator::normalize(&leg, query.hgvs().as_str(), build)
                        }),
                };
                let error = result.as_ref().err().copied();
                let mut source = status::source(spec.source, leg.finish(), error);
                let result = result
                    .unwrap_or_else(|_| biodata::NormalizationResult::unavailable(spec.source));
                if result.outcome == VariantResolution::Rejected {
                    source.state = SourceState::Failed;
                    source.reason = Some(biodata::Reason::Rejected);
                }
                (result, source, error.and_then(status::local))
            }) as Box<dyn FnOnce() -> _ + Send>
        })
        .collect();
    let rows = crate::fan_out::run(jobs);
    let mut results = Vec::new();
    let mut sources = Vec::new();
    let mut local = None;
    let mut more = false;
    let mut states = Vec::new();
    for (mut result, source, error) in rows {
        states.push(
            if matches!(
                result.outcome,
                VariantResolution::Rejected | VariantResolution::Unavailable
            ) {
                Status::Failed
            } else {
                Status::Complete
            },
        );
        result.total_messages = result.messages.len() as u32;
        more |= result.total_messages as u64 > query.offset() as u64 + query.limit() as u64;
        result.messages = result
            .messages
            .into_vec()
            .into_iter()
            .skip(query.offset() as usize)
            .take(query.limit() as usize)
            .collect();
        more |= result.total_mappings as u64 > query.offset() as u64 + query.limit() as u64;
        result.mappings = result
            .mappings
            .into_vec()
            .into_iter()
            .skip(query.offset() as usize)
            .take(query.limit() as usize)
            .collect();
        results.push(result);
        sources.push(source);
        local = local.or(error);
    }
    let next = if more {
        vec![(
            StepKind::Page,
            Action::Run {
                request: Request::VariantNormalize(
                    query.at_offset(query.offset().saturating_add(query.limit())),
                ),
                fresh: false,
            },
        )]
    } else {
        Vec::new()
    };
    let outcome = Outcome {
        status: status::status(states.into_iter()),
        body: QueryBody::VariantNormalization(VariantNormalization {
            results: results.into(),
        }),
        sources,
        problems: Vec::new(),
        next,
        page: None,
        local,
    };
    command.finish(deadline.remaining());
    outcome
}
