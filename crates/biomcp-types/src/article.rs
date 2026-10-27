mod card;
mod indexing;
pub use indexing::{Indexing, MeshHeading, MeshQualifier};
mod input;
use crate::{Doi, EuropePmcId, PartialDate, Pmcid, Pmid, Text};
pub use card::{AbstractPart, Article, Correction, CorrectionKind, CorrectionRelation};
pub use input::{ArticleInput, ArticleMatchKind};
use serde::{Deserialize, Serialize};
biodata::canonical_enum! {
    /// Clinically meaningful PubMed publication types.
    PublicationType, SourceUnknown {
        /// Randomized controlled trial.
        RandomizedControlledTrial => "randomized_controlled_trial",
        /// Clinical trial.
        ClinicalTrial => "clinical_trial",
        /// Phase one trial.
        ClinicalTrialPhase1 => "clinical_trial_phase_1",
        /// Phase two trial.
        ClinicalTrialPhase2 => "clinical_trial_phase_2",
        /// Phase three trial.
        ClinicalTrialPhase3 => "clinical_trial_phase_3",
        /// Phase four trial.
        ClinicalTrialPhase4 => "clinical_trial_phase_4",
        /// Meta-analysis.
        MetaAnalysis => "meta_analysis",
        /// Systematic review.
        SystematicReview => "systematic_review",
        /// Review.
        Review => "review",
        /// Practice guideline.
        PracticeGuideline => "practice_guideline",
        /// Guideline.
        Guideline => "guideline",
        /// Case report.
        CaseReport => "case_report",
        /// Observational study.
        ObservationalStudy => "observational_study",
        /// Multicenter study.
        MulticenterStudy => "multicenter_study",
        /// Letter.
        Letter => "letter",
        /// Comment.
        Comment => "comment",
        /// Editorial.
        Editorial => "editorial",
        /// Preprint.
        Preprint => "preprint",
        /// Retracted publication.
        RetractedPublication => "retracted_publication",
        /// Retraction notice.
        RetractionNotice => "retraction_notice",
        /// Erratum.
        Erratum => "erratum",
    }
}
/// Citation metadata from PubMed or Europe PMC.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticleHeader {
    /// Checked PubMed identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Pmid"))]
    pub pmid: Option<Pmid>,
    /// Checked DOI, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<Doi>,
    /// Checked PMC identity, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmcid: Option<Pmcid>,
    /// Europe PMC source-local identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub europepmc_id: Option<EuropePmcId>,
    /// Whole source title.
    pub title: Text,
    /// Journal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Text>,
    /// Publication date at source precision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<PartialDate>,
    /// Electronic publication date when different from the publication date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub electronic_date: Option<PartialDate>,
    /// Clinical publication types.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub types: Box<[PublicationType]>,
    /// The provider does not supply checked publication types.
    #[serde(default, skip_serializing_if = "is_false")]
    pub types_unknown: bool,
    /// First author in source order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_author: Option<Text>,
    /// Exact Semantic Scholar ID for the displayed native first author, when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_author_id: Option<biodata::SemanticScholarAuthorId>,
    /// Whole source author count.
    pub author_count: u32,
}

fn is_false(value: &bool) -> bool {
    !value
}
