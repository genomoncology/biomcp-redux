use crate::raw::OneOrMany;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Page {
    pub total: u64,
    pub hits: Vec<Hit>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        for hit in &self.hits { hit.check()?; }
        if self.hits.len() as u64 > self.total { return Err(BadReply); }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Hit {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u64>,
    pub drugbank: Option<OneOrMany<Component>>,
    pub chembl: Option<OneOrMany<Component>>,
    pub drugcentral: Option<OneOrMany<Component>>,
    pub gtopdb: Option<OneOrMany<Component>>,
    pub unii: Option<OneOrMany<Component>>,
    pub ndc: Option<OneOrMany<Component>>,
    pub openfda: Option<OneOrMany<Component>>,
}
impl Reply for Hit {
    fn check(&self) -> Result<(), BadReply> {
        if self.id.is_empty() { Err(BadReply) } else { Ok(()) }
    }
}
#[derive(Debug, Default, Deserialize)]
pub(super) struct Component {
    pub id: Option<String>,
    pub name: Option<String>,
    pub molecule_chembl_id: Option<String>,
    pub pref_name: Option<String>,
    pub molecule_type: Option<String>,
    pub ligand_id: Option<String>,
    pub r#type: Option<String>,
    pub unii: Option<String>,
    pub display_name: Option<String>,
    pub ingredient_type: Option<String>,
    pub product_id: Option<String>,
    pub productndc: Option<String>,
    pub producttypename: Option<String>,
    pub nonproprietaryname: Option<String>,
    pub proprietaryname: Option<String>,
    pub synonyms: Option<OneOrMany<String>>,
    pub molecule_synonyms: Option<Vec<Synonym>>,
    pub molecule_hierarchy: Option<Hierarchy>,
    pub chembl_id: Option<String>,
    pub rxcui: Option<String>,
    pub ncit: Option<String>,
    pub applicationnumber: Option<String>,
    pub generic_name: Option<OneOrMany<String>>,
    pub brand_name: Option<OneOrMany<String>>,
    pub spl_set_id: Option<OneOrMany<String>>,
    pub application_number: Option<OneOrMany<String>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Synonym { pub molecule_synonym: Option<String> }
#[derive(Debug, Deserialize)]
pub(super) struct Hierarchy {
    pub active_chembl_id: Option<String>,
    pub parent_chembl_id: Option<String>,
}
