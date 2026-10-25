use biomcp_fetch::{BadReply, KeyKind, Keys, Row};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Deserialize)]
pub(super) struct Meta {
    pub timestamp: biodata::Text,
}
#[derive(Debug)]
pub(super) struct Medicine {
    pub id: biodata::DrugId,
    pub name: biodata::Text,
    pub source: biodata::Text,
    pub record: biodata::Text,
    pub meta: Meta,
    pub fields: BTreeMap<String, String>,
}
#[derive(Deserialize)]
struct Fields {
    #[serde(rename = "_bundle_meta")]
    meta: Meta,
    #[serde(flatten)]
    fields: BTreeMap<String, String>,
}
impl<'de> Deserialize<'de> for Medicine {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(Fields::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("Invalid EMA medicine record"))
    }
}
impl TryFrom<Fields> for Medicine {
    type Error = BadReply;
    fn try_from(value: Fields) -> Result<Self, Self::Error> {
        let field = |name| value.fields.get(name).ok_or(BadReply);
        let id = field("ema_product_number")?.parse().map_err(|_| BadReply)?;
        let name = biodata::Text::new(field("name_of_medicine")?).map_err(|_| BadReply)?;
        field("category")?;
        field("medicine_status")?;
        for (key, text) in &value.fields {
            if key.ends_with("_date") && !text.is_empty() {
                date(text)?;
            }
        }
        let source = biodata::Text::new("ema").map_err(|_| BadReply)?;
        let record = biodata::Text::new(field("ema_product_number")?).map_err(|_| BadReply)?;
        Ok(Self {
            id,
            name,
            source,
            record,
            meta: value.meta,
            fields: value.fields,
        })
    }
}
pub(super) fn date(text: &str) -> Result<biodata::PartialDate, BadReply> {
    let parts: Vec<_> = text.split('/').collect();
    if parts.len() != 3 {
        return Err(BadReply);
    }
    format!("{}-{}-{}", parts[2], parts[1], parts[0])
        .parse()
        .map_err(|_| BadReply)
}
impl Row for Medicine {
    fn check_document(document: &serde_json::Value) -> Result<(), BadReply> {
        let rows = document
            .get("data")
            .and_then(serde_json::Value::as_array)
            .ok_or(BadReply)?;
        let count = document
            .pointer("/meta/total_records")
            .and_then(serde_json::Value::as_u64)
            .ok_or(BadReply)?;
        let timestamp = document
            .pointer("/meta/timestamp")
            .and_then(serde_json::Value::as_str)
            .ok_or(BadReply)?;
        // Keep the complete publisher timestamp; never substitute retrieval time.
        if count != rows.len() as u64 || timestamp.is_empty() {
            return Err(BadReply);
        }
        let mut ids = BTreeSet::new();
        for row in rows {
            let id = row
                .get("ema_product_number")
                .and_then(serde_json::Value::as_str)
                .ok_or(BadReply)?;
            if !ids.insert(id) {
                return Err(BadReply);
            }
        }
        Ok(())
    }
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.id.to_string().into())],
            words: vec![self.name.to_string().into()],
        }
    }
}
impl Medicine {
    pub(super) fn text(&self, field: &str) -> Option<biodata::Text> {
        self.fields
            .get(field)
            .and_then(|v| biodata::Text::new(v).ok())
    }
}
