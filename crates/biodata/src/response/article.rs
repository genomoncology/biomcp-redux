mod supplement;
use crate::{ArticleHeader, ArticleSearch, Pmid};
use serde::{Deserialize, Serialize};
pub use supplement::{
    ArticleSupplementResult, CellFragment, FileKind, SheetInfo, SheetRow, SupplementContent,
    SupplementFile, Supplements,
};
/// Stable identity independent of whether citation metadata was accepted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "namespace", content = "id", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ArticleKey {
    /// PubMed identity.
    Pubmed(Pmid),
}
/// One original pool position, including a summary hole.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleItem {
    /// Stable checked pool key.
    pub key: ArticleKey,
    /// The selected PMID, retained when summary mapping fails.
    pub pmid: Pmid,
    /// Citation metadata when accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<ArticleHeader>,
}
/// A page in PubMed's original relevance order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticlePage {
    /// Original ordered items.
    pub items: Box<[ArticleItem]>,
}
impl super::ResponseBody for ArticlePage {
    const SCHEMA: &'static str = "biomcp.article_page/1";
    type Query = ArticleSearch;
}
