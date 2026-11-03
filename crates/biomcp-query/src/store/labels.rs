//! Read old retained labels through current source and component declarations.
use biomcp_fetch::CacheError;
use biomcp_types::Source;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

pub(crate) fn decode<T: DeserializeOwned>(text: &str) -> Result<Option<T>, CacheError> {
    let mut value: Value = serde_json::from_str(text).map_err(|_| CacheError::Encoding)?;
    if relabel(&mut value).is_none() {
        return Ok(None);
    }
    serde_json::from_value(value).map(Some).map_err(|_| CacheError::Encoding)
}
fn relabel(value: &mut Value) -> Option<()> {
    match value {
        Value::Array(rows) => { for row in rows { relabel(row)?; } }
        Value::Object(row) => {
            if row.contains_key("use") && row.contains_key("terms_url") {
                let source = Source::deserialize(row.get("source")?).ok()?;
                let terms = crate::sources::terms(source)?;
                row.insert("use".into(), terms.usage.as_str().into());
                row.insert("terms".into(), terms.name.into());
                row.insert("terms_url".into(), terms.url.into());
            }
            if row.get("terms").is_some_and(|terms| terms.get("usage") == Some(&Value::from("restricted"))) {
                let source = Source::deserialize(row.get("source")?).ok()?;
                let name = row.get("name")?.as_str()?;
                let component = crate::sources::components().find(|c| c.source == source && c.name == name)?;
                row.get_mut("terms")?.as_object_mut()?.insert("usage".into(), component.terms.usage.as_str().into());
            }
            for item in row.values_mut() { relabel(item)?; }
        }
        _ => {}
    }
    Some(())
}
