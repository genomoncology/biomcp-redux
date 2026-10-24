mod card;
mod input;
use crate::{Doi, PartialDate, Pmcid, Pmid, Text};
pub use card::{AbstractPart, Article, Correction, CorrectionKind, CorrectionRelation};
pub use input::{ArticleInput, ArticleMatchKind};
use serde::{Deserialize, Serialize};
crate::source::canonical_enum! {
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
