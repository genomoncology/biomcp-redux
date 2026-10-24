use super::{GeneGet, GeneSearch, ArticleSearch};
use serde::{Deserialize, Serialize};
/// One checked query. Housekeeping actions are separate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Request {
    /// Search PubMed article identities.
    SearchArticle(ArticleSearch),
    /// Get gene identities and annotations.
    GetGene(GeneGet),
    /// Search gene headers.
    SearchGene(GeneSearch),
}
