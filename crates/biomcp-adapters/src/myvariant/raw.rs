use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Page {
    pub total: u64,
    pub hits: Vec<Hit>,
}
#[derive(Deserialize)]
pub(super) struct Hit {
    #[serde(rename = "_id")]
    pub id: biodata::Text,
    pub dbsnp: Option<Dbsnp>,
    pub clinvar: Option<Clinvar>,
    pub cosmic: Option<Cosmic>,
}
#[derive(Deserialize)]
pub(super) struct Dbsnp {
    pub rsid: Option<OneOrMany>,
    pub gene: Option<Genes>,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Genes {
    One(Gene),
    Many(Vec<Gene>),
}
#[derive(Deserialize)]
pub(super) struct Gene {
    pub symbol: Option<biodata::Text>,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum OneOrMany {
    One(biodata::Text),
    Many(Vec<biodata::Text>),
}
impl OneOrMany {
    pub(super) fn values(self) -> Vec<biodata::Text> {
        match self {
            Self::One(v) => vec![v],
            Self::Many(v) => v,
        }
    }
}
#[derive(Deserialize)]
pub(super) struct Clinvar {
    pub variant_id: u64,
}
#[derive(Deserialize)]
pub(super) struct Cosmic {
    pub cosmic_id: Option<OneOrMany>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.total > 10_000 || self.hits.len() > 1000 || self.hits.len() as u64 > self.total {
            return Err(BadReply);
        }
        if self.hits.iter().any(|h| {
            !h.id.as_str().starts_with("chr")
                || h.id.as_str().parse::<biodata::VariantInput>().is_err()
        }) {
            return Err(BadReply);
        }
        Ok(())
    }
}
