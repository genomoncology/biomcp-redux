use super::{Leg, RequestError, Source, fetch};

use biomcp_types::{
    NormalizationField, NormalizationMessage, NormalizationResult, VariantResolution,
};
/// Preserve source descriptions and messages without deriving a local equivalence.
pub fn normalize(leg: &Leg<'_>, input: &str) -> Result<NormalizationResult, RequestError> {
    let raw = fetch(leg, input)?;
    let mut result = NormalizationResult::unavailable(Source::Mutalyzer);
    result.outcome = if raw.errors().is_empty() {
        VariantResolution::Resolved
    } else {
        VariantResolution::Rejected
    };
    result.normalized = raw.normalized_description.map(Into::into);
    result.corrected = raw.corrected_description.map(Into::into);
    let mut mappings = Vec::new();
    for (field, description) in [
        ("protein.description", raw.protein),
        ("rna.description", raw.rna),
    ] {
        if let Some(d) = description {
            mappings.push(NormalizationField {
                field: field.into(),
                value: d.description.into(),
            });
        }
    }
    for (kind, descriptions) in raw.equivalent_descriptions.into_iter().flatten() {
        for d in descriptions {
            mappings.push(NormalizationField {
                field: format!("equivalent_descriptions.{kind}").into(),
                value: d.description.into(),
            });
        }
    }
    for value in raw.back_translated_descriptions.into_iter().flatten() {
        mappings.push(NormalizationField {
            field: "back_translated_descriptions".into(),
            value: value.into(),
        });
    }
    result.total_mappings = mappings.len() as u32;
    result.mappings = mappings.into();
    let mut messages = Vec::new();
    for (category, rows) in [
        ("errors", raw.errors),
        ("warnings", raw.warnings),
        ("infos", raw.infos),
    ] {
        for m in rows.into_iter().flatten() {
            messages.push(NormalizationMessage {
                category: category.into(),
                code: Some(m.code.to_string().into()),
                text: m.details.unwrap_or_default().into(),
            });
        }
    }
    result.messages = messages.into();
    if let Some(value) = raw.version {
        result.version = vec![NormalizationField {
            field: "version".into(),
            value: value.into(),
        }]
        .into();
    }
    Ok(result)
}
