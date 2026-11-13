use crate::native_text::NativeText;
use crate::raw::{Field, OneOrMany};
use biodata::Curie;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub(super) struct Document {
    #[serde(rename = "_id")]
    pub id: Curie,
    #[serde(rename = "_version")]
    pub version: Option<crate::raw::Field<u64>>,
    pub mondo: Option<Field<Component>>,
    pub disease_ontology: Option<Field<Component>>,
}
impl Reply for Document {
    fn check(&self) -> Result<(), BadReply> {
        let mondo = self
            .mondo
            .as_ref()
            .and_then(Field::as_ref)
            .is_some_and(|c| c.id == self.id);
        let doid = self
            .disease_ontology
            .as_ref()
            .and_then(Field::as_ref)
            .is_some_and(|c| c.id == self.id);
        if mondo || doid { Ok(()) } else { Err(BadReply) }
    }
}
impl Document {
    pub(super) fn matches(&self, id: &Curie, fields: &str) -> bool {
        let mondo = fields
            .split(',')
            .any(|f| f == "mondo")
            .then_some(self.mondo.as_ref().and_then(Field::as_ref))
            .flatten();
        let doid = fields
            .split(',')
            .any(|f| f == "disease_ontology")
            .then_some(self.disease_ontology.as_ref().and_then(Field::as_ref))
            .flatten();
        let primary =
            mondo.is_some_and(|c| c.id == self.id) || doid.is_some_and(|c| c.id == self.id);
        primary
            && (self.id == *id
                || (id.to_string().starts_with("DOID:")
                    && mondo.is_some_and(|c| {
                        c.xrefs
                            .as_ref()
                            .and_then(|xrefs| match xrefs { Field::Value(v) => v.get("doid"), Field::Invalid(_) => None })
                            .is_some_and(|values| match values {
                                OneOrMany::One(value) => {
                                    matches!(value, Field::Value(v) if v.as_str().parse::<Curie>().ok().as_ref() == Some(id))
                                }
                                OneOrMany::Many(values) => values
                                    .iter()
                                    .any(|v| matches!(v, Field::Value(v) if v.as_str().parse::<Curie>().ok().as_ref() == Some(id))),
                            })
                    })))
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Component {
    #[serde(alias = "mondo", alias = "doid")]
    pub id: Curie,
    #[serde(alias = "name")]
    pub label: Option<Field<NativeText>>,
    #[serde(alias = "def")]
    pub definition: Option<Field<NativeText>>,
    #[serde(alias = "synonyms")]
    pub synonym: Option<Field<BTreeMap<String, OneOrMany<Field<NativeText>>>>>,
    pub xrefs: Option<Field<BTreeMap<String, OneOrMany<Field<String>>>>>,
    pub parents: Option<Field<Box<[Field<Curie>]>>>,
    pub is_obsolete: Option<crate::raw::Field<bool>>,
    pub consider: Option<Field<Box<[Field<Curie>]>>>,
    pub replaced_by: Option<Field<Box<[Field<Curie>]>>>,
}

/// Batch reply for direct-parent labels; each hit names the query it answers.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(super) struct LabelHits(pub Vec<LabelHit>);
impl Reply for LabelHits {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.iter().any(|hit| hit.query.is_empty()) {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct LabelHit {
    pub query: String,
    pub mondo: Option<Field<LabelComponent>>,
    pub disease_ontology: Option<Field<LabelComponent>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct LabelComponent {
    #[serde(alias = "mondo", alias = "doid")]
    pub id: Option<Field<Curie>>,
    #[serde(alias = "name")]
    pub label: Option<Field<NativeText>>,
}
