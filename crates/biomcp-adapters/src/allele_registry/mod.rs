//! Read-only ClinGen Allele Registry resolution, without local allele repair.
mod map;
mod raw;
use crate::quotas;
use biodata::{Source, Text, VariantHeader};
use biomcp_fetch::{Arg, Endpoint, KeyUse, Leg, PathSegment, RequestError, SourceSpec};
use std::time::Duration;
/// Public GET access; raw output redistribution rights remain unverified.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "ClinGen Allele Registry service terms; output rights require review",
        url: "https://clinicalgenome.org/about/terms-of-use/",
        usage: biodata::Use::Restricted,
    },
    source: Source::AlleleRegistry,
    base: "https://reg.genome.network",
    key: KeyUse::None,
    quota: &quotas::ALLELE_REGISTRY,
    deadline: Duration::from_secs(15),
};
const ALLELE: Endpoint = SPEC
    .get("/allele")
    .json_rejections()
    .fresh(biomcp_fetch::Freshness::Days(1));
const FIELDS: &str = "none+@id+communityStandardTitle+genomicAlleles+transcriptAlleles.MANE+externalRecords.dbSNP+externalRecords.ClinVarVariations";
/// Checked service result including a source validation code.
#[derive(Debug)]
pub enum Resolution {
    /// Source-resolved canonical identity.
    Found(crate::Mapped<VariantHeader>),
    /// Source rejected the unchanged notation.
    Rejected(Text),
}
/// Resolve unchanged HGVS through the declared GET endpoint.
pub fn hgvs(leg: &Leg<'_>, input: &str) -> Result<Resolution, RequestError> {
    map::allele(read(leg, input, false)?).map_err(|_| bad())
}
/// Read a canonical source record directly, without another resolver.
pub fn by_id(leg: &Leg<'_>, input: &str) -> Result<Resolution, RequestError> {
    map::allele(read(leg, input, true)?).map_err(|_| bad())
}
fn read(leg: &Leg<'_>, input: &str, by_id: bool) -> Result<raw::Allele, RequestError> {
    let endpoint = if by_id {
        ALLELE.clone().with_segments(&[PathSegment::new(input)?])
    } else {
        ALLELE.clone()
    };
    let mut params = vec![("fields", Arg::Text(FIELDS))];
    if !by_id {
        params.insert(0, ("hgvs", Arg::Text(input)));
    }
    let reply = leg.json_checked_admission::<raw::Allele>(
        &endpoint,
        &params,
        |r| {
            if !by_id
                || r.error_type.is_some()
                || r.id
                    .as_ref()
                    .is_some_and(|url| url.as_str().rsplit('/').next() == Some(input))
            {
                Ok(())
            } else {
                Err(biomcp_fetch::BadReply)
            }
        },
        |r| r.error_type.is_none(),
    )?;
    Ok(reply)
}

fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::AlleleRegistry,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}

/// Preserve the registry's own mapped forms and validation rejection.
pub fn normalize(leg: &Leg<'_>, input: &str) -> Result<biodata::NormalizationResult, RequestError> {
    let mut result = biodata::NormalizationResult::unavailable(Source::AlleleRegistry);
    let raw = read(leg, input, false)?;
    if let Some(code) = raw.error_type {
        result.outcome = biodata::VariantResolution::Rejected;
        let mut messages = vec![biodata::NormalizationMessage {
            category: "description".into(),
            code: Some(code.to_string()),
            text: raw.error_description.unwrap_or_default(),
        }];
        if let Some(text) = raw.message {
            messages.push(biodata::NormalizationMessage {
                category: "message".into(),
                code: None,
                text,
            });
        }
        result.messages = messages.into();
    } else {
        result.outcome = biodata::VariantResolution::Resolved;
        let mut mappings = Vec::new();
        if let Some(value) = raw.id {
            mappings.push(biodata::NormalizationField {
                field: "@id".into(),
                value: value.to_string(),
            });
        }
        for row in raw.genomic.into_iter().flatten() {
            for value in row.hgvs {
                mappings.push(biodata::NormalizationField {
                    field: row.assembly.as_ref().map_or_else(
                        || "genomicAlleles.hgvs".into(),
                        |b| format!("genomicAlleles.{}.hgvs", b.as_str()),
                    ),
                    value: value.to_string(),
                });
            }
        }
        for row in raw.transcripts.into_iter().flatten() {
            if let Some(mane) = row.mane {
                for forms in [mane.nucleotide, mane.protein].into_iter().flatten() {
                    for hgvs in [forms.ensembl, forms.refseq].into_iter().flatten() {
                        mappings.push(biodata::NormalizationField {
                            field: "transcriptAlleles.MANE.hgvs".into(),
                            value: hgvs.hgvs.to_string(),
                        });
                    }
                }
            }
        }
        result.total_mappings = mappings.len() as u32;
        result.mappings = mappings.into();
    }
    Ok(result)
}
