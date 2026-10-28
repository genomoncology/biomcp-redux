//! Keep the user's JSON object field order when persisting credential changes.
use serde::{Deserializer, Serialize, Serializer, de, ser::SerializeMap};
use serde_json::{Value, value::RawValue};

pub(super) struct Fields<'a>(pub &'a Value, pub Option<&'a str>);

struct Object(Vec<(String, Box<RawValue>)>);
impl<'de> de::Visitor<'de> for Object {
    type Value = Self;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a configuration object")
    }
    fn visit_map<M: de::MapAccess<'de>>(mut self, mut map: M) -> Result<Self, M::Error> {
        while let Some(field) = map.next_entry()? {
            self.0.push(field);
        }
        Ok(self)
    }
}

impl Serialize for Fields<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Some(fields) = self.0.as_object() else {
            return self.0.serialize(serializer);
        };
        let original = self
            .1
            .filter(|text| text.trim_start().starts_with('{'))
            .map_or_else(
                || Ok(Object(Vec::new())),
                |text| serde_json::Deserializer::from_str(text).deserialize_map(Object(Vec::new())),
            )
            .map_err(serde::ser::Error::custom)?;
        let mut map = serializer.serialize_map(Some(fields.len()))?;
        for (key, raw) in &original.0 {
            if let Some(value) = fields.get(key) {
                map.serialize_entry(key, &Fields(value, Some(raw.get())))?;
            }
        }
        for (key, value) in fields {
            if !original.0.iter().any(|(old, _)| old == key) {
                map.serialize_entry(key, &Fields(value, None))?;
            }
        }
        map.end()
    }
}
