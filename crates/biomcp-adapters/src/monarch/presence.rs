//! Observe one optional native field without replacing missing values with a default.
use biodata::Curie;
use serde::{
    Deserialize, Deserializer,
    de::{IgnoredAny, MapAccess, Visitor},
};
use std::fmt;
#[derive(Debug)]
pub(super) struct Aggregators(pub Option<Option<Box<[Curie]>>>);
impl<'de> Deserialize<'de> for Aggregators {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Fields;
        impl<'de> Visitor<'de> for Fields {
            type Value = Aggregators;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a native association field map")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "aggregator_knowledge_source" {
                        if value.is_some() {
                            return Err(serde::de::Error::duplicate_field(
                                "aggregator_knowledge_source",
                            ));
                        }
                        value = Some(map.next_value::<Option<Box<[Curie]>>>()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                Ok(Aggregators(value))
            }
        }
        deserializer.deserialize_map(Fields)
    }
}
