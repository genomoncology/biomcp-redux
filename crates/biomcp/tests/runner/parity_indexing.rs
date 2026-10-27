//! Project native released PubMed indexing without adding expected facts.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Indexing {
    pub(super) article_indexing: String,
    field: Field,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Field {
    AffiliationsTotal,
    AffiliationsItems,
    Orcids,
    Mesh,
}
impl Indexing {
    pub(super) fn select(&self, value: &Value) -> Result<Value, String> {
        let raw = super::super::assertions::select(value, &self.article_indexing)?;
        match self.field {
            Field::Mesh => mesh(&raw),
            Field::AffiliationsItems | Field::AffiliationsTotal => {
                let mut texts = Vec::new();
                for author in array(&raw, "authors")? {
                    for affiliation in array(author, "affiliations")? {
                        texts.push(string(affiliation, "text")?);
                    }
                }
                Ok(match self.field {
                    Field::AffiliationsTotal => json!(texts.len()),
                    // Current indexing retains the first ten in native byline order.
                    _ => json!(texts.into_iter().take(10).collect::<Vec<_>>()),
                })
            }
            Field::Orcids => {
                let mut ids = Vec::new();
                for author in array(&raw, "authors")? {
                    if author.get("orcid").is_some() {
                        let id = string(author, "orcid")?;
                        if !ids.contains(&id) {
                            ids.push(id);
                        }
                    }
                }
                Ok(json!(ids))
            }
        }
    }
}
fn array<'a>(value: &'a Value, key: &str) -> Result<&'a [Value], String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("released indexing {key} requires an array"))
}
fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("released indexing {key} requires a string"))
}
fn major(value: &Value) -> Result<bool, String> {
    value
        .get("major_topic")
        .and_then(Value::as_bool)
        .ok_or_else(|| "released indexing major_topic requires a boolean".into())
}
fn mesh(raw: &Value) -> Result<Value, String> {
    array(raw, "mesh_headings")?
        .iter()
        .map(|heading| {
            let descriptor = heading
                .get("descriptor")
                .ok_or("missing released descriptor")?;
            let qualifiers = array(heading, "qualifiers")?
                .iter()
                .map(|term| Ok(json!({"name":string(term, "text")?,"major":major(term)?})))
                .collect::<Result<Vec<_>, String>>()?;
            Ok(json!({"descriptor":string(descriptor, "text")?,
                "major":major(descriptor)?,"qualifiers":qualifiers}))
        })
        .collect::<Result<Vec<_>, String>>()
        .map(Value::Array)
}
