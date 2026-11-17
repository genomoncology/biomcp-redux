//! Native CIViC evidence selectors shared by drug and variant queries.
use serde::{Deserialize, Serialize};

/// Restrict evidence at CIViC before retrieving a page.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CivicEvidenceFilter {
    #[doc = "A–E\nNative CIViC evidence levels; repeat to include several levels."]
    /// Only with --source civic on get drug, and the civic section on get variant.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub evidence_level: Box<[CivicEvidenceLevel]>,
    #[doc = "Status\nNative CIViC evidence status; omitted preserves the source's existing default."]
    /// Only with --source civic on get drug, and the civic section on get variant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_status: Option<CivicEvidenceStatus>,
    #[doc = "Significance\nNative CIViC evidence significance, such as resistance or sensitivityresponse."]
    /// Only with --source civic on get drug, and the civic section on get variant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_significance: Option<CivicEvidenceSignificance>,
}
impl CivicEvidenceFilter {
    /// Whether the caller selected any evidence restriction.
    pub fn is_empty(&self) -> bool {
        self.evidence_level.is_empty()
            && self.evidence_status.is_none()
            && self.evidence_significance.is_none()
    }
    /// Distinct levels in native A through E order, or the unrestricted stream.
    pub fn levels(&self) -> Vec<Option<CivicEvidenceLevel>> {
        if self.evidence_level.is_empty() {
            return vec![None];
        }
        [
            CivicEvidenceLevel::A,
            CivicEvidenceLevel::B,
            CivicEvidenceLevel::C,
            CivicEvidenceLevel::D,
            CivicEvidenceLevel::E,
        ]
        .into_iter()
        .filter(|level| self.evidence_level.contains(level))
        .map(Some)
        .collect()
    }
}
biodata::wire_enum! {
    /// Native CIViC evidence level.
    CivicEvidenceLevel {
        /// Validated evidence.
        A => "A",
        /// Clinical evidence.
        B => "B",
        /// Case study evidence.
        C => "C",
        /// Preclinical evidence.
        D => "D",
        /// Inferential evidence.
        E => "E",
    }
}
biodata::wire_enum! {
    /// Native CIViC evidence review status.
    CivicEvidenceStatus {
        /// Accepted evidence.
        Accepted => "accepted",
        /// Submitted evidence.
        Submitted => "submitted",
        /// Rejected evidence.
        Rejected => "rejected",
    }
}
biodata::wire_enum! {
    /// Native CIViC evidence significance, from the live EvidenceSignificance enum.
    CivicEvidenceSignificance {
        /// CIViC SENSITIVITYRESPONSE.
        SensitivityResponse => "sensitivityresponse",
        /// CIViC RESISTANCE.
        Resistance => "resistance",
        /// CIViC BETTER_OUTCOME.
        BetterOutcome => "better_outcome",
        /// CIViC POOR_OUTCOME.
        PoorOutcome => "poor_outcome",
        /// CIViC POSITIVE.
        Positive => "positive",
        /// CIViC NEGATIVE.
        Negative => "negative",
        /// CIViC NA.
        NotApplicable => "na",
        /// CIViC ADVERSE_RESPONSE.
        AdverseResponse => "adverse_response",
        /// CIViC PATHOGENIC.
        Pathogenic => "pathogenic",
        /// CIViC LIKELY_PATHOGENIC.
        LikelyPathogenic => "likely_pathogenic",
        /// CIViC BENIGN.
        Benign => "benign",
        /// CIViC LIKELY_BENIGN.
        LikelyBenign => "likely_benign",
        /// CIViC UNCERTAIN_SIGNIFICANCE.
        UncertainSignificance => "uncertain_significance",
        /// CIViC REDUCED_SENSITIVITY.
        ReducedSensitivity => "reduced_sensitivity",
        /// CIViC GAIN_OF_FUNCTION.
        GainOfFunction => "gain_of_function",
        /// CIViC LOSS_OF_FUNCTION.
        LossOfFunction => "loss_of_function",
        /// CIViC UNALTERED_FUNCTION.
        UnalteredFunction => "unaltered_function",
        /// CIViC NEOMORPHIC.
        Neomorphic => "neomorphic",
        /// CIViC UNKNOWN.
        UnknownFunction => "unknown",
        /// CIViC DOMINANT_NEGATIVE.
        DominantNegative => "dominant_negative",
        /// CIViC PREDISPOSITION.
        Predisposition => "predisposition",
        /// CIViC PROTECTIVENESS.
        Protectiveness => "protectiveness",
        /// CIViC ONCOGENICITY.
        Oncogenicity => "oncogenicity",
        /// CIViC LIKELY_ONCOGENIC.
        LikelyOncogenic => "likely_oncogenic",
    }
}
