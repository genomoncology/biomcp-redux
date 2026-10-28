use super::*;

use biomcp_types::{
    NormalizationField, NormalizationMessage, NormalizationResult, VariantResolution,
};
/// Keep each provider mapping and warning; never select a transcript.
pub fn normalize(
    leg: &Leg<'_>,
    input: &str,
    build: Build,
) -> Result<NormalizationResult, RequestError> {
    let raw = fetch(leg, input, build)?;
    let mut result = NormalizationResult::unavailable(Source::VariantValidator);
    result.outcome = if raw.flag.as_str() == "warning" {
        VariantResolution::Rejected
    } else {
        VariantResolution::Resolved
    };
    result.version = raw
        .metadata
        .into_iter()
        .map(|(field, value)| NormalizationField {
            field: field.into(),
            value: value.into(),
        })
        .collect();
    let mut mappings = Vec::new();
    let mut messages = Vec::new();
    for (key, row) in raw.variants {
        for text in row.validation_warnings {
            messages.push(NormalizationMessage {
                category: "validation_warnings".into(),
                code: None,
                text: text.into(),
            });
        }
        for (field, value) in [
            ("hgvs_transcript_variant", Some(row.hgvs_transcript_variant)),
            (
                "hgvs_lrg_transcript_variant",
                row.hgvs_lrg_transcript_variant,
            ),
            ("hgvs_lrg_variant", row.hgvs_lrg_variant),
            ("hgvs_refseqgene_variant", row.hgvs_refseqgene_variant),
        ] {
            if let Some(value) = value.filter(|s| !s.is_empty()) {
                mappings.push(NormalizationField {
                    field: format!("{key}.{field}").into(),
                    value: value.into(),
                });
            }
        }
        for (field, value) in row.hgvs_predicted_protein_consequence.into_iter().flatten() {
            if !value.is_empty() {
                mappings.push(NormalizationField {
                    field: format!("{key}.hgvs_predicted_protein_consequence.{field}").into(),
                    value: value.into(),
                });
            }
        }
        for (assembly, locus) in row.primary_assembly_loci.into_iter().flatten() {
            mappings.push(NormalizationField {
                field: format!("{key}.primary_assembly_loci.{assembly}").into(),
                value: locus.hgvs_genomic_description.to_string().into(),
            });
        }
        for loci in row.alt_genomic_loci.into_iter().flatten() {
            for (assembly, locus) in loci {
                mappings.push(NormalizationField {
                    field: format!("{key}.alt_genomic_loci.{assembly}").into(),
                    value: locus.hgvs_genomic_description.to_string().into(),
                });
            }
        }
    }
    result.total_mappings = mappings.len() as u32;
    result.mappings = mappings.into();
    result.messages = messages.into();
    Ok(result)
}
