//! Exact GraphQL wire values from the CIViC publisher schema.
// https://github.com/griffithlab/civic-v2/tree/main/server/app/graphql/types
biodata::native_enum! {
    /// Native CIViC curation status from evidence_status_type.rb.
    CivicStatus {
        /// Accepted source record.
        Accepted => "ACCEPTED",
        /// Submitted source record.
        Submitted => "SUBMITTED",
        /// Rejected source record.
        Rejected => "REJECTED",
    }
}
biodata::native_enum! {
    /// Native CIViC evidence/assertion type from evidence_type_type.rb.
    CivicKind {
        /// Source diagnostic category.
        Diagnostic => "DIAGNOSTIC",
        /// Source prognostic category.
        Prognostic => "PROGNOSTIC",
        /// Source predictive category.
        Predictive => "PREDICTIVE",
        /// Source predisposing category.
        Predisposing => "PREDISPOSING",
        /// Source functional category.
        Functional => "FUNCTIONAL",
        /// Source oncogenic category.
        Oncogenic => "ONCOGENIC",
    }
}
biodata::native_enum! {
    /// Native CIViC significance from evidence_significance_type.rb and assertion_significance_type.rb.
    CivicSignificance {
        /// Publisher significance.
        SensitivityResponse => "SENSITIVITYRESPONSE",
        /// Publisher significance.
        Resistance => "RESISTANCE",
        /// Publisher significance.
        BetterOutcome => "BETTER_OUTCOME",
        /// Publisher significance.
        PoorOutcome => "POOR_OUTCOME",
        /// Publisher significance.
        Positive => "POSITIVE",
        /// Publisher significance.
        Negative => "NEGATIVE",
        /// Publisher significance.
        NotApplicable => "NA",
        /// Publisher significance.
        AdverseResponse => "ADVERSE_RESPONSE",
        /// Publisher significance.
        Pathogenic => "PATHOGENIC",
        /// Publisher significance.
        LikelyPathogenic => "LIKELY_PATHOGENIC",
        /// Publisher significance.
        Benign => "BENIGN",
        /// Publisher significance.
        LikelyBenign => "LIKELY_BENIGN",
        /// Publisher significance.
        UncertainSignificance => "UNCERTAIN_SIGNIFICANCE",
        /// Publisher significance.
        ReducedSensitivity => "REDUCED_SENSITIVITY",
        /// Publisher significance.
        GainOfFunction => "GAIN_OF_FUNCTION",
        /// Publisher significance.
        LossOfFunction => "LOSS_OF_FUNCTION",
        /// Publisher significance.
        UnalteredFunction => "UNALTERED_FUNCTION",
        /// Publisher significance.
        Neomorphic => "NEOMORPHIC",
        /// Publisher significance.
        UnknownFunction => "UNKNOWN",
        /// Publisher significance.
        DominantNegative => "DOMINANT_NEGATIVE",
        /// Publisher significance.
        Predisposition => "PREDISPOSITION",
        /// Publisher significance.
        Protectiveness => "PROTECTIVENESS",
        /// Publisher significance.
        Oncogenicity => "ONCOGENICITY",
        /// Publisher significance.
        LikelyOncogenic => "LIKELY_ONCOGENIC",
        /// Publisher assertion significance.
        Oncogenic => "ONCOGENIC",
    }
}

impl CivicKind {
    /// Parse assertion_type_type.rb, which omits evidence's FUNCTIONAL member.
    pub fn from_assertion(value: String) -> Self {
        match Self::from(value.clone()) {
            Self::Functional => Self::Unknown(value.into()),
            known => known,
        }
    }
}
impl CivicSignificance {
    /// Parse evidence_significance_type.rb, which omits assertion's ONCOGENIC member.
    pub fn from_evidence(value: String) -> Self {
        match Self::from(value.clone()) {
            Self::Oncogenic => Self::Unknown(value.into()),
            known => known,
        }
    }
    /// Parse the exact assertion_significance_type.rb field vocabulary.
    pub fn from_assertion(value: String) -> Self {
        match Self::from(value.clone()) {
            Self::GainOfFunction
            | Self::LossOfFunction
            | Self::UnalteredFunction
            | Self::Neomorphic
            | Self::UnknownFunction
            | Self::DominantNegative
            | Self::Predisposition
            | Self::Protectiveness
            | Self::Oncogenicity => Self::Unknown(value.into()),
            known => known,
        }
    }
}
