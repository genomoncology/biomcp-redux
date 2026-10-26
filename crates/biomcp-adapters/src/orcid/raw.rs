use biodata::{AuthorCard, AuthorHeader, AuthorId, OrcidId, OrcidWorkGroup, Text};
use serde::{Deserialize, Deserializer};
use serde_json::{Value, json};
#[derive(Deserialize)]
pub(super) struct Person {
    pub path: String,
    pub name: Option<Name>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct Name {
    pub path: String,
    visibility: Option<String>,
    credit_name: Option<Wrapped>,
    given_names: Option<Wrapped>,
    family_name: Option<Wrapped>,
}
#[derive(Deserialize)]
struct Wrapped { value: String }
impl Person {
    pub fn card(self, id: &OrcidId) -> Result<Option<AuthorCard>, ()> {
        let Some(name) = self.name.filter(|n| n.visibility.as_deref() == Some("public")) else { return Ok(None) };
        let display = name.credit_name.map(|n| n.value).filter(|n| !n.trim().is_empty()).unwrap_or_else(|| {
            [name.given_names.map(|n| n.value), name.family_name.map(|n| n.value)].into_iter().flatten().collect::<Vec<_>>().join(" ")
        });
        if display.trim().is_empty() { return Ok(None); }
        Ok(Some(AuthorCard {
            header: AuthorHeader { id: AuthorId::Orcid(id.clone()), name: display.parse::<Text>().map_err(|_| ())?, affiliations: None, counts: None },
            url: Some(format!("https://orcid.org/{id}").parse().map_err(|_| ())?),
        }))
    }
}
pub(super) struct Works {
    pub path: String,
    pub paths: Vec<(String, u64)>,
    pub groups: Box<[OrcidWorkGroup]>,
}
impl<'de> Deserialize<'de> for Works {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        works(value).map_err(serde::de::Error::custom)
    }
}
fn required<'a>(v: &'a Value, key: &str) -> Result<&'a Value, &'static str> {
    v.get(key).ok_or("missing native ORCID field")
}
fn wrapped(v: &Value) -> Value { v.get("value").cloned().unwrap_or(Value::Null) }
fn text(v: &Value, key: &str) -> Value { v.get(key).map(wrapped).unwrap_or(Value::Null) }
fn ids(v: &Value) -> Result<Value, &'static str> {
    let entries = required(v, "external-id")?.as_array().ok_or("invalid ORCID external IDs")?;
    let values = entries.iter().map(|id| Ok(json!({
        "kind": required(id, "external-id-type")?,
        "value": required(id, "external-id-value")?,
        "relationship": required(id, "external-id-relationship")?,
        "url": text(id, "external-id-url")
    }))).collect::<Result<Vec<_>, &'static str>>()?;
    Ok(json!(values))
}
fn source(v: &Value) -> Value {
    let mut out = serde_json::Map::new();
    for key in ["source-orcid", "source-client-id", "source-name", "assertion-origin-orcid", "assertion-origin-client-id", "assertion-origin-name"] {
        let native = v.get(key).unwrap_or(&Value::Null);
        let value = if key.ends_with("name") { wrapped(native) } else { native.get("path").cloned().unwrap_or(Value::Null) };
        if !value.is_null() { out.insert(key.replace('-', "_"), value); }
    }
    Value::Object(out)
}
fn date(v: &Value) -> Result<Value, &'static str> {
    if v.is_null() { return Ok(Value::Null); }
    let mut out = serde_json::Map::new();
    for key in ["year", "month", "day"] {
        let val = text(v, key);
        if !val.is_null() {
            let num = val.as_str().ok_or("invalid native date")?.parse::<u32>().map_err(|_| "invalid native date")?;
            out.insert(key.into(), json!(num));
        }
    }
    Ok(Value::Object(out))
}
fn assertion(v: &Value) -> Result<Value, &'static str> {
    let title = v.get("title").unwrap_or(&Value::Null);
    Ok(json!({
        "put_code": required(v, "put-code")?,
        "title": text(title, "title"), "subtitle": text(title, "subtitle"),
        "translated_title": text(title, "translated-title"),
        "work_type": required(v, "type")?,
        "publication_date": date(v.get("publication-date").unwrap_or(&Value::Null))?,
        "journal_title": text(v, "journal-title"), "url": text(v, "url"),
        "external_ids": ids(required(v, "external-ids")?)?,
        "source": source(required(v, "source")?), "visibility": "public",
        "display_index": v.get("display-index")
    }))
}
fn works(v: Value) -> Result<Works, &'static str> {
    let path = required(&v, "path")?.as_str().ok_or("invalid ORCID path")?.to_owned();
    let mut paths = Vec::new();
    let mut groups = Vec::new();
    for group in required(&v, "group")?.as_array().ok_or("invalid ORCID groups")? {
        let mut assertions = Vec::new();
        for row in required(group, "work-summary")?.as_array().ok_or("invalid ORCID assertions")? {
            let code = required(row, "put-code")?.as_u64().ok_or("invalid ORCID put-code")?;
            paths.push((required(row, "path")?.as_str().ok_or("invalid ORCID path")?.to_owned(), code));
            if row.get("visibility").and_then(Value::as_str) == Some("public") { assertions.push(assertion(row)?); }
        }
        if !assertions.is_empty() {
            groups.push(serde_json::from_value(json!({"external_ids": ids(required(group, "external-ids")?)?, "assertions": assertions})).map_err(|_| "invalid ORCID group")?);
        }
    }
    Ok(Works { path, paths, groups: groups.into() })
}
