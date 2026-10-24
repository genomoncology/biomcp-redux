use super::{gene::{QueryInvalid, present}};
use crate::{Pmid, Text};
use serde::{Deserialize, Serialize};
/// Page the citing or referenced PubMed papers of one checked PMID.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct ArticleEdges(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// PMID whose graph edges to read.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pmid: Pmid,
    /// Maximum papers on one page.
    #[serde(default = "limit")]
    #[cfg_attr(feature="schema", schemars(range(min=1,max=50)))]
    limit: u32,
    /// Number of pool items to skip.
    #[serde(default)]
    offset: u32,
    /// Fixed snapshot from an earlier page.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    pool: Option<Text>,
}
fn limit() -> u32 {10}
impl TryFrom<Fields> for ArticleEdges {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&v.limit) { return Err(QueryInvalid); }
        Ok(Self(v))
    }
}
impl ArticleEdges {
    /// First page of citing or referenced papers.
    pub fn new(pmid: Pmid) -> Self {Self(Fields {pmid, limit:limit(), offset:0, pool:None})}
    /// Parent paper.
    pub fn pmid(&self) -> Pmid {self.0.pmid}
    /// Page length.
    pub fn limit(&self) -> u32 {self.0.limit}
    /// Page offset.
    pub fn offset(&self) -> u32 {self.0.offset}
    /// Snapshot selector.
    pub fn pool(&self) -> Option<&Text> {self.0.pool.as_ref()}
    /// Select a page without changing identity.
    pub fn page(&self, offset: u32, pool: Option<Text>) -> Self {
        let mut v=self.clone(); v.0.offset=offset; v.0.pool=pool; v
    }
    /// Canonical selectors for one edge direction.
    pub fn identity(&self) -> Self {Self::new(self.pmid())}
}
