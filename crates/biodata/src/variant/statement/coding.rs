use serde::{Deserialize, Serialize};
/// A code in one explicitly named source framework.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "system", content = "code", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub enum Coding {
    /// CIViC evidence level.
    #[serde(rename = "civic_evidence_level")]
    CivicEvidenceLevel(CivicEvidenceLevelCode),
    /// acmg_amp_2015 vocabulary.
    #[serde(rename = "acmg_amp_2015")]
    AcmgAmp2015(AcmgAmp2015Code),
    /// clingen_low_penetrance_2024 vocabulary.
    #[serde(rename = "clingen_low_penetrance_2024")]
    ClingenLowPenetrance2024(ClingenLowPenetrance2024Code),
    /// amp_asco_cap_2017_tier vocabulary.
    #[serde(rename = "amp_asco_cap_2017_tier")]
    AmpAscoCap2017Tier(AmpAscoCap2017TierCode),
    /// amp_asco_cap_2017_level vocabulary.
    #[serde(rename = "amp_asco_cap_2017_level")]
    AmpAscoCap2017Level(AmpAscoCap2017LevelCode),
    /// clingen_cgc_vicc_2022 vocabulary.
    #[serde(rename = "clingen_cgc_vicc_2022")]
    ClingenCgcVicc2022(ClingenCgcVicc2022Code),
}
crate::macros::wire_enum! {
    /// Checked AcmgAmp2015 codes.
    AcmgAmp2015Code {
        /// Published pathogenic value.
        Pathogenic => "pathogenic",
        /// Published likely_pathogenic value.
        LikelyPathogenic => "likely_pathogenic",
        /// Published uncertain_significance value.
        UncertainSignificance => "uncertain_significance",
        /// Published likely_benign value.
        LikelyBenign => "likely_benign",
        /// Published benign value.
        Benign => "benign",
    }
}
crate::macros::wire_enum! {
    /// Checked ClingenLowPenetrance2024 codes.
    ClingenLowPenetrance2024Code {
        /// Published pathogenic_low_penetrance value.
        PathogenicLowPenetrance => "pathogenic_low_penetrance",
        /// Published likely_pathogenic_low_penetrance value.
        LikelyPathogenicLowPenetrance => "likely_pathogenic_low_penetrance",
        /// Published established_risk_allele value.
        EstablishedRiskAllele => "established_risk_allele",
        /// Published likely_risk_allele value.
        LikelyRiskAllele => "likely_risk_allele",
        /// Published uncertain_risk_allele value.
        UncertainRiskAllele => "uncertain_risk_allele",
    }
}
crate::macros::wire_enum! {
    /// Checked AmpAscoCap2017Tier codes.
    AmpAscoCap2017TierCode {
        /// Published tier_i value.
        TierI => "tier_i",
        /// Published tier_ii value.
        TierIi => "tier_ii",
        /// Published tier_iii value.
        TierIii => "tier_iii",
        /// Published tier_iv value.
        TierIv => "tier_iv",
    }
}
crate::macros::wire_enum! {
    /// Checked AmpAscoCap2017Level codes.
    AmpAscoCap2017LevelCode {
        /// Published level_a value.
        LevelA => "level_a",
        /// Published level_b value.
        LevelB => "level_b",
        /// Published level_c value.
        LevelC => "level_c",
        /// Published level_d value.
        LevelD => "level_d",
    }
}
crate::macros::wire_enum! {
    /// Checked ClingenCgcVicc2022 codes.
    ClingenCgcVicc2022Code {
        /// Published oncogenic value.
        Oncogenic => "oncogenic",
        /// Published likely_oncogenic value.
        LikelyOncogenic => "likely_oncogenic",
        /// Published uncertain_significance value.
        UncertainSignificance => "uncertain_significance",
        /// Published likely_benign value.
        LikelyBenign => "likely_benign",
        /// Published benign value.
        Benign => "benign",
    }
}

crate::macros::wire_enum! {
    /// Source-defined CIViC evidence levels.
    CivicEvidenceLevelCode {
        /// A.
        A => "a",
        /// B.
        B => "b",
        /// C.
        C => "c",
        /// D.
        D => "d",
        /// E.
        E => "e",
    }
}
