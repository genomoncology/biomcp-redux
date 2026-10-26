use biodata::{Curie, HpId};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Term {
    pub id: HpId,
    pub name: Box<str>,
    pub definition: Option<Box<str>>,
    pub comment: Option<Box<str>>,
    #[serde(rename = "publicationReferences")]
    pub publications: Option<Box<[Curie]>>,
    #[serde(rename = "descendantCount")]
    pub descendants: Option<u64>,
    pub synonyms: Option<Box<[Box<str>]>>,
    pub xrefs: Option<Box<[Curie]>>,
}
impl Reply for Term {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
