use super::gene::{QueryInvalid, present};
use crate::{ArticleInput, Text};
use serde::{Deserialize, Serialize};
/// Read every source context and optional explicit full-text citation paragraph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct ArticleEvidence(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Citing PMID, PMC identity or DOI.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    citing: ArticleInput,
    /// Cited PMID, PMC identity or DOI.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=1)))]
    cited: ArticleInput,
    /// Include the citing paper's own explicit reference paragraphs.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    fulltext: bool,
    /// Stored checked source scan and original hit positions.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
    /// Context fragment position returned by the advancing step.
    #[serde(default)]
    offset: u32,
    /// Full-text fragment position returned by the advancing step.
    #[serde(default)]
    paragraph_offset: u32,
}
impl TryFrom<Fields> for ArticleEvidence {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if [&v.citing, &v.cited]
            .iter()
            .any(|id| matches!(id, ArticleInput::EuropePmc(_)))
            || v.pool.is_none() && (v.offset != 0 || v.paragraph_offset != 0)
            || !v.fulltext && v.paragraph_offset != 0
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl ArticleEvidence {
    /// Initial evidence request.
    pub fn new(citing: ArticleInput, cited: ArticleInput) -> Self {
        Self(Fields {
            citing,
            cited,
            fulltext: false,
            pool: None,
            offset: 0,
            paragraph_offset: 0,
        })
    }
    /// Source article.
    pub fn citing(&self) -> &ArticleInput {
        &self.0.citing
    }
    /// Target article.
    pub fn cited(&self) -> &ArticleInput {
        &self.0.cited
    }
    /// Include explicit JATS reference paragraphs.
    pub fn fulltext(&self) -> bool {
        self.0.fulltext
    }
    /// Fixed evidence scan.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Original context fragment offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Original paragraph fragment offset.
    pub fn paragraph_offset(&self) -> u32 {
        self.0.paragraph_offset
    }
    /// Advance without changing either paper or optional source selection.
    pub fn page(&self, pool: Text, offset: u32, paragraph_offset: u32) -> Self {
        let mut q = self.clone();
        q.0.pool = Some(pool);
        q.0.offset = offset;
        q.0.paragraph_offset = paragraph_offset;
        q
    }
    /// Checked identity for a stored scan.
    pub fn identity(&self) -> Self {
        let mut q = self.clone();
        q.0.pool = None;
        q.0.offset = 0;
        q.0.paragraph_offset = 0;
        q
    }
}
