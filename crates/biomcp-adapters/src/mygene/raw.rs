use crate::raw::{Field, OneOrMany};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(super) struct Genes(pub Vec<Hit>);
impl Reply for Genes {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.iter().any(|hit| hit.query.is_empty()) {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Hit {
    pub query: String,
    pub notfound: Option<bool>,
    pub entrezgene: Option<Field<String>>,
    #[serde(rename = "HGNC")]
    pub hgnc: Option<Field<String>>,
    pub summary: Option<Field<String>>,
    // MyGene returns one position object or several source-stated positions.
    pub genomic_pos: Option<Field<OneOrMany<Pos>>>,
    pub genomic_pos_hg19: Option<Field<OneOrMany<Pos>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Pos {
    pub chr: Option<Field<String>>,
    pub start: Option<Field<u32>>,
    pub end: Option<Field<u32>>,
    pub strand: Option<Field<i8>>,
}
