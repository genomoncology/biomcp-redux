use crate::{Text, GeneInput};
use serde::{Serialize, Deserialize};
use super::gene::{QueryInvalid, present};
/// Source selectors supported by article search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub enum ArticleSource { /// PubMed's own result order.
    Pubmed }
/// Article ordering supported by this command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
pub enum ArticleSort { /// Native relevance order.
    Rank }
/// Checked search selectors and pool cursor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from="Fields")]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="Fields", extend("anyOf"=[{"required":["text"]},{"required":["gene"]},{"required":["variant"]},{"required":["disease"]},{"required":["drug"]}])))]
pub struct ArticleSearch(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Words sent to PubMed in its default fields.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text", extend("x-positional"=0)))]
    text: Option<Text>,
    /// Sources in caller order; defaults to PubMed.
    #[serde(default="sources")]
    #[cfg_attr(feature="schema", schemars(length(min=1,max=1)))]
    source: Box<[ArticleSource]>,
    /// Native relevance order.
    #[serde(default="sort")]
    sort: ArticleSort,
    /// Gene input resolved through HGNC.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="GeneInput"))]
    gene: Option<GeneInput>,
    /// Variant words.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    variant: Option<Text>,
    /// Disease words.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    disease: Option<Text>,
    /// Drug words.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    drug: Option<Text>,
    /// Maximum page length.
    #[serde(default="limit")]
    #[cfg_attr(feature="schema", schemars(range(min=1,max=50)))]
    limit: u32,
    /// Number of pool items to skip.
    #[serde(default)]
    offset: u32,
    /// Stable snapshot from an earlier response.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    pool: Option<Text>,
}
fn sources() -> Box<[ArticleSource]> { Box::new([ArticleSource::Pubmed]) }
fn sort() -> ArticleSort { ArticleSort::Rank }
fn limit() -> u32 { 10 }
impl TryFrom<Fields> for ArticleSearch {
    type Error=QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if !(1..=50).contains(&v.limit) || v.source.len()!=1 || [&v.text,&v.variant,&v.disease,&v.drug].iter().all(|v| v.is_none()) && v.gene.is_none() { return Err(QueryInvalid); }
        Ok(Self(v))
    }
}
impl ArticleSearch {
    /// Free text followed by variant, disease and drug selectors in the approved term order.
    pub fn words(&self) -> String { [&self.0.text,&self.0.variant,&self.0.disease,&self.0.drug].into_iter().flatten().map(Text::as_str).collect::<Vec<_>>().join(" ") }
    /// Optional gene identity selector.
    pub fn gene(&self) -> Option<&GeneInput> { self.0.gene.as_ref() }
    /// Page length.
    pub fn limit(&self) -> u32 { self.0.limit }
    /// Page offset.
    pub fn offset(&self) -> u32 { self.0.offset }
    /// Optional explicit snapshot.
    pub fn pool(&self) -> Option<&Text> { self.0.pool.as_ref() }
    /// Select a page without changing selectors.
    pub fn page(&self, offset:u32, pool:Option<Text>) -> Self { let mut v=self.clone();v.0.offset=offset;v.0.pool=pool;v }
    /// Canonical identity excludes cursor and page size.
    pub fn identity(&self) -> Self { let mut v=self.page(0,None);v.0.limit=limit();v }
}
