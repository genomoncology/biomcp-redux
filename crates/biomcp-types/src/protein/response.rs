use super::{
    ProteinComplexMembersQuery, ProteinDomainsQuery, ProteinFeaturesQuery, ProteinGet,
    ProteinInput, ProteinRelationQuery, ProteinSearch, ProteinSection, ProteinSequenceQuery,
};
use crate::{ItemStatus, ResponseBody, Section};
use biodata::{
    ProteinAudit, ProteinComplexHeader, ProteinComplexMember, ProteinDomainMatch, ProteinFeature,
    ProteinFunction, ProteinHeader, ProteinIsoform, ProteinSequence, ProteinStructureRef,
};
use serde::{Deserialize, Serialize};
macro_rules! page {
    ($name:ident, $row:ty, $query:ty, $schema:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
                #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        pub struct $name {
            /// Whole native rows in source order; totals apply only to this collection.
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            pub items: Box<[$row]>,
        }
        impl ResponseBody for $name {
            const SCHEMA: &'static str = $schema;
            type Query = $query;
        }
    };
}
page!(
    ProteinPage,
    ProteinHeader,
    ProteinSearch,
    "biomcp.protein-page/1",
    "Source-ordered human reviewed protein headers."
);
page!(
    ProteinFeatures,
    ProteinFeature,
    ProteinFeaturesQuery,
    "biomcp.protein-features/1",
    "Native UniProt feature rows, preserving exact labels and uncertain bounds."
);
/// Whole native rows with the independent unfiltered source count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomains {
    /// Bounded retained rows in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[ProteinDomainMatch]>,
    /// Native source total before type selection or exact participant admission.
    pub native_total: u64,
}
impl ResponseBody for ProteinDomains {
    const SCHEMA: &'static str = "biomcp.protein-domains/3";
    type Query = ProteinDomainsQuery;
}
page!(
    ProteinIsoforms,
    ProteinIsoform,
    ProteinRelationQuery,
    "biomcp.protein-isoforms/1",
    "Alternative products with every stated identifier and explicit direct sequence state."
);
/// Whole native rows with the independent unfiltered source count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinComplexes {
    /// Bounded retained rows in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[ProteinComplexHeader]>,
    /// Native source total before type selection or exact participant admission.
    pub native_total: u64,
}
impl ResponseBody for ProteinComplexes {
    const SCHEMA: &'static str = "biomcp.protein-complexes/3";
    type Query = ProteinRelationQuery;
}
page!(
    ProteinComplexMembers,
    ProteinComplexMember,
    ProteinComplexMembersQuery,
    "biomcp.protein-complex-members/1",
    "Native participants of one exact containing complex, including non-protein source facts."
);
page!(
    ProteinStructures,
    ProteinStructureRef,
    ProteinRelationQuery,
    "biomcp.protein-structures/1",
    "Source structure links and native properties; no residue coverage claim."
);
/// Answers to requested inputs; ambiguous gene inputs retain all candidates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinCard {
    /// One result per input in caller order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub items: Box<[ProteinCardItem]>,
}
impl ResponseBody for ProteinCard {
    const SCHEMA: &'static str = "biomcp.protein-card/4";
    type Query = ProteinGet;
}
/// One requested identity and its source outcome, without first-hit selection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinCardItem {
    /// Original caller identity.
    pub input: ProteinInput,
    /// Outcome of this input.
    pub status: ItemStatus,
    /// Exact resolved native header; absent for ambiguity or failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<ProteinHeader>,
    /// Competing exact source identities when a gene does not resolve uniquely.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub candidates: Box<[ProteinHeader]>,
    /// Native audit for the selected entry only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit: Option<ProteinAudit>,
    /// Requested bounded sections only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<ProteinSections>,
    /// Supported sections not requested.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub more: Box<[ProteinSection]>,
}
/// Independent source section states; all discloses bounded first pages only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinSections {
    /// Requested UniProt feature page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Section<ProteinFeatures>>,
    /// Requested InterPro match page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Section<ProteinDomains>>,
    /// Requested native alternative-product page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isoforms: Option<Section<ProteinIsoforms>>,
    /// Requested exact participant-admitted header page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexes: Option<Section<ProteinComplexes>>,
    /// Requested structure-link page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structures: Option<Section<ProteinStructures>>,
    /// Requested whole native function text and evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<Section<ProteinFunctionResult>>,
    /// Requested direct sequence; absent/unavailable states remain explicit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Section<ProteinSequenceResult>>,
}
/// One directly returned source sequence, with exact identity and audit metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinSequenceResult {
    /// Native sequence record; never constructed from another entry.
    pub sequence: ProteinSequence,
}
impl ResponseBody for ProteinSequenceResult {
    const SCHEMA: &'static str = "biomcp.protein-sequence/1";
    type Query = ProteinSequenceQuery;
}
/// Whole source function paragraphs and their own evidence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFunctionResult {
    /// Native paragraphs in source order, without inferred interpretation.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub items: Box<[ProteinFunction]>,
}
impl ResponseBody for ProteinFunctionResult {
    const SCHEMA: &'static str = "biomcp.protein-function/2";
    type Query = ProteinGet;
}
