//! Require a caller key and a source-stated genomic allele before scoring.
use crate::{Ctx, status};
use biodata::{Build, Chromosome};
use biomcp_adapters::alphagenome;
use biomcp_fetch::{Deadline, SecretName, grpc::alphagenome::ScoreRequest};
use biomcp_types::{
    Reason, Section, Source, SourceOutcome, SourceState, VariantAlphaGenome, VariantGet,
    VariantHeader,
};

pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
) -> super::sections::Result<VariantAlphaGenome> {
    if !cx.fetch.has_key(SecretName::AlphaGenomeApiKey) {
        return refused(Reason::NoKey, SourceState::Skipped);
    }
    if query.build().is_some_and(|build| build != Build::GRCh38) {
        return refused(Reason::Unsupported, SourceState::Failed);
    }
    let Some(request) = candidates.first().and_then(request) else {
        return refused(Reason::NoJoinId, SourceState::Failed);
    };
    let leg = cx.fetch.leg(&alphagenome::SPEC, deadline).fresh(cx.fresh);
    let reply = leg
        .grpc_checked(&alphagenome::SCORE_VARIANT, request.bytes(), |limits| {
            request.decoder(limits)
        })
        .and_then(|reply| super::alphagenome_page::page(query, &request, reply));
    let error = reply.as_ref().err().copied();
    let source = status::source(Source::AlphaGenome, leg.finish(), error);
    let section = match reply {
        Ok(data) => Section::Data {
            source: Source::AlphaGenome,
            data,
        },
        Err(error) => Section::unavailable(
            Source::AlphaGenome,
            status::reason(error).unwrap_or(Reason::Unavailable),
        ),
    };
    (
        section,
        vec![source],
        Vec::new(),
        error.and_then(status::local),
    )
}

fn request(row: &VariantHeader) -> Option<ScoreRequest> {
    let mut requests = row
        .identity
        .representations
        .iter()
        .filter_map(|representation| {
            if representation.build != Some(Build::GRCh38) {
                return None;
            }
            let allele = representation.allele.as_ref()?;
            let chromosome =
                Chromosome::from_accession(allele.location().sequence, Build::GRCh38).ok()?;
            ScoreRequest::human_legacy(
                Build::GRCh38,
                chromosome,
                i64::from(allele.location().span.start()) + 1,
                &allele.reference().to_string(),
                &allele.alternate().to_string(),
            )
            .ok()
        });
    let first = requests.next()?;
    // Multiple source representations may repeat one identity, but never select between alleles.
    requests
        .all(|request| request.bytes() == first.bytes())
        .then_some(first)
}

fn refused(reason: Reason, state: SourceState) -> super::sections::Result<VariantAlphaGenome> {
    let section = if state == SourceState::Skipped {
        Section::Skipped {
            source: Source::AlphaGenome,
            reason,
        }
    } else {
        Section::unavailable(Source::AlphaGenome, reason)
    };
    (
        section,
        vec![biomcp_adapters::registry::outcome(
            Source::AlphaGenome,
            state,
            Some(reason),
        )],
        Vec::new(),
        None,
    )
}

pub(super) fn next(sources: &[SourceOutcome]) -> Vec<(biomcp_types::StepKind, crate::Action)> {
    if sources
        .iter()
        .any(|source| source.source == Source::AlphaGenome && source.reason == Some(Reason::NoKey))
    {
        vec![(
            biomcp_types::StepKind::Key,
            crate::Action::KeyHelp(Source::AlphaGenome),
        )]
    } else {
        Vec::new()
    }
}
