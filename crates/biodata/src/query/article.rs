use super::gene::{QueryInvalid, present};
use crate::{GeneInput, PartialDate, Text};
use serde::{Deserialize, Serialize};
/// Source selectors supported by article search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="str",extend("enum"=["pubmed", "europepmc", "semantic-scholar"])))]
pub enum ArticleSource {
    /// PubMed's own result order.
    Pubmed,
    /// Europe PMC native relevance or publication-date order.
    Europepmc,
    /// Semantic Scholar native relevance order.
    #[serde(rename = "semantic-scholar")]
    SemanticScholar,
}
/// Article ordering supported by this command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="str",extend("enum"=["rank", "date", "citations"])))]
pub enum ArticleSort {
    /// Native relevance order.
    Rank,
    /// PubMed most recently added order.
    Date,
    /// Whole-pool iCite citation count, with native-rank ties.
    Citations,
}
/// Publication-type selectors sent to the named sources.
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
    /// Words sent to each named source in its default fields.
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
    /// semantic-scholar: broad discovery; an optional free key raises the limit.
    #[serde(default = "sources")]
    #[cfg_attr(feature = "schema", schemars(length(min = 1, max = 3), extend("x-comma-separated"=true)))]
    source: Box<[ArticleSource]>,
    /// rank (default): native provider order; several sources take turns.
    /// date: newest first in each source; PubMed means date added to PubMed.
    /// citations: most cited first; costs about 1 s per 1,000 PMIDs.
    /// All sources support citations.
    #[serde(default = "sort")]
    sort: ArticleSort,
    /// Gene input; PubMed resolves it through HGNC.
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
            || v.source.len() > 3
            || v.source
                .iter()
                .enumerate()
                .any(|(i, s)| v.source[..i].contains(s))
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
    /// Change ordering and restart the request without an old pool.
    pub fn ordered(&self, sort: ArticleSort) -> Self {
        let mut v = self.page(0, None);
        v.0.sort = sort;
        v
    }
    /// Canonical identity excludes cursor and page size.
    pub fn identity(&self) -> Self {
        let mut v = self.page(0, None);
        v.0.limit = limit();
        v
    }
}

/// Source capability selectors checked before any lookup or transport request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = ["TEXT", "--gene", "--variant", "--disease", "--drug", "--author", "--journal", "--since", "--until", "--type", "--sort date"])))]
pub enum ArticleFilter {
    /// Free text.
    #[serde(rename = "TEXT")]
    Text,
    /// Gene selector.
    #[serde(rename = "--gene")]
    Gene,
    /// Variant selector.
    #[serde(rename = "--variant")]
    Variant,
    /// Disease selector.
    #[serde(rename = "--disease")]
    Disease,
    /// Drug selector.
    #[serde(rename = "--drug")]
    Drug,
    /// Author selector.
    #[serde(rename = "--author")]
    Author,
    /// Journal selector.
    #[serde(rename = "--journal")]
    Journal,
    /// Lower publication date bound.
    #[serde(rename = "--since")]
    Since,
    /// Upper publication date bound.
    #[serde(rename = "--until")]
    Until,
    /// Publication type selector.
    #[serde(rename = "--type")]
    Type,
    /// Native date ordering.
    #[serde(rename = "--sort date")]
    DateSort,
}
impl ArticleSearch {
    /// Whether any named unsupported capability is requested.
    pub fn rejects(&self, filters: &[ArticleFilter]) -> Option<ArticleFilter> {
        filters.iter().copied().find(|filter| match filter {
            ArticleFilter::Text => self.0.text.is_some(),
            ArticleFilter::Gene => self.0.gene.is_some(),
            ArticleFilter::Variant => self.0.variant.is_some(),
            ArticleFilter::Disease => self.0.disease.is_some(),
            ArticleFilter::Drug => self.0.drug.is_some(),
            ArticleFilter::Author => self.0.author.is_some(),
            ArticleFilter::Journal => self.0.journal.is_some(),
            ArticleFilter::Since => self.0.since.is_some(),
            ArticleFilter::Until => self.0.until.is_some(),
            ArticleFilter::Type => self.0.kind.is_some(),
            ArticleFilter::DateSort => self.0.sort == ArticleSort::Date,
        })
    }
}
