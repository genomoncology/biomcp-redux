use biodata::{Curie, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Search {
    pub response: SearchResponse,
}
#[derive(Deserialize)]
pub(super) struct SearchResponse {
    #[serde(rename = "numFound")]
    pub total: u64,
    pub start: u32,
    pub docs: Vec<Hit>,
}
#[derive(Deserialize)]
pub(super) struct Hit {
    pub obo_id: Curie,
    pub label: Text,
    pub ontology_name: Text,
    pub iri: Text,
    pub is_obsolete: Option<bool>,
}
impl Reply for Search {
    fn check(&self) -> Result<(), BadReply> {
        if self.response.docs.len() > 50 || self.response.docs.len() as u64 > self.response.total {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
pub(super) struct Exact {
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}
#[derive(Deserialize)]
pub(super) struct Embedded {
    pub terms: Vec<Term>,
}
#[derive(Deserialize)]
pub(super) struct Term {
    pub obo_id: Curie,
    pub label: Text,
    pub ontology_name: Text,
    pub iri: Text,
    pub description: Vec<Text>,
    pub is_obsolete: Option<bool>,
    pub term_replaced_by: Option<Vec<Text>>,
    pub annotation: Annotation,
    pub synonyms: Vec<Text>,
    pub obo_synonym: Option<Vec<Synonym>>,
    pub obo_xref: Option<Vec<Xref>>,
}
#[derive(Default, Deserialize)]
pub(super) struct Annotation {
    #[serde(default)]
    pub consider: Vec<Text>,
}
#[derive(Deserialize)]
pub(super) struct Synonym {
    pub name: Text,
    pub scope: Option<Text>,
}
#[derive(Deserialize)]
pub(super) struct Xref {
    pub database: Text,
    pub id: Text,
    pub description: Option<Box<str>>,
}
impl Reply for Exact {
    fn check(&self) -> Result<(), BadReply> {
        if self.embedded.terms.len() > 1 {
            return Err(BadReply);
        }
        Ok(())
    }
}
