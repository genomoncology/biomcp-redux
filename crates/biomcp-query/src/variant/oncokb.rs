//! Missing keys and disabled policy stop before source, cache and replay access.
use crate::{Ctx, status};
use biodata::Build;
use biomcp_adapters::oncokb;
use biomcp_fetch::{Deadline, SecretName};
use biomcp_types::{
    PageInfo, Reason, Section, Source, SourceOutcome, SourceState, VariantGet, VariantHeader,
    VariantOncokb, VariantSection,
};

pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
) -> super::sections::Result<VariantOncokb> {
    if cx.fetch.disabled(Source::Oncokb) {
        return skipped(Reason::Disabled);
    }
    if !cx.fetch.has_key(SecretName::OncokbApiKey) {
        return skipped(Reason::NoKey);
    }
    let (Some(build), Some(tumor)) = (query.build(), query.tumor_type()) else {
        return skipped(Reason::NoJoinId);
    };
    let Some(hgvsg) = candidates
        .first()
        .and_then(|row| genomic(row, build))
        .or_else(|| input_genomic(query, build))
    else {
        return unavailable(Reason::NoJoinId);
    };
    let leg = cx.fetch.leg(&oncokb::SPEC, deadline).fresh(cx.fresh);
    let reply = oncokb::variant::annotation(&leg, &hgvsg, build, tumor.as_str());
    let error = reply.as_ref().err().copied();
    let source = status::source(Source::Oncokb, leg.finish(), error);
    let mut problems = Vec::new();
    let section = match reply {
        Ok(row) => {
            let mapped = oncokb::variant_map::records(&row);
            problems = mapped.problems;
            let total = mapped.value.len() as u64;
            let rows = mapped
                .value
                .into_iter()
                .skip(query.section_offset() as usize)
                .take(query.section_limit() as usize)
                .collect::<Vec<_>>();
            let returned = rows.len() as u32;
            let next_offset = u64::from(query.section_offset()) + u64::from(returned);
            Section::Data {
                source: Source::Oncokb,
                data: VariantOncokb {
                    hgvsg: row.query.hgvs,
                    build,
                    tumor_type: tumor.clone(),
                    data_version: row.data_version.and_then(|s| s.parse().ok()),
                    last_update: row.last_update.and_then(|s| s.parse().ok()),
                    variant_exist: row.variant_exist,
                    gene_exist: row.gene_exist,
                    vus: row.vus,
                    hotspot: row.hotspot,
                    allele_exist: row.allele_exist,
                    records: rows.into(),
                    page: PageInfo {
                        offset: query.section_offset(),
                        returned,
                        total,
                        pool: None,
                        truncated: None,
                    },
                    next: if next_offset < total {
                        Some(
                            query
                                .at_section_offset(next_offset as u32)
                                .with_sections(vec![VariantSection::Oncokb].into()),
                        )
                    } else {
                        None
                    },
                },
            }
        }
        Err(error) => Section::unavailable(
            Source::Oncokb,
            crate::status::reason(error).unwrap_or(biomcp_types::Reason::Unavailable),
        ),
    };
    (
        section,
        vec![source],
        problems,
        error.and_then(status::local),
    )
}
fn genomic(row: &VariantHeader, build: Build) -> Option<String> {
    row.identity.representations.iter().find_map(|r| {
        if r.build != Some(build) || r.expression.syntax() != biodata::ExpressionSyntax::HgvsG {
            return None;
        }
        let (reference, body) = r.expression.value().as_str().split_once(':')?;
        let chromosome = if let Some(allele) = &r.allele {
            biodata::Chromosome::from_accession(allele.location().sequence, build).ok()?
        } else {
            reference
                .strip_prefix("chr")
                .unwrap_or(reference)
                .parse()
                .ok()?
        };
        Some(format!("{}:{body}", chromosome.name()))
    })
}
fn skipped(reason: Reason) -> super::sections::Result<VariantOncokb> {
    (
        Section::Skipped {
            source: Source::Oncokb,
            reason,
        },
        vec![source(SourceState::Skipped, reason)],
        Vec::new(),
        None,
    )
}
fn unavailable(reason: Reason) -> super::sections::Result<VariantOncokb> {
    (
        Section::unavailable(Source::Oncokb, reason),
        vec![source(SourceState::Failed, reason)],
        Vec::new(),
        None,
    )
}
fn source(state: SourceState, reason: Reason) -> SourceOutcome {
    biomcp_adapters::registry::outcome(Source::Oncokb, state, Some(reason))
}

pub(super) fn input_genomic(query: &VariantGet, build: Build) -> Option<String> {
    let (reference, body) = query.input().as_str().split_once(':')?;
    if !body.starts_with("g.") {
        return None;
    }
    let chromosome = if let Ok(accession) = reference.parse::<biodata::Accession>() {
        biodata::Chromosome::from_accession(accession, build).ok()?
    } else {
        reference
            .strip_prefix("chr")
            .unwrap_or(reference)
            .parse()
            .ok()?
    };
    Some(format!("{}:{body}", chromosome.name()))
}

/// Offer setup only for the missing caller context; never supply a default.
pub(super) fn next(
    query: &VariantGet,
    sources: &[SourceOutcome],
) -> Vec<(biomcp_types::StepKind, crate::Action)> {
    if sources
        .iter()
        .any(|s| s.source == Source::Oncokb && s.reason == Some(Reason::NoKey))
    {
        vec![(
            biomcp_types::StepKind::Key,
            crate::Action::KeyHelp(Source::Oncokb),
        )]
    } else if (query.build().is_none() || query.tumor_type().is_none())
        && sources
            .iter()
            .any(|s| s.source == Source::Oncokb && s.reason == Some(Reason::NoJoinId))
    {
        vec![(
            biomcp_types::StepKind::Configure,
            crate::Action::QueryHelp(biomcp_types::Request::GetVariant(query.clone())),
        )]
    } else {
        Vec::new()
    }
}
