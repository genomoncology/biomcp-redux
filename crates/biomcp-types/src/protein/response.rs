use super::{
    ProteinComplexMembersQuery, ProteinDomainsQuery, ProteinGet, ProteinInput,
    ProteinRelationQuery, ProteinSearch, ProteinSection, ProteinSequenceQuery,
};
use crate::{ItemStatus, ResponseBody, Section};
use biodata::{
    ProteinAudit, ProteinComplexHeader, ProteinComplexMember, ProteinDomainMatch, ProteinFunction,
    ProteinHeader, ProteinIsoform, ProteinSequence, ProteinStructureRef, Text,
};
use serde::{Deserialize, Serialize};

page!(
    protein,
    ProteinPage,
    ProteinHeader,
    ProteinSearch,
    "biomcp.protein-page/6",
    "Source-ordered human reviewed protein headers."
);
/// How to read every span in a reply, stated once beside the spans themselves.
pub const COORDINATE_NOTE: &str = "Spans are zero-based interbase, half-open [start, end): start is included and end is excluded. \
    Original UniProt/InterPro source coordinates are one-based inclusive, so a source 10..20 is span 9..20; \
    source_coordinates describes that original convention. The --position selector remains one-based.";
/// The coordinate note when a reply holds at least one span.
pub fn coordinate_note(spans: bool) -> Option<Box<str>> {
    spans.then(|| COORDINATE_NOTE.into())
}
/// Whole native rows with the independent unfiltered source count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomains {
    // The totals come before the rows (0942): a reader of only the start sees them.
    /// Rows in the whole selected collection before paging; the count to report.
    #[serde(default)]
    pub total: u64,
    /// Native source total before type selection or exact participant admission.
    pub native_total: u64,
    /// Bounded retained rows in source order.
    #[serde(default)]
    pub items: Box<[ProteinDomainMatch]>,
    /// How to read the spans above; present when any row has a fragment, but not inside a card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinate_note: Option<Box<str>>,
}
impl ResponseBody for ProteinDomains {
    const SCHEMA: &'static str = "biomcp.protein-domains/9";
    type Query = ProteinDomainsQuery;
}
page!(
    protein,
    ProteinIsoforms,
    ProteinIsoform,
    ProteinRelationQuery,
    "biomcp.protein-isoforms/5",
    "Alternative products with every stated identifier and explicit direct sequence state."
);
/// Whole native rows with the independent unfiltered source count.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinComplexes {
    // The totals come before the rows (0942): a reader of only the start sees them.
    /// Rows in the whole selected collection before paging; the count to report.
    #[serde(default)]
    pub total: u64,
    /// Native source total before type selection or exact participant admission.
    pub native_total: u64,
    /// Bounded retained rows in source order.
    #[serde(default)]
    pub items: Box<[ProteinComplexHeader]>,
}
impl ResponseBody for ProteinComplexes {
    const SCHEMA: &'static str = "biomcp.protein-complexes/6";
    type Query = ProteinRelationQuery;
}
page!(
    protein,
    ProteinComplexMembers,
    ProteinComplexMember,
    ProteinComplexMembersQuery,
    "biomcp.protein-complex-members/5",
    "Native participants of one exact containing complex, including non-protein source facts."
);
page!(
    protein,
    ProteinStructures,
    ProteinStructureRef,
    ProteinRelationQuery,
    "biomcp.protein-structures/6",
    "Source structure links and native properties; no residue coverage claim."
);
/// Answers to requested inputs; ambiguous gene inputs retain all candidates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinCard {
    /// One result per input in caller order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub items: Box<[ProteinCardItem]>,
    /// How to read feature and domain spans; present when any section holds one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinate_note: Option<Box<str>>,
}
impl ResponseBody for ProteinCard {
    const SCHEMA: &'static str = "biomcp.protein-card/13";
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
    pub features: Option<Section<super::ProteinFeaturePage>>,
    /// Requested InterPro match page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Section<ProteinDomains>>,
    /// Requested native alternative-product page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isoforms: Option<Section<ProteinSectionPage<ProteinIsoform>>>,
    /// Requested exact participant-admitted header page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexes: Option<Section<ProteinComplexes>>,
    /// Requested STRING interaction partners, strongest first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Section<ProteinInteractions>>,
    /// Requested ChEMBL target and its assay and activity totals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chembl: Option<Section<ProteinChembl>>,
    /// Requested structure-link page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structures: Option<Section<ProteinSectionPage<ProteinStructureRef>>>,
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
    const SCHEMA: &'static str = "biomcp.protein-sequence/5";
    type Query = ProteinSequenceQuery;
}
/// Whole source function paragraphs and their own evidence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFunctionResult {
    /// Paragraphs in the whole section, all of them shown; the count to report (1022).
    #[serde(default)]
    pub total: u64,
    /// Native paragraphs in source order, without inferred interpretation.
    #[serde(default)]
    pub items: Box<[ProteinFunction]>,
}
impl ResponseBody for ProteinFunctionResult {
    const SCHEMA: &'static str = "biomcp.protein-function/5";
    type Query = ProteinGet;
}

/// Bounded card rows and the size of their complete UniProt collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinSectionPage<T> {
    /// Number of source rows before card bounding.
    pub total: u64,
    /// Whole native rows in source order.
    pub items: Box<[T]>,
}

/// STRING interaction partners of one protein, strongest combined score first.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinInteractions {
    /// Partners at or above `min_score` in the whole STRING list; the count to report.
    #[serde(default)]
    pub total: u64,
    /// The lowest combined score kept: STRING's medium confidence, 0.4.
    pub min_score: f64,
    /// STRING release that scored these partners.
    pub version: Text,
    /// STRING entry the asked accession maps to. STRING keeps one protein per gene, so distinct
    /// products of one gene, such as CDKN2A's P42771 and Q8N726, share one entry and one partner list.
    pub string_id: Text,
    /// Bounded first rows in STRING's order, highest combined score first.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[ProteinInteraction]>,
}
/// One STRING functional association; a score is STRING's confidence (0 to 1), not a measured affinity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinInteraction {
    /// STRING preferred name of the partner, usually its gene symbol.
    pub name: Text,
    /// Partner STRING protein identity.
    pub string_id: Text,
    /// STRING combined score from all evidence channels.
    pub score: f64,
    /// Channel scores that are above zero; an absent channel scored zero.
    pub evidence: InteractionEvidence,
}
/// STRING evidence channel scores (0 to 1); zero channels are omitted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct InteractionEvidence {
    /// Experimental and biochemical data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiments: Option<f64>,
    /// Curated pathway and complex databases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub databases: Option<f64>,
    /// Co-mention in the literature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textmining: Option<f64>,
    /// Co-expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coexpression: Option<f64>,
    /// Conserved gene neighborhood.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neighborhood: Option<f64>,
    /// Gene fusion events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fusion: Option<f64>,
    /// Phylogenetic co-occurrence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooccurrence: Option<f64>,
}
/// ChEMBL's single-protein targets for one UniProt accession, each with ChEMBL's own totals.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinChembl {
    /// The UniProt accession ChEMBL was asked about: an isoform's canonical accession, since
    /// ChEMBL names canonical accessions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accession: Option<Text>,
    /// Single-protein targets ChEMBL lists for the accession; the count to report.
    #[serde(default)]
    pub total: u64,
    /// The targets, each with its assay and activity totals.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[ProteinChemblTarget]>,
}
/// One ChEMBL target; its totals are ChEMBL's own counts, not rows BioMCP read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinChemblTarget {
    /// ChEMBL target ID, such as CHEMBL3105.
    pub id: Text,
    /// ChEMBL preferred target name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// ChEMBL target organism.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organism: Option<Text>,
    /// Assays ChEMBL lists for this target.
    pub assay_count: u64,
    /// Activity records ChEMBL lists for this target.
    pub activity_count: u64,
}
