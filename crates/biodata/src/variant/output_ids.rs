//! Namespace-qualified output for variant identifier collections.
use crate::VariantId;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

pub(super) fn serialize<S: Serializer>(
    ids: &[VariantId],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut output = serializer.serialize_seq(Some(ids.len()))?;
    for id in ids {
        output.serialize_element(&Qualified(id))?;
    }
    output.end()
}

pub(super) struct Qualified<'a>(&'a VariantId);

impl Serialize for Qualified<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self.0 {
            VariantId::Rs(id) => format!("rs:{}", id.get()),
            VariantId::Ca(id) => format!("clingen:{id}"),
            VariantId::Pa(id) => format!("clingen:{id}"),
            VariantId::Vcv(id) => format!("clinvar:{id}"),
            VariantId::Cosmic(id) => format!("cosmic:{id}"),
            id @ (VariantId::ClinVarVariation(_) | VariantId::Vrs(_)) => id.to_string(),
        };
        serializer.serialize_str(&value)
    }
}

pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Box<[VariantId]>, D::Error> {
    Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|value| {
            let result = if let Some(number) = value.strip_prefix("rs:") {
                format!("rs{number}").parse().map(VariantId::Rs)
            } else if let Some(id) = value.strip_prefix("clingen:") {
                id.parse()
                    .map(VariantId::Ca)
                    .or_else(|_| id.parse().map(VariantId::Pa))
            } else if let Some(id) = value.strip_prefix("cosmic:") {
                id.parse().map(VariantId::Cosmic)
            } else if let Some(id) = value.strip_prefix("clinvar:VCV") {
                format!("VCV{id}").parse().map(VariantId::Vcv)
            } else {
                value.parse()
            };
            result.map_err(serde::de::Error::custom)
        })
        .collect()
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Qualified<'_> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "QualifiedVariantId".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let pattern = concat!(
            "^(rs:[1-9][0-9]*|clingen:(CA[0-9]{6,}|PA[1-9][0-9]*)|",
            "clinvar:(VCV[0-9]{9}(\\.[1-9][0-9]*)?|[1-9][0-9]*)|",
            "cosmic:COS[VMN][1-9][0-9]*|ga4gh:VA\\.[A-Za-z0-9_-]{32})$"
        );
        schemars::json_schema!({"type":"string", "pattern":pattern})
    }
}
