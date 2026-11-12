//! Several named drug sections read in one call, each the reply that section alone gives.
use super::{
    DrugApprovals, DrugCivic, DrugGet, DrugIndications, DrugInteractions, DrugLabel,
    DrugRegulatory, DrugTargets, DrugXrefs,
};
use crate::{Envelope, ResponseBody};
use serde::{Deserialize, Serialize};

/// Several sections of one drug record in one reply, one part per section and label heading.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugSections {
    /// Parts in the caller's order.
    pub parts: Box<[DrugSectionPart]>,
}

/// One requested section, named by `section`, and the reply a call for it alone gives in
/// `reply`, with its own status, sources, paging and next calls.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "section", content = "reply", rename_all = "kebab-case")]
pub enum DrugSectionPart {
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
