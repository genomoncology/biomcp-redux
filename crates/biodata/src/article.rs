use crate::{Doi, PartialDate, Pmcid, Pmid, Text};
use serde::{Deserialize, Serialize};
/// Clinically meaningful PubMed publication types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub enum PublicationType {
    /// Randomized controlled trial.
    RandomizedControlledTrial,
    /// Clinical trial.
    ClinicalTrial,
    /// Phase one trial.
    ClinicalTrialPhase1,
    /// Phase two trial.
    ClinicalTrialPhase2,
    /// Phase three trial.
    ClinicalTrialPhase3,
    /// Phase four trial.
    ClinicalTrialPhase4,
    /// Meta-analysis.
    MetaAnalysis,
    /// Systematic review.
    SystematicReview,
    /// Review.
    Review,
    /// Practice guideline.
    PracticeGuideline,
    /// Guideline.
    Guideline,
    /// Case report.
    CaseReport,
    /// Observational study.
    ObservationalStudy,
    /// Multicenter study.
    MulticenterStudy,
    /// Letter.
    Letter,
    /// Comment.
    Comment,
    /// Editorial.
    Editorial,
    /// Preprint.
    Preprint,
    /// Retracted publication.
    RetractedPublication,
    /// Retraction notice.
    RetractionNotice,
    /// Erratum.
    Erratum,
}
/// Citation metadata owned by PubMed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleHeader {
    /// Checked PubMed identity.
    pub pmid: Pmid,
    /// Checked DOI, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<Doi>,
    /// Checked PMC identity, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmcid: Option<Pmcid>,
    /// Whole source title.
    pub title: Text,
    /// Journal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Text>,
    /// Publication date at source precision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<PartialDate>,
    /// Clinical publication types.
    pub types: Box<[PublicationType]>,
    /// First author in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_author: Option<Text>,
    /// Whole source author count.
    pub author_count: u32,
}
