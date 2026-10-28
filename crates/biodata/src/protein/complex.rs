use crate::{ComplexPortalId, Curie, Text, UniProtIsoform};
use serde::{Deserialize, Serialize};
/// Exact source complex header admitted by a native participant identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinComplexHeader {
    /// Native complex accession.
    pub id: ComplexPortalId,
    /// Native source name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// Source taxonomy identifier.
    pub taxon: u32,
    /// Literal native predictedComplex flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicted: Option<bool>,
    /// Actual participants in this exact complex, never a complex search total.
    pub participant_total: u64,
    /// Exact native participant identifier supporting admission.
    pub matched_participant: Text,
    /// Native complex record URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
}
/// A native participant; names never establish a protein identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinComplexMember {
    /// Native participant identifier, including non-protein identifiers.
    pub id: Text,
    /// Literal native participant type; non-protein members remain source facts.
    pub participant_type: Text,
    /// Native taxonomy identifier when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxon: Option<u32>,
    /// Literal supplied organism text, including non-taxonomic source sentinels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organism: Option<Text>,
    /// Native display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// Native stoichiometry text; no numeric imputation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stoichiometry: Option<Text>,
    /// Source-supplied cross-references only.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub xrefs: Box<[Curie]>,
    /// Only an exact source-supplied protein identifier; never inferred from a name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protein: Option<UniProtIsoform>,
}
