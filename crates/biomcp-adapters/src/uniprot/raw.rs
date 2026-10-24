use crate::raw::Field;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Proteins {
    pub results: Vec<Field<Protein>>,
}
impl Reply for Proteins {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Protein {
    pub primary_accession: String,
    pub entry_type: String,
    pub organism: Organism,
    pub protein_description: Description,
    pub sequence: Sequence,
    pub comments: Option<Field<Vec<Field<Comment>>>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Organism {
    pub taxon_id: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Description {
    pub recommended_name: Name,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Name {
    pub full_name: Value,
}
#[derive(Debug, Deserialize)]
pub(super) struct Value {
    pub value: String,
}
#[derive(Debug, Deserialize)]
pub(super) struct Sequence {
    pub length: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Comment {
    pub comment_type: String,
    pub texts: Option<Field<Vec<Field<Value>>>>,
    pub subcellular_locations: Option<Field<Vec<Field<Location>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Location {
    pub location: Value,
}
