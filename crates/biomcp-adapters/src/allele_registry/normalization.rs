//! Preserve independent native registry mappings and validation messages.
use super::raw;
use biomcp_types::Source;
pub(super) fn map(raw: raw::Allele) -> crate::Mapped<biomcp_types::NormalizationResult> {
    let mut result = biomcp_types::NormalizationResult::unavailable(Source::AlleleRegistry);
    let mut problems = crate::mapped::Problems {
        source: Source::AlleleRegistry,
        record: raw.id.as_ref().and_then(crate::mapped::record),
        list: Vec::new(),
    };
    if let Some(code) = raw.error_type {
        result.outcome = biomcp_types::VariantResolution::Rejected;
        let mut messages = vec![biomcp_types::NormalizationMessage {
            category: "description".into(),
            code: Some(code.to_string().into()),
            text: raw.error_description.unwrap_or_default().into(),
        }];
        if let Some(text) = raw.message {
            messages.push(biomcp_types::NormalizationMessage {
                category: "message".into(),
                code: None,
                text: text.into(),
            });
        }
        result.messages = messages.into();
    } else {
        result.outcome = biomcp_types::VariantResolution::Resolved;
        let mut mappings = Vec::new();
        if let Some(value) = raw.id {
            mappings.push(biomcp_types::NormalizationField {
                field: "@id".into(),
                value: value.to_string().into(),
            });
        }
        for row in problems.fields("genomicAlleles", raw.genomic) {
            let assembly = problems.field("genomicAlleles.referenceGenome", row.assembly);
            for value in problems.fields("genomicAlleles.hgvs", row.hgvs) {
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
        result.total_mappings = mappings.len() as u32;
        result.mappings = mappings.into();
    }
    crate::Mapped {
        value: result,
        problems: problems.list,
    }
}
