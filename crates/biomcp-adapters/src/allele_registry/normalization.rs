//! Preserve independent native registry mappings and validation messages.
use super::raw;
use biomcp_types::{NormalizationField, Source, VariantResolution};
use std::collections::HashSet;

pub(super) fn map(
    raw: raw::Allele,
    protein: bool,
) -> crate::Mapped<biomcp_types::NormalizationResult> {
    let mut refusals = crate::Refusal::None;
    let mut result = biomcp_types::NormalizationResult::unavailable(Source::AlleleRegistry);
    let mut problems = crate::mapped::Problems {
        source: Source::AlleleRegistry,
        record: None,
        list: Vec::new(),
    };
    if let Some(code) = &raw.error_type {
        result.outcome = biomcp_types::VariantResolution::Rejected;
        let mut messages = Vec::new();
        // The registry may return a code without a description; preserve its empty native body.
        let description = match raw.error_description {
            None => Some("".into()),
            Some(value) => problems.field("description", Some(value)),
        };
        if let Some(text) = description {
            messages.push(biomcp_types::NormalizationMessage {
                category: "description".into(),
                code: Some(code.to_string().into()),
                text,
            });
        }
        if let Some(text) = problems.field("message", raw.message) {
            messages.push(biomcp_types::NormalizationMessage {
                category: "message".into(),
                code: None,
                text,
            });
        }
        result.messages = messages.into();
    } else if protein {
        if let Some((record, allele_type)) = problems.optional("@id", Some(raw.protein_identity()))
        {
            problems.record = Some(record);
            let candidates = protein_candidates(raw.proteins, &mut problems);
            result.outcome = if candidates.is_empty() {
                VariantResolution::Resolved
            } else {
                VariantResolution::Candidates
            };
            let mut mappings = identity_mapping(raw.id);
            mappings.push(NormalizationField {
                field: "type".into(),
                value: allele_type.to_string().into(),
            });
            mappings.extend(candidates);
            result.total_mappings = mappings.len() as u32;
            result.mappings = mappings.into();
        } else {
            refusals = crate::Refusal::Refused;
        }
    } else if let Some((_, record, _)) = problems.optional("@id", Some(raw.identity(None))) {
        problems.record = Some(record);
        result.outcome = biomcp_types::VariantResolution::Resolved;
        let mappings = mappings(raw, &mut problems);
        result.total_mappings = mappings.len() as u32;
        result.mappings = mappings.into();
    } else {
        refusals = crate::Refusal::Refused;
    }
    crate::Mapped {
        refusals,
        value: result,
        problems: problems.list,
    }
}

fn protein_candidates(
    proteins: Option<crate::raw::Field<Vec<crate::raw::Field<raw::Protein>>>>,
    problems: &mut crate::mapped::Problems,
) -> Vec<NormalizationField> {
    const MATCHES: &str = "aminoAcidAlleles.matchingRegisteredTranscripts";
    const ID: &str = "aminoAcidAlleles.matchingRegisteredTranscripts.@id";
    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    for protein in problems.fields("aminoAcidAlleles", proteins) {
        for link in problems.fields(MATCHES, protein.matches) {
            let Some(url) = problems.required_field(ID, link.id) else {
                continue;
            };
            let id = raw::allele_id(&url).and_then(|(_, id)| match id {
                biodata::VariantId::Ca(id) => Ok(id),
                _ => Err(biomcp_fetch::BadReply),
            });
            if problems
                .optional(ID, Some(id))
                .is_some_and(|id| seen.insert(id))
            {
                candidates.push(NormalizationField {
                    field: ID.into(),
                    value: url.to_string().into(),
                });
            }
        }
    }
    candidates
}

fn identity_mapping(id: Option<crate::raw::Field<biodata::Text>>) -> Vec<NormalizationField> {
    id.and_then(crate::raw::Field::value)
        .map(|value| NormalizationField {
            field: "@id".into(),
            value: value.to_string().into(),
        })
        .into_iter()
        .collect()
}

fn mappings(raw: raw::Allele, problems: &mut crate::mapped::Problems) -> Vec<NormalizationField> {
    let mut mappings = identity_mapping(raw.id);
    for row in problems.fields("genomicAlleles", raw.genomic) {
        let assembly = problems.field("genomicAlleles.referenceGenome", row.assembly);
        for value in problems.fields("genomicAlleles.hgvs", row.hgvs) {
            let expression = problems.optional(
                "genomicAlleles.hgvs",
                Some(biodata::Expression::hgvs(value.clone())),
            );
            if expression.is_none() {
                continue;
            }
            mappings.push(biomcp_types::NormalizationField {
                field: assembly.as_ref().map_or_else(
                    || "genomicAlleles.hgvs".into(),
                    |b| format!("genomicAlleles.{}.hgvs", b.as_str()).into(),
                ),
                value: value.to_string().into(),
            });
        }
    }
    for row in problems.fields("transcriptAlleles", raw.transcripts) {
        if let Some(mane) = problems.field("transcriptAlleles.MANE", row.mane) {
            for forms in [mane.nucleotide, mane.protein].into_iter().flatten() {
                if let Some(forms) = problems.field("transcriptAlleles.MANE", Some(forms)) {
                    for hgvs in [forms.ensembl, forms.refseq].into_iter().flatten() {
                        let hgvs = problems.field("transcriptAlleles.MANE.hgvs", Some(hgvs));
                        if let Some(hgvs) = hgvs.and_then(|form| {
                            problems.field("transcriptAlleles.MANE.hgvs", Some(form.hgvs))
                        }) {
                            if problems
                                .optional(
                                    "transcriptAlleles.MANE.hgvs",
                                    Some(biodata::Expression::hgvs(hgvs.clone())),
                                )
                                .is_none()
                            {
                                continue;
                            }
                            mappings.push(biomcp_types::NormalizationField {
                                field: "transcriptAlleles.MANE.hgvs".into(),
                                value: hgvs.to_string().into(),
                            });
                        }
                    }
                }
            }
        }
    }
    mappings
}
