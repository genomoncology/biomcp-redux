//! Closed projections preserve released values while adapting identifier shape.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum Projection {
    Pointer(String),
    Object(Object),
    Length(Length),
    ReleasedSource(ReleasedSource),
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Object {
    object: BTreeMap<String, Member>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Length {
    array_length: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Member {
    Pointer(Pointer),
    PathwaySource(PathwaySource),
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Pointer {
    pointer: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PathwaySource {
    pathway_source: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReleasedSource {
    released_source: String,
}
impl Member {
    fn pointer(&self) -> &str {
        match self {
            Self::Pointer(member) => &member.pointer,
            Self::PathwaySource(member) => &member.pathway_source,
        }
    }
}
impl Projection {
    pub(super) fn pointers(&self) -> Vec<&str> {
        match self {
            Self::Pointer(pointer) => vec![pointer],
            Self::Length(length) => vec![&length.array_length],
            Self::ReleasedSource(source) => vec![&source.released_source],
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
                match raw.as_str() {
                    Some("Reactome") => Ok(json!("reactome")),
                    Some("Semantic Scholar") => Ok(json!("semantic-scholar")),
                    Some("UniProt") => Ok(json!("uniprot")),
                    Some("ORCID") => Ok(json!("orcid")),
                    _ => Err("unknown released provider spelling".into()),
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
