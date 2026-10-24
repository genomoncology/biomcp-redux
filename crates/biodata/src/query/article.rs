use super::gene::{QueryInvalid, present};
use crate::{GeneInput, PartialDate, Text};
use serde::{Deserialize, Serialize};
/// Source selectors supported by article search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="str",extend("enum"=["pubmed", "europepmc"])))]
pub enum ArticleSource {
    /// PubMed's own result order.
    Pubmed,
    /// Europe PMC native relevance or publication-date order.
    Europepmc,
}
/// Article ordering supported by this command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="str",extend("enum"=["rank", "date"])))]
pub enum ArticleSort {
    /// Native relevance order.
    Rank,
    /// PubMed most recently added order.
    Date,
}
/// Publication-type selectors sent to PubMed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="str",extend("enum"=["trial", "randomized", "review", "systematic_review", "meta_analysis", "guideline", "case_report"])))]
pub enum ArticleType {
    /// Clinical trials, including narrower phases.
    Trial,
    /// Randomized controlled trials.
    Randomized,
    /// Reviews.
    Review,
    /// Systematic reviews.
    SystematicReview,
    /// Meta-analyses.
    MetaAnalysis,
    /// Guidelines and practice guidelines.
    Guideline,
    /// Case reports.
    CaseReport,
}
/// Checked search selectors and pool cursor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="Fields", extend("anyOf"=[{"required":["text"]},{"required":["gene"]},{"required":["variant"]},{"required":["disease"]},{"required":["drug"]},{"required":["author"]},{"required":["journal"]},{"required":["since"]},{"required":["until"]},{"required":["type"]}])))]
pub struct ArticleSearch(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Words sent to PubMed in its default fields.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature="schema", schemars(with="Text", extend("x-positional"=0)))]
    text: Option<Text>,
    /// Sources to search in caller order, comma separated. Default: pubmed.
    /// Several sources take turns, remove duplicates and show every native rank.
    /// pubmed: keyword search with PubMed MeSH mapping, in plain PubMed order.
    /// europepmc: words in open full text, and preprints.
    #[serde(default = "sources")]
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 2), extend("x-comma-separated"=true)))]
    source: Box<[ArticleSource]>,
    /// rank (default): native provider order; several sources take turns.
    /// date: newest first in each source; PubMed means date added to PubMed.
    /// Both pubmed and europepmc support rank and date.
    #[serde(default = "sort")]
    sort: ArticleSort,
    /// Gene input resolved through HGNC.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "GeneInput"))]
    gene: Option<GeneInput>,
    /// Variant words.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    variant: Option<Text>,
    /// Disease words.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    disease: Option<Text>,
    /// Drug words.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    drug: Option<Text>,
    /// Author words sent to PubMed's author field.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    author: Option<Text>,
    /// Journal words sent to PubMed's journal field.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    journal: Option<Text>,
    /// Inclusive lower publication-date bound at the stated precision.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PartialDate"))]
    since: Option<PartialDate>,
    /// Inclusive upper publication-date bound at the stated precision.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PartialDate"))]
    until: Option<PartialDate>,
    /// Publication-type filter.
    #[serde(
        default,
        rename = "type",
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "ArticleType"))]
    kind: Option<ArticleType>,
    /// Maximum page length.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Number of pool items to skip.
    #[serde(default)]
    offset: u32,
    /// Stable snapshot from an earlier response.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
}
fn sources() -> Box<[ArticleSource]> {
    Box::new([ArticleSource::Pubmed])
}
fn sort() -> ArticleSort {
    ArticleSort::Rank
}
fn limit() -> u32 {
    10
}
impl TryFrom<Fields> for ArticleSearch {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if !(1..=50).contains(&v.limit)
            || v.source.is_empty()
            || v.source.len() > 2
            || (v.source.len() == 2 && v.source.first() == v.source.get(1))
            || [
                &v.text, &v.variant, &v.disease, &v.drug, &v.author, &v.journal,
            ]
            .iter()
            .all(|v| v.is_none())
                && v.gene.is_none()
                && v.since.is_none()
                && v.until.is_none()
                && v.kind.is_none()
            || v.since.zip(v.until).is_some_and(|(since, until)| {
                (
                    since.year(),
                    since.month().unwrap_or(1),
                    since.day().unwrap_or(1),
                ) > (
                    until.year(),
                    until.month().unwrap_or(12),
                    until.day().unwrap_or(31),
                )
            })
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl ArticleSearch {
    /// Sources in the caller’s turn order.
    pub fn sources(&self) -> &[ArticleSource] {
        &self.0.source
    }
    /// Native provider order requested by the caller.
    pub fn sort(&self) -> ArticleSort {
        self.0.sort
    }
    /// Author filter.
    pub fn author(&self) -> Option<&Text> {
        self.0.author.as_ref()
    }
    /// Journal filter.
    pub fn journal(&self) -> Option<&Text> {
        self.0.journal.as_ref()
    }
    /// Inclusive date bounds, preserving precision.
    pub fn dates(&self) -> (Option<PartialDate>, Option<PartialDate>) {
        (self.0.since, self.0.until)
    }
    /// Requested publication type.
    pub fn kind(&self) -> Option<ArticleType> {
        self.0.kind
    }

    /// Free text followed by variant, drug and disease selectors in the approved term order.
    pub fn words(&self) -> String {
        [&self.0.text, &self.0.variant, &self.0.drug, &self.0.disease]
            .into_iter()
            .flatten()
            .map(Text::as_str)
            .collect::<Vec<_>>()
            .join(" ")
    }
    /// Optional gene identity selector.
    pub fn gene(&self) -> Option<&GeneInput> {
        self.0.gene.as_ref()
    }
    /// Replace only the checked gene selector.
    pub fn with_gene(&self, gene: GeneInput) -> Self {
        let mut v = self.clone();
        v.0.gene = Some(gene);
        v
    }
    /// Page length.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Page offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Optional explicit snapshot.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Select a page without changing selectors.
    pub fn page(&self, offset: u32, pool: Option<Text>) -> Self {
        let mut v = self.clone();
        v.0.offset = offset;
        v.0.pool = pool;
        v
    }
    /// Canonical identity excludes cursor and page size.
    pub fn identity(&self) -> Self {
        let mut v = self.page(0, None);
        v.0.limit = limit();
        v
    }
}
