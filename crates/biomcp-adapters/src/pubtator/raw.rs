use crate::raw::Field;
use biodata::Pmid;
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Export {
    #[serde(rename = "PubTator3")]
    pub documents: Vec<Document>,
}
#[derive(Deserialize)]
pub(super) struct Document {
    pub id: Pmid,
    pub passages: Vec<Passage>,
}
#[derive(Deserialize)]
pub(super) struct Passage {
    #[serde(default)]
    pub annotations: Vec<Annotation>,
}
#[derive(Deserialize)]
pub(super) struct Annotation {
    pub infons: Infons,
    #[serde(default)]
    pub text: Option<String>,
}
#[derive(Deserialize)]
pub(super) struct Infons {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub identifier: Option<Field<String>>,
    #[serde(default)]
    pub name: Option<Field<String>>,
    #[serde(default)]
    pub hgvs: Option<Field<String>>,
    #[serde(default)]
    pub rsid: Option<Field<String>>,
    #[serde(default)]
    pub gene_id: Option<Field<u32>>,
}
impl biomcp_fetch::Reply for Export {
    fn check(&self) -> Result<(), biomcp_fetch::BadReply> {
        let mut ids = std::collections::BTreeSet::new();
        if self.documents.iter().any(|d| !ids.insert(d.id)) {
            return Err(biomcp_fetch::BadReply);
        }
        Ok(())
    }
}
