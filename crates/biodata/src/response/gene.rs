mod civic;
mod clingen;
mod constraint;
mod expression;
mod oncokb;
mod pathways;
mod protein;
use super::{ItemStatus, ResponseBody, Section};
use crate::GeneSection;
use crate::{GeneGet, GeneHeader, GeneInput, GeneResolution, GeneSearch};
pub use civic::CivicGene;
pub use clingen::{ClinGenGene, ClinGenValidity};
pub use constraint::{Constraint, FiniteNumber, FiniteNumberError};
pub use expression::{CancerPtpm, Expression, RnaAbundance, TissueNtpm};
pub use oncokb::{Oncokb, OncokbCard, OncokbGeneType, OncokbTranscript};
pub use pathways::{Capped, PathwayRef, ReactomeId, ReactomeIdInvalid};
pub use protein::Protein;
use serde::{Deserialize, Serialize};
/// Answer to one requested input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneItem {
    /// Verified keyed source card annotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncokb: Option<OncokbCard>,
    /// Original checked input.
    pub input: GeneInput,
    /// This input's outcome.
    pub status: ItemStatus,
    /// Absent only when identity lookup failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<GeneResolution>,
    /// Requested sections, present only for a resolved identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<GeneSections>,
    /// Applicable sections left out of this request.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub more: Box<[GeneSection]>,
}
/// Gene answers in input order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneCards {
    /// Requested inputs and their answers.
    pub items: Box<[GeneItem]>,
}
/// One page of gene headers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenePage {
    /// Headers after ranking and paging.
    pub items: Box<[GeneHeader]>,
}
impl ResponseBody for GeneCards {
    const SCHEMA: &'static str = "biomcp.gene_cards/1";
    type Query = GeneGet;
}
impl ResponseBody for GenePage {
    const SCHEMA: &'static str = "biomcp.gene_page/1";
    type Query = GeneSearch;
}
/// Typed dispatch result before the interface constructs an envelope.
#[derive(Clone, Debug)]
pub enum QueryBody {
    /// Native NCI discovery page.
    NciTrialPage(crate::NciTrialPage),
    /// Native NCI exact cards.
    NciTrialCards(crate::NciTrialCards),
    /// Fixed NCI disease identity guidance.
    NciDiseaseError,

    /// Exact trial cards and source protocol sections.
    TrialCards(crate::TrialCards),
    /// Native registry search page.
    TrialPage(crate::TrialPage),
    /// Explicit PDF save result.
    ArticlePdf(crate::ArticlePdfResult),
    /// Invalid destination or unavailable save surface, retaining the source link.
    PdfSaveRefused(crate::PdfLink),
    /// An explicit article save was refused, retaining its source URL.
    ArticleSaveRefused(crate::Text),
    /// Citation context evidence.
    ArticleEvidence(crate::ArticleEvidenceResult),
    /// Source-native recommendations.
    ArticleRecommendations(crate::ArticleRecommendations),
    /// Licensed supplement reader.
    ArticleSupplement(crate::ArticleSupplementResult),
    /// A selected source cannot apply the checked filters.
    UnsupportedFilter {
        /// Selected source that cannot apply the filter.
        source: crate::Source,
        /// Exact unsupported selector, without caller text.
        option: crate::ArticleFilter,
    },
    /// Bounded licensed body reader.
    ArticleText(crate::ArticleTextResult),
    /// Invalid body reader position.
    TextPositionError,
    /// Citation graph page.
    ArticleEdges(crate::ArticleEdgePage),
    /// Citation ordering exceeds the supported PMID window.
    TooManyPmids {
        /// Unique PMID count before enrichment.
        count: u64,
    },
    /// PubMed article cards.
    ArticleCards(crate::ArticleCards),
    /// Invalid snapshot selector; the interface spells its typed restart.
    PoolError {
        /// Whether selectors differ from an existing snapshot.
        mismatch: bool,
    },
    /// PubMed search page.
    ArticlePage(crate::ArticlePage),
    /// Gene get answers.
    GeneCards(GeneCards),
    /// Gene search page.
    GenePage(GenePage),
}

/// Explicit source synchronization selector.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncQuery {
    /// Source whose complete identity data is synchronized.
    pub source: crate::Source,
}
/// Published bundle metadata after synchronization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SyncResult {
    /// Imported rows.
    pub rows: u64,
    /// Unencoded source bytes.
    pub bytes: u64,
}
impl ResponseBody for SyncResult {
    const SCHEMA: &'static str = "biomcp.sync/1";
    type Query = SyncQuery;
}

/// Landed gene sections, each retaining its source and explicit state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneSections {
    /// Reviewed human proteins listed by HGNC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein: Option<Section<Box<[Protein]>>>,
    /// CIViC gene-level curation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub civic: Option<Section<CivicGene>>,
    /// ClinGen validity and dosage assessments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clingen: Option<Section<ClinGenGene>>,
    /// Population constraint reported by gnomAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Section<Constraint>>,
    /// Human Protein Atlas expression annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Section<Expression>>,
    /// Lowest-level human Reactome mappings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathways: Option<Section<Capped<PathwayRef>>>,
    /// Keyed source gene details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncokb: Option<Section<Oncokb>>,
}
