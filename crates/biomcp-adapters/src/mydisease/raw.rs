use crate::raw::OneOrMany;
use biodata::{Curie, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub(super) struct Document {
    #[serde(rename = "_id")]
    pub id: Curie,
    #[serde(rename = "_version")]
    pub version: Option<u64>,
    pub mondo: Option<Component>,
    pub disease_ontology: Option<Component>,
}
impl Reply for Document {
    fn check(&self) -> Result<(), BadReply> {
        let mondo = self.mondo.as_ref().is_some_and(|c| c.id == self.id);
        let doid = self.disease_ontology.as_ref().is_some_and(|c| c.id == self.id);
        if mondo || doid { Ok(()) } else { Err(BadReply) }
    }
}
impl Document {
    pub fn matches(&self, id: &Curie, fields: &str) -> bool {
        let mondo = fields.split(',').any(|f| f == "mondo").then_some(self.mondo.as_ref()).flatten();
        let doid = fields.split(',').any(|f| f == "disease_ontology").then_some(self.disease_ontology.as_ref()).flatten();
        let primary = mondo.is_some_and(|c| c.id == self.id) || doid.is_some_and(|c| c.id == self.id);
        primary && (self.id == *id || mondo.is_some_and(|c| {
            c.xrefs.get("doid").is_some_and(|values| match values {
                OneOrMany::One(value) => value.as_str() == id.as_str(),
                OneOrMany::Many(values) => values.iter().any(|v| v.as_str() == id.as_str()),
            })
        }))
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Component {
    #[serde(alias = "mondo", alias = "doid")]
    pub id: Curie,
    #[serde(alias = "name")]
    pub label: Text,
    #[serde(alias = "def")]
    pub definition: Option<Text>,
    #[serde(default, alias = "synonyms")]
    pub synonym: BTreeMap<String, OneOrMany<Text>>,
    #[serde(default)]
    pub xrefs: BTreeMap<String, OneOrMany<Text>>,
    #[serde(default)]
    pub parents: Box<[Curie]>,
    pub is_obsolete: Option<bool>,
    pub consider: Option<Box<[Curie]>>,
    pub replaced_by: Option<Box<[Curie]>>,
}
