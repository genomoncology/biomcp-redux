//! Preserve the registry's gnomAD v4 cross-reference, never infer padding or alignment.
use crate::{Mapped, mapped::Problems};
use biodata::{Expression, ExpressionSyntax, Text};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::Source;

/// Read one canonical allele's exact GRCh38 gnomAD v4 key.
/// gnomAD v2 uses GRCh37 and is deliberately not a fallback.
pub fn population_id(leg: &Leg<'_>, id: &str) -> Result<Mapped<Option<Text>>, RequestError> {
    let raw = super::read_fields(leg, id, true, "none+@id+externalRecords.gnomAD_4")?;
    raw.identity(Some(id))
        .map_err(|_| biomcp_fetch::bad_reply(Source::AlleleRegistry, None))?;
    let mut fields = Problems {
        source: Source::AlleleRegistry,
        record: Text::new(id).ok(),
        list: Vec::new(),
    };
    let mut ids = std::collections::BTreeSet::new();
    if let Some(external) = fields.field("externalRecords", raw.external) {
        for row in fields.fields("externalRecords.gnomAD_4", external.gnomad) {
            let id = fields.field("externalRecords.gnomAD_4.id", Some(row.id));
            if let Some(expression) = fields.optional(
                "externalRecords.gnomAD_4.id",
                id.map(|id| Expression::new(ExpressionSyntax::Gnomad, id, None)),
            ) {
                ids.insert(expression.value().clone());
            }
        }
    }
    let value = if ids.len() == 1 && fields.list.is_empty() {
        ids.into_iter().next()
    } else {
        if ids.len() > 1 {
            fields.required::<()>("externalRecords.gnomAD_4.id", None);
        }
        None
    };
    Ok(Mapped {
        value,
        problems: fields.list,
        refusals: crate::Refusal::None,
    })
}
