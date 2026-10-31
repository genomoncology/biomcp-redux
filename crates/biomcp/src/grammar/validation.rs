//! Decode each field once and retain only its grammar-owned identity on failure.
use biomcp_types::{CheckedQuery, QueryInvalid};
use serde::de::{DeserializeSeed, IntoDeserializer, MapAccess};
use serde_json::{Map, Value};

pub(super) fn decode<T: CheckedQuery>(value: Value) -> Result<T, QueryInvalid> {
    let Value::Object(fields) = value else {
        return Err(QueryInvalid::default());
    };
    let message = T::USAGE_MESSAGE;
    T::from_fields(serde::de::value::MapAccessDeserializer::new(Fields {
        fields: fields.into_iter(),
        pending: None,
        message,
    })).map_err(|cause| {
        if cause.message().is_none() {
            if let Some(message) = message { cause.with_message(message) } else { cause }
        } else { cause }
    })
}
struct Fields {
    fields: serde_json::map::IntoIter,
    pending: Option<(String, Value)>,
    message: Option<biomcp_types::ErrorMessage>,
}
impl<'de> MapAccess<'de> for Fields {
    type Error = QueryInvalid;
    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let Some((key, value)) = self.fields.next() else {
            return Ok(None);
        };
        self.pending = Some((key.clone(), value));
        seed.deserialize(key.into_deserializer()).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let (key, value) = self.pending.take().ok_or_else(QueryInvalid::default)?;
        seed.deserialize(value)
            .map_err(|_| {
                let cause = QueryInvalid::new(&[&key]);
                if key == "p_value" && self.message == Some(biomcp_types::ErrorMessage::GwasSearchArguments) {
                    cause.with_message(biomcp_types::ErrorMessage::GwasPositiveThreshold)
                } else { cause }
            })
    }
}

pub(super) fn argument_names(
    verb: &str,
    entity: &str,
    matches: &clap::ArgMatches,
    cause: &QueryInvalid,
) -> Option<String> {
    let id = super::QUERIES
        .iter()
        .find(|(v, e, _)| *v == verb && *e == entity)?
        .2;
    let schema: Value = serde_json::from_str(crate::schema::find(id)?).ok()?;
    let properties = schema.get("properties")?.as_object()?;
    let names: Vec<_> = cause
        .arguments()
        .iter()
        .filter_map(|name| {
            if name.as_ref() == "query"
                && matches
                    .try_get_one::<String>("query_option")
                    .ok()
                    .flatten()
                    .is_some()
            {
                Some("--query".into())
            } else {
                identity(name, properties)
            }
        })
        .collect();
    (!names.is_empty()).then(|| names.join(", "))
}

fn identity(name: &str, properties: &Map<String, Value>) -> Option<String> {
    let field = properties.get(name)?;
    Some(if field.get("x-positional").is_some() {
        name.to_owned()
    } else {
        format!("--{}", name.replace('_', "-"))
    })
}
