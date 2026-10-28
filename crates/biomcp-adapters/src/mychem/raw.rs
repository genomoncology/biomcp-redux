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
        for hit in &self.hits {
            hit.check()?;
        }
        if self.hits.len() as u64 > self.total {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Hit {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u64>,
    pub drugbank: Option<OneOrMany<ReceivedComponent>>,
    pub chembl: Option<OneOrMany<ReceivedComponent>>,
    pub drugcentral: Option<OneOrMany<ReceivedComponent>>,
    pub gtopdb: Option<OneOrMany<ReceivedComponent>>,
    pub unii: Option<OneOrMany<ReceivedComponent>>,
    pub ndc: Option<OneOrMany<ReceivedComponent>>,
    pub openfda: Option<OneOrMany<ReceivedComponent>>,
}
impl Reply for Hit {
    fn check(&self) -> Result<(), BadReply> {
        if self.id.parse::<biodata::DrugId>().is_err() {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Default)]
pub(super) struct Component {
    pub xrefs: Option<std::collections::BTreeMap<String, OneOrMany<String>>>,
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
#[derive(Debug)]
pub(super) struct Synonym {
    pub molecule_synonym: Option<String>,
}
#[derive(Debug)]
pub(super) struct Hierarchy {
    pub active_chembl_id: Option<String>,
    pub parent_chembl_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub(super) struct ReceivedComponent {
    pub xrefs: Option<std::collections::BTreeMap<String, OneOrMany<crate::raw::Field<String>>>>,
    pub id: Option<crate::raw::Field<String>>,
    pub name: Option<crate::raw::Field<String>>,
    pub molecule_chembl_id: Option<crate::raw::Field<String>>,
    pub pref_name: Option<crate::raw::Field<String>>,
    pub molecule_type: Option<crate::raw::Field<String>>,
    pub ligand_id: Option<crate::raw::Field<String>>,
    pub r#type: Option<crate::raw::Field<String>>,
    pub unii: Option<crate::raw::Field<String>>,
    pub display_name: Option<crate::raw::Field<String>>,
    pub ingredient_type: Option<crate::raw::Field<String>>,
    pub product_id: Option<crate::raw::Field<String>>,
    pub productndc: Option<crate::raw::Field<String>>,
    pub producttypename: Option<crate::raw::Field<String>>,
    pub nonproprietaryname: Option<crate::raw::Field<String>>,
    pub proprietaryname: Option<crate::raw::Field<String>>,
    pub synonyms: Option<OneOrMany<crate::raw::Field<String>>>,
    pub molecule_synonyms: Option<Vec<ReceivedSynonym>>,
    pub molecule_hierarchy: Option<ReceivedHierarchy>,
    pub chembl_id: Option<crate::raw::Field<String>>,
    pub rxcui: Option<crate::raw::Field<String>>,
    pub ncit: Option<crate::raw::Field<String>>,
    pub applicationnumber: Option<crate::raw::Field<String>>,
    pub generic_name: Option<OneOrMany<crate::raw::Field<String>>>,
    pub brand_name: Option<OneOrMany<crate::raw::Field<String>>>,
    pub spl_set_id: Option<OneOrMany<crate::raw::Field<String>>>,
    pub application_number: Option<OneOrMany<crate::raw::Field<String>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct ReceivedSynonym {
    pub molecule_synonym: Option<crate::raw::Field<String>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct ReceivedHierarchy {
    pub active_chembl_id: Option<crate::raw::Field<String>>,
    pub parent_chembl_id: Option<crate::raw::Field<String>>,
}
