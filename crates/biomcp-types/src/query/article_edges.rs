use super::gene::{QueryInvalid, present};
use crate::{ArticleInput, Pmid, Text};
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
    /// PMID, PMC or DOI; iCite accepts only a PMID.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pmid: ArticleInput,
    /// Graph provider; iCite remains the default.
    #[serde(default)]
    source: EdgeSource,
    /// Maximum papers on one page.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Number of pool items to skip.
    #[serde(default)]
    offset: u32,
    /// Fixed snapshot from an earlier page.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
}
fn limit() -> u32 {
    10
}
crate::query::validation::checked_query!(ArticleEdges, Fields);
impl TryFrom<Fields> for ArticleEdges {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, Self::Error> {
        if matches!(v.pmid, ArticleInput::EuropePmc(_)) {
            return Err(QueryInvalid::new(&["pmid"]));
        }
        if !(1..=50).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if v.source == EdgeSource::Icite && !matches!(v.pmid, ArticleInput::Pmid(_)) {
            return Err(QueryInvalid::new(&["source", "pmid"]));
        }
        Ok(Self(v))
    }
}
impl ArticleEdges {
    /// First page of citing or referenced papers.
    pub fn new(pmid: Pmid) -> Self {
        Self(Fields {
            pmid: ArticleInput::Pmid(pmid),
            source: EdgeSource::Icite,
            limit: limit(),
            offset: 0,
            pool: None,
        })
    }
    /// Parent paper.
    pub fn pmid(&self) -> Option<Pmid> {
        if let ArticleInput::Pmid(id) = self.0.pmid {
            Some(id)
        } else {
            None
        }
    }
    /// Checked article identity for the selected provider.
    pub fn input(&self) -> &ArticleInput {
        &self.0.pmid
    }
    /// Graph provider.
    pub fn source(&self) -> EdgeSource {
        self.0.source
    }
    /// Select another graph provider and restart its pool.
    pub fn sourced(&self, source: EdgeSource) -> Self {
        let mut q = self.page(0, None);
        q.0.source = source;
        q
    }
    /// Page length.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Snapshot selector.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Select a page without changing identity.
    pub fn page(&self, offset: u32, pool: Option<Text>) -> Self {
        let mut v = self.clone();
        v.0.offset = offset;
        v.0.pool = pool;
        v
    }
    /// Canonical selectors for one edge direction.
    pub fn identity(&self) -> Self {
        let mut q = self.page(0, None);
        q.0.limit = limit();
        q
    }
}

/// Graph edge provider, independent of article search providers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with="str", extend("enum"=["icite", "semantic-scholar"])))]
pub enum EdgeSource {
    /// NIH iCite PubMed graph.
    #[default]
    #[serde(rename = "icite")]
    Icite,
    /// Semantic Scholar graph with contexts and intents.
    #[serde(rename = "semantic-scholar")]
    SemanticScholar,
}
