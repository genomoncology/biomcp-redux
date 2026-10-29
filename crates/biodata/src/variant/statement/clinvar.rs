//! ClinVar coding spellings follow the publisher review and origin vocabularies.
// https://www.ncbi.nlm.nih.gov/clinvar/docs/review_status/
// https://www.ncbi.nlm.nih.gov/clinvar/docs/properties/
crate::wire_enum! {
    /// Checked ClinVar review status codes.
    ClinvarReviewStatusCode {
        /// Publisher label: practice guideline.
        PracticeGuideline => "practice_guideline",
        /// Publisher label: reviewed by expert panel.
        ExpertPanel => "expert_panel",
        /// Publisher label: criteria provided, multiple submitters, no conflicts.
        CriteriaMultipleSubmittersNoConflicts => "criteria_provided_multiple_submitters_no_conflicts",
        /// Publisher label: criteria provided, conflicting classifications.
        CriteriaConflictingClassifications => "criteria_provided_conflicting_classifications",
        /// Publisher label: criteria provided, multiple submitters.
        CriteriaMultipleSubmitters => "criteria_provided_multiple_submitters",
        /// Publisher label: criteria provided, single submitter.
        CriteriaSingleSubmitter => "criteria_provided_single_submitter",
        /// Publisher label: no assertion criteria provided.
        NoAssertionCriteria => "no_assertion_criteria_provided",
        /// Publisher label: no classification provided.
        NoClassification => "no_classification_provided",
        /// Publisher label: no classification for the individual variant.
        NoClassificationForIndividualVariant => "no_classification_for_the_individual_variant",
    }
}
impl ClinvarReviewStatusCode {
    /// Return the coding for an exact publisher label; preserve other labels uncoded.
    pub fn from_source_label(label: &str) -> Option<Self> {
        match label {
            "practice guideline" => Some(Self::PracticeGuideline),
            "reviewed by expert panel" => Some(Self::ExpertPanel),
            "criteria provided, multiple submitters, no conflicts" => Some(Self::CriteriaMultipleSubmittersNoConflicts),
            "criteria provided, conflicting classifications" => Some(Self::CriteriaConflictingClassifications),
            "criteria provided, multiple submitters" => Some(Self::CriteriaMultipleSubmitters),
            "criteria provided, single submitter" => Some(Self::CriteriaSingleSubmitter),
            "no assertion criteria provided" => Some(Self::NoAssertionCriteria),
            "no classification provided" => Some(Self::NoClassification),
            "no classification for the individual variant" => Some(Self::NoClassificationForIndividualVariant),
            _ => None,
        }
    }
}
crate::wire_enum! {
    /// Checked ClinVar allele origin codes.
    ClinvarAlleleOriginCode {
        /// Publisher label: germline.
        Germline => "germline",
        /// Publisher label: somatic.
        Somatic => "somatic",
        /// Publisher label: de novo.
        DeNovo => "de_novo",
        /// Publisher label: inherited.
        Inherited => "inherited",
        /// Publisher label: maternal.
        Maternal => "maternal",
        /// Publisher label: paternal.
        Paternal => "paternal",
        /// Publisher label: biparental.
        Biparental => "biparental",
        /// Publisher label: uniparental.
        Uniparental => "uniparental",
        /// Publisher label: not provided.
        NotProvided => "not_provided",
        /// Publisher label: unknown.
        Unknown => "unknown",
    }
}
impl ClinvarAlleleOriginCode {
    /// Return the coding for an exact publisher label; preserve other labels uncoded.
    pub fn from_source_label(label: &str) -> Option<Self> {
        match label {
            "germline" => Some(Self::Germline),
            "somatic" => Some(Self::Somatic),
            "de novo" => Some(Self::DeNovo),
            "inherited" => Some(Self::Inherited),
            "maternal" => Some(Self::Maternal),
            "paternal" => Some(Self::Paternal),
            "biparental" => Some(Self::Biparental),
            "uniparental" => Some(Self::Uniparental),
            "not provided" => Some(Self::NotProvided),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
}
