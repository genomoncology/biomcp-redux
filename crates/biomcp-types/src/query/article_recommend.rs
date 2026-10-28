use super::gene::{QueryInvalid, present};
use crate::{ArticleInput, Text};
use serde::{Deserialize, Serialize};
/// Recommendations in the provider's recent-paper pool order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct ArticleRecommend(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// One to ten positive paper identities.
    #[cfg_attr(feature="schema", schemars(length(min=1,max=10), extend("x-positional"=0)))]
    seeds: Box<[ArticleInput]>,
    /// Up to ten negative paper identities.
    #[serde(default)]
    #[cfg_attr(feature = "schema", schemars(length(max = 10)))]
    negative: Box<[ArticleInput]>,
    /// Maximum recommendations requested, in native order.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 100)))]
    limit: u32,
    /// Number of stored recommendations to skip.
    #[serde(default)]
    offset: u32,
    /// Fixed recommendation pool.
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
crate::query::validation::checked_query!(ArticleRecommend, Fields);
impl TryFrom<Fields> for ArticleRecommend {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if v.seeds.is_empty() {
            return Err(QueryInvalid::new(&["seeds"]));
        }
        if v.seeds.len() > 10 {
            return Err(QueryInvalid::new(&["seeds"]));
        }
        if v.negative.len() > 10 {
            return Err(QueryInvalid::new(&["negative"]));
        }
        if !(1..=100).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if v.seeds
            .iter()
            .chain(v.negative.iter())
            .any(|id| matches!(id, ArticleInput::EuropePmc(_)))
        {
            return Err(QueryInvalid::new(&["seeds", "negative"]));
        }
        Ok(Self(v))
    }
}
impl ArticleRecommend {
    /// Native recent-paper recommendations from one seed.
    pub fn new(seed: ArticleInput) -> Self {
        Self(Fields {
            seeds: vec![seed].into(),
            negative: Box::default(),
            limit: limit(),
            offset: 0,
            pool: None,
        })
    }
    /// Positive identities.
    pub fn seeds(&self) -> &[ArticleInput] {
        &self.0.seeds
    }
    /// Negative identities.
    pub fn negative(&self) -> &[ArticleInput] {
        &self.0.negative
    }
    /// Source window bound.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Stored result position.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Snapshot identifier.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Advance the fixed source window.
    pub fn page(&self, offset: u32, pool: Option<Text>) -> Self {
        let mut q = self.clone();
        q.0.offset = offset;
        q.0.pool = pool;
        q
    }
    /// Source request identity includes the requested native window length.
    pub fn identity(&self) -> Self {
        self.page(0, None)
    }
}
