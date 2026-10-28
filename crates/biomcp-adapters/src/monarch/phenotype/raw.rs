use biodata::{FiniteNumber, HpId, PhenotypeAssociationCount};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Node {
    pub id: HpId,
    pub name: Box<str>,
    pub category: Box<str>,
    pub namespace: Option<Box<str>>,
    pub description: Option<Box<str>>,
    pub score: Option<FiniteNumber>,
    pub matched_field: Option<Box<str>>,
    pub match_type: Option<Box<str>>,
    pub deprecated: Option<bool>,
    pub has_descendant_count: Option<u64>,
    #[serde(default, deserialize_with = "crate::strict_input::deserialize")]
    pub association_counts: Option<Box<[PhenotypeAssociationCount]>>,
    pub synonym: Option<Box<[Box<str>]>>,
    pub exact_synonym: Option<Box<[Box<str>]>>,
    pub broad_synonym: Option<Box<[Box<str>]>>,
    pub narrow_synonym: Option<Box<[Box<str>]>>,
    pub related_synonym: Option<Box<[Box<str>]>>,
    pub xref: Option<Box<[biodata::Curie]>>,
    pub node_hierarchy: Option<Hierarchy>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Hierarchy {
    pub super_classes: Option<Box<[Link]>>,
    pub sub_classes: Option<Box<[Link]>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Link {
    pub id: biodata::Curie,
    pub name: Box<str>,
}
impl Reply for Node {
    fn check(&self) -> Result<(), BadReply> {
        if self.category.as_ref() != "biolink:PhenotypicFeature"
            || self.namespace.as_deref().is_some_and(|v| v != "HP")
        {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Page {
    pub limit: u32,
    pub offset: u32,
    pub total: u64,
    pub items: Vec<Node>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.items.len() > self.limit as usize
            || (!self.items.is_empty()
                && u64::from(self.offset) + self.items.len() as u64 > self.total)
        {
            return Err(BadReply);
        }
        for node in &self.items {
            node.check()?;
        }
        Ok(())
    }
}
