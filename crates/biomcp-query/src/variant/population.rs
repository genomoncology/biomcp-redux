//! Exact GRCh38 source joins and bounded population observations.
use crate::{Ctx, status};
use biodata::{
    Attribution, Build, Chromosome, PopulationFrequency, Problem, ProblemCode, Reason, Section,
    Source, SourceOutcome, Text, VariantGet, VariantHeader, VariantPopulation, VariantRef,
};
use biomcp_adapters::gnomad;
use biomcp_fetch::{Deadline, RequestError};
/// Read only gnomAD after one unambiguous source allele has been established.
pub(super) fn section(
    cx: &Ctx<'_>,
    _query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
) -> (
    Section<VariantPopulation>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
) {
    let Some((id, subject)) = join(candidates) else {
        return missing();
    };
    let leg = cx.fetch.leg(&gnomad::SPEC, deadline).fresh(cx.fresh);
    let result = gnomad::variant::population(&leg, &id)
        .and_then(|raw| raw.map(|raw| summary(&raw, &subject)).transpose());
    let error = result.as_ref().err().copied();
    let source = status::source(Source::Gnomad, leg.finish(), error);
    let value = match result {
        Ok(Some(data)) => Section::Data {
            source: Source::Gnomad,
            data,
        },
        Ok(None) => Section::Empty {
            source: Source::Gnomad,
            record: Text::new(&id)
                .ok()
                .map(|id| biodata::SourceRecord { id, url: None }),
        },
        Err(e) => Section::unavailable(Source::Gnomad, reason(e)),
    };
    (
        value,
        vec![source],
        Vec::new(),
        error.and_then(status::local),
    )
}
/// Keep only source-qualified SNVs; indels need an explicit padded source representation.
pub(super) fn join(candidates: &[VariantHeader]) -> Option<(String, VariantRef)> {
    let [row] = candidates else {
        return None;
    };
    let mut forms = std::collections::BTreeSet::new();
    for representation in &row.identity.representations {
        if representation.build != Some(Build::Grch38) {
            continue;
        }
        let Some(allele) = &representation.allele else {
            continue;
        };
        let chromosome =
            Chromosome::from_accession(allele.location().sequence, Build::Grch38).ok()?;
        let reference = allele.reference().to_string();
        let alternate = allele.alternate().to_string();
        if allele.location().span.end() - allele.location().span.start() != 1
            || reference.len() != 1
            || alternate.len() != 1
            || !reference
                .bytes()
                .chain(alternate.bytes())
                .all(|b| b"ACGT".contains(&b))
        {
            continue;
        }
        forms.insert(format!(
            "{}-{}-{reference}-{alternate}",
            chromosome.name(),
            allele.location().span.start() + 1
        ));
    }
    if forms.len() != 1 {
        return None;
    }
    let subject = row
        .identity
        .ids
        .first()
        .cloned()
        .map(VariantRef::allele)
        .or_else(|| {
            row.identity.representations.iter().find_map(|r| {
                r.expression
                    .value()
                    .as_str()
                    .parse()
                    .ok()
                    .map(VariantRef::allele_hgvs)
            })
        })?;
    Some((forms.into_iter().next()?, subject))
}
pub(super) fn summary(
    raw: &gnomad::variant::Variant,
    subject: &VariantRef,
) -> Result<VariantPopulation, RequestError> {
    Ok(VariantPopulation {
        variant: raw.variant_id.clone(),
        dataset: text(gnomad::variant::DATASET)?,
        build: Build::Grch38,
        exome: raw
            .exome
            .as_ref()
            .map(|s| observation(&raw.variant_id, subject, None, s.ac, s.an, &s.filters))
            .transpose()?
            .map(Box::new),
        genome: raw
            .genome
            .as_ref()
            .map(|s| observation(&raw.variant_id, subject, None, s.ac, s.an, &s.filters))
            .transpose()?
            .map(Box::new),
    })
}
pub(super) fn observation(
    id: &Text,
    subject: &VariantRef,
    group: Option<Text>,
    ac: u64,
    an: u64,
    filters: &[Text],
) -> Result<PopulationFrequency, RequestError> {
    Ok(PopulationFrequency {
        subject: subject.clone(),
        dataset: text(gnomad::variant::DATASET)?,
        build: Build::Grch38,
        group,
        allele_count: Some(ac),
        allele_number: Some(an),
        homozygotes: None,
        frequency: None,
        filters: filters.into(),
        attribution: Attribution {
            source: Source::Gnomad,
            record: Some(id.clone()),
            version: None,
            release: None,
            submitter: None,
            evaluated: None,
            url: Some(text(&format!(
                "https://gnomad.broadinstitute.org/variant/{id}?dataset={}",
                gnomad::variant::DATASET
            ))?),
        },
    })
}
fn missing() -> (
    Section<VariantPopulation>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
) {
    (
        Section::Unavailable {
            source: Source::Gnomad,
            reason: Reason::NoJoinId,
        },
        Vec::new(),
        vec![Problem {
            source: Source::Gnomad,
            record: None,
            field: "GRCh38_allele".into(),
            code: ProblemCode::ValueUnknown,
        }],
        None,
    )
}
fn text(value: &str) -> Result<Text, RequestError> {
    Text::new(value).map_err(|_| {
        biomcp_fetch::SourceError {
            source: Source::Gnomad,
            reason: Reason::BadReply,
            status: None,
            retry_after: None,
        }
        .into()
    })
}
fn reason(error: RequestError) -> Reason {
    match error {
        RequestError::Source(e) => e.reason,
        _ => Reason::Unavailable,
    }
}
