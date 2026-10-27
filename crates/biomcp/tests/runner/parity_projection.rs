//! Closed projections preserve released values while adapting identifier shape.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum Projection {
    Pointer(String),
    Object(Object),
    Length(Length),
    ReleasedSource(ReleasedSource),
    AuthorIds(AuthorIds),
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Object {
    object: BTreeMap<String, Member>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Length {
    array_length: String,
}
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Member {
    Pointer(Pointer),
    PathwaySource(PathwaySource),
    ReleasedSource(ReleasedSource),
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Pointer {
    pointer: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PathwaySource {
    pathway_source: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReleasedSource {
    released_source: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AuthorIds {
    author_ids: String,
}
impl Member {
    fn pointer(&self) -> &str {
        match self {
            Self::Pointer(member) => &member.pointer,
            Self::PathwaySource(member) => &member.pathway_source,
            Self::ReleasedSource(member) => &member.released_source,
        }
    }
}
impl Projection {
    pub(super) fn pointers(&self) -> Vec<&str> {
        match self {
            Self::Pointer(pointer) => vec![pointer],
            Self::Length(length) => vec![&length.array_length],
            Self::ReleasedSource(source) => vec![&source.released_source],
            Self::AuthorIds(ids) => vec![&ids.author_ids],
            Self::Object(object) => object.object.values().map(Member::pointer).collect(),
        }
    }
    pub(super) fn select(&self, value: &Value) -> Result<Value, String> {
        match self {
            Self::Pointer(pointer) => super::super::assertions::select(value, pointer),
            Self::Length(length) => {
                let selected = super::super::assertions::select(value, &length.array_length)?;
                Ok(json!(
                    selected
                        .as_array()
                        .ok_or("array length projection requires an actual array")?
                        .len()
                ))
            }
            Self::Object(object) => project_object(object, value),
            Self::ReleasedSource(source) => {
                let raw = super::super::assertions::select(value, &source.released_source)?;
                released_source(&raw)
            }
            Self::AuthorIds(ids) => {
                let raw = super::super::assertions::select(value, &ids.author_ids)?;
                match raw {
                    Value::Array(values) => values
                        .iter()
                        .map(author_id)
                        .collect::<Result<Vec<_>, _>>()
                        .map(Value::Array),
                    raw => author_id(&raw),
                }
            }
        }
    }
}
fn project_object(object: &Object, value: &Value) -> Result<Value, String> {
    if object.object.is_empty() {
        return Err("object projection requires explicit members".into());
    }
    let mut result = serde_json::Map::new();
    for (name, member) in &object.object {
        let raw = super::super::assertions::select(value, member.pointer())?;
        let projected = match member {
            Member::Pointer(_) => json!(
                raw.as_str()
                    .ok_or("identifier object member requires a string")?
            ),
            Member::ReleasedSource(_) => {
                if name != "source" {
                    return Err("released provider enum belongs to the source member".into());
                }
                released_source(&raw)?
            }
            Member::PathwaySource(_) => {
                if name != "source" {
                    return Err("pathway source enum belongs to the source member".into());
                }
                match raw.as_str() {
                    Some("Reactome") => json!("reactome"),
                    _ => return Err("unknown released pathway source spelling".into()),
                }
            }
        };
        if projected.is_null() {
            return Err("object projection member is null".into());
        }
        result.insert(name.clone(), projected);
    }
    Ok(Value::Object(result))
}

fn released_source(raw: &Value) -> Result<Value, String> {
    match raw.as_str().ok_or("released provider requires a string")? {
        "Reactome" => Ok(json!("reactome")),
        "Semantic Scholar" => Ok(json!("semantic-scholar")),
        "UniProt" => Ok(json!("uniprot")),
        "ORCID" => Ok(json!("orcid")),
        "ClinGen" => Ok(json!("clingen")),
        "clingen_erepo" => Ok(json!("erepo")),
        _ => Err("unknown released provider spelling".into()),
    }
}
fn author_id(raw: &Value) -> Result<Value, String> {
    let suffix = raw
        .as_str()
        .and_then(|id| id.strip_prefix("semanticscholar:"))
        .filter(|suffix| !suffix.is_empty() && suffix.bytes().all(|b| b.is_ascii_digit()))
        .ok_or("released author ID requires exact semanticscholar: numeric namespace")?;
    Ok(json!(format!("semantic-scholar:{suffix}")))
}
