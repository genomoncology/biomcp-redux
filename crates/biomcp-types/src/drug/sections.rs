//! Several named drug sections read in one call, each the reply that section alone gives.
use super::{
    DrugApprovals, DrugCard, DrugCivic, DrugGet, DrugIndications, DrugInteractions, DrugLabel,
    DrugRegulatory, DrugSection, DrugTargets, DrugXrefs,
};
use crate::{Envelope, ResponseBody};
use biodata::Text;
use serde::{Deserialize, Serialize};

/// Several sections of one drug record in one reply, one part per section and label heading.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugSections {
    /// Parts in the caller's order.
    pub parts: Box<[DrugSectionPart]>,
}

/// One requested section and the reply a call for it alone gives, with its own paging.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugSectionPart {
    /// The section this part reads.
    pub section: DrugSection,
    /// The label heading this part reads, for a label part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_section: Option<Text>,
    /// The section's own reply: status, sources, page and next calls.
    pub reply: DrugSectionReply,
}

/// The reply of one drug section, named by its own schema.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum DrugSectionReply {
    /// Identity card, when the record was not found.
    Card(Box<Envelope<DrugCard>>),
    /// Cross-references.
    Xrefs(Box<Envelope<DrugXrefs>>),
    /// Targets.
    Targets(Box<Envelope<DrugTargets>>),
    /// Indications.
    Indications(Box<Envelope<DrugIndications>>),
    /// CIViC context.
    Civic(Box<Envelope<DrugCivic>>),
    /// One label heading.
    Label(Box<Envelope<DrugLabel>>),
    /// Approvals.
    Approvals(Box<Envelope<DrugApprovals>>),
    /// Regulator events.
    Regulatory(Box<Envelope<DrugRegulatory>>),
    /// Interactions.
    Interactions(Box<Envelope<DrugInteractions>>),
}

impl ResponseBody for DrugSections {
    const SCHEMA: &'static str = "biomcp.drug-sections/1";
    type Query = DrugGet;
}
