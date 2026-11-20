//! Indication rows share the identity of the requested drug.
use super::{DrugGet, DrugHeader, DrugRecords};
use crate::{ResponseBody, Section};
use biodata::{
    Attribution, DrugDiseaseReference, DrugIdentity, DrugIndicationClaim, DrugNativeReference,
    DrugStage, Text,
};
use serde::{Deserialize, Serialize};

/// Source-native indication wording, populations and combinations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugIndications {
    /// Source-native drug identity shared by all indication records.
    pub identity: DrugHeader,
    /// Indication facts and how to find related FDA application facts.
    #[serde(flatten)]
    pub part: DrugIndicationsPart,
}

/// Indication facts for the reply's drug identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugIndicationsPart {
    /// Attribution shared by Open Targets indications. ChEMBL identifies each claim separately.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indication_attribution: Option<Attribution>,
    /// The command for FDA application events matched independently by drug name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regulatory_context: Option<Box<str>>,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub indications: Section<DrugRecords<DrugIndicationRow>>,
}

/// One claim about the drug identified by the reply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugIndicationRow {
    /// Complete native indication wording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Text>,
    /// Source-native disease references.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub disease_references: Box<[DrugDiseaseReference]>,
    /// Source-native development stage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<DrugStage>,
    /// Native supporting references.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub references: Box<[DrugNativeReference]>,
    /// Native population wording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population: Option<Text>,
    /// Native combination members.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub combination: Box<[DrugIdentity]>,
    /// Provenance of an individual claim when the source identifies it separately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

impl DrugIndicationRow {
    /// The owning reply supplies the drug identity and, for Open Targets, attribution.
    pub fn from_claim(claim: DrugIndicationClaim, include_attribution: bool) -> Self {
        Self {
            indication: claim.indication,
            disease_references: claim.disease_references,
            stage: claim.stage,
            references: claim.references,
            population: claim.population,
            combination: claim.combination,
            attribution: include_attribution.then_some(claim.attribution),
        }
    }
}

impl ResponseBody for DrugIndications {
    const SCHEMA: &'static str = "biomcp.drug-indications/10";
    type Query = DrugGet;
}
