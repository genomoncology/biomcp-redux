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
                || super::xref(id.as_str()).is_some_and(|(field, spelling)| {
                    mondo.is_some_and(|c| c.states(field, &spelling))
                }))
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
    // MyDisease writes a one-item list as a bare value.
    pub parents: Option<Field<OneOrMany<Field<Curie>>>>,
    pub is_obsolete: Option<crate::raw::Field<bool>>,
    pub consider: Option<Field<OneOrMany<Field<Curie>>>>,
    pub replaced_by: Option<Field<OneOrMany<Field<Curie>>>>,
}
impl Component {
    /// Whether this component's cross-reference field states the identifier, with or without its namespace.
    fn states(&self, field: &str, spelling: &str) -> bool {
        let Some(Field::Value(xrefs)) = &self.xrefs else {
            return false;
        };
        let values: &[Field<String>] = match xrefs.get(field) {
            Some(OneOrMany::One(value)) => std::slice::from_ref(value),
            Some(OneOrMany::Many(values)) => values,
            None => return false,
        };
        values.iter().any(|value| {
            matches!(value, Field::Value(v)
                if v.eq_ignore_ascii_case(spelling) || format!("{field}:{v}").eq_ignore_ascii_case(spelling))
        })
    }
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
