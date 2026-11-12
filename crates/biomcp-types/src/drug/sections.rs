//! Several named drug sections read in one call: the drug once, then one part per section.
use super::{
    DateRoleLabel, DrugApprovals, DrugCivic, DrugCivicStreams, DrugFdaProduct, DrugFdaSubmission,
    DrugGet, DrugHeader, DrugIndications, DrugInteractions, DrugLabel, DrugLabelText, DrugRecords,
    DrugRegulatory, DrugRegulatoryContext, DrugTargets, DrugXrefs,
};
use crate::{CacheProblem, PageInfo, Problem, ResponseBody, Section, SourceOutcome, Status, Step};
use biodata::{
    AuthorizationEvent, DrugIdentityRelation, DrugIndicationClaim, DrugInteractionClaim,
    DrugTargetRelation, Text,
};
use serde::{Deserialize, Serialize};

/// Several sections of one drug record in one reply: the drug once, then one part per section
/// and label heading.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugSections {
    /// The drug every part describes, with its name lookup; absent when no part found it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DrugHeader>,
    /// Parts in the caller's order.
    pub parts: Box<[DrugSectionPart]>,
}

/// One requested section, named by `section`, and what a call for it alone answers in `reply`,
/// less the drug identity the reply states once.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "section", content = "reply", rename_all = "kebab-case")]
pub enum DrugSectionPart {
    /// Cross-references.
    Xrefs(Box<DrugSectionReply<DrugXrefsPart>>),
    /// Targets.
    Targets(Box<DrugSectionReply<DrugTargetsPart>>),
    /// Indications.
    Indications(Box<DrugSectionReply<DrugIndicationsPart>>),
    /// CIViC context.
    Civic(Box<DrugSectionReply<DrugCivicPart>>),
    /// One label heading.
    Label(Box<DrugSectionReply<DrugLabelPart>>),
    /// Approvals.
    Approvals(Box<DrugSectionReply<DrugApprovalsPart>>),
    /// Regulator events.
    Regulatory(Box<DrugSectionReply<DrugRegulatoryPart>>),
    /// Interactions.
    Interactions(Box<DrugSectionReply<DrugInteractionsPart>>),
}

/// One part's own outcome, paging and next calls.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(bound(deserialize = "B: Deserialize<'de>"))]
#[cfg_attr(feature = "schema", schemars(rename = "DrugSectionReply_{B}"))]
pub struct DrugSectionReply<B> {
    /// This part's outcome.
    pub status: Status,
    /// Bounded local cache failure, independent of provider outcomes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_error: Option<CacheProblem>,
    /// The drug this part describes, only when it differs from the reply's identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<DrugHeader>,
    /// Answer when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<B>,
    /// Section page metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<PageInfo>,
    /// Sources actually asked.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceOutcome>,
    /// Rejected source fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problems: Vec<Problem>,
    /// Available next commands for this part.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<Step>,
}

/// Cross-references without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugXrefsPart {
    /// Native links with an exact link count, original version and retrieval date.
    pub xrefs: Section<DrugRecords<DrugIdentityRelation>>,
}
/// Targets without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugTargetsPart {
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub targets: Section<DrugRecords<DrugTargetRelation>>,
}
/// Indications without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugIndicationsPart {
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub indications: Section<DrugRecords<DrugIndicationClaim>>,
    /// Independent FDA application events found by the source's drug-name lookup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regulatory_context: Option<DrugRegulatoryContext>,
}
/// CIViC context without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugCivicPart {
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub civic: Section<DrugCivicStreams>,
}
/// One label heading without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugLabelPart {
    /// The requested label heading, as given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_section: Option<Text>,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub label: Section<DrugLabelText>,
    /// Label section names present for this record, listed when no label_section was given.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub sections: Box<[Text]>,
    /// How to read one of those sections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<Text>,
}
/// Approvals without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugApprovalsPart {
    /// Native products, filtered only by an explicit product number; independent of submissions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub products: Box<[DrugFdaProduct]>,
    /// Native FDA dataset last_updated date, independent of submission dates and retrieval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_updated: Option<crate::SourceUpdated>,
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub approvals: Section<DrugRecords<DrugFdaSubmission>>,
    /// Presentation order of application submissions, without interpreting their statuses.
    pub order: Box<str>,
}
/// Regulator events without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugRegulatoryPart {
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub regulatory: Section<DrugRecords<AuthorizationEvent>>,
    /// The source's own published description of each date role in these records.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub date_roles: Box<[DateRoleLabel]>,
}
/// Interactions without the identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugInteractionsPart {
    /// Existing typed section state, preserving empty, unavailable and skipped outcomes.
    pub interactions: Section<DrugRecords<DrugInteractionClaim>>,
}

/// A section reply body that splits into the drug identity and the rest.
pub trait DrugSectionBody {
    /// The body without its identity.
    type Part;
    /// Split the identity from the rest; `query` names the label heading for label parts.
    fn split(self, query: &DrugGet) -> (DrugHeader, Self::Part);
}
impl DrugSectionBody for DrugXrefs {
    type Part = DrugXrefsPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (self.identity, DrugXrefsPart { xrefs: self.xrefs })
    }
}
impl DrugSectionBody for DrugTargets {
    type Part = DrugTargetsPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (
            self.identity,
            DrugTargetsPart {
                targets: self.targets,
            },
        )
    }
}
impl DrugSectionBody for DrugIndications {
    type Part = DrugIndicationsPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (
            self.identity,
            DrugIndicationsPart {
                indications: self.indications,
                regulatory_context: self.regulatory_context,
            },
        )
    }
}
impl DrugSectionBody for DrugCivic {
    type Part = DrugCivicPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (self.identity, DrugCivicPart { civic: self.civic })
    }
}
impl DrugSectionBody for DrugLabel {
    type Part = DrugLabelPart;
    fn split(self, query: &DrugGet) -> (DrugHeader, Self::Part) {
        (
            self.identity,
            DrugLabelPart {
                label_section: query.label_section().cloned(),
                label: self.label,
                sections: self.sections,
                hint: self.hint,
            },
        )
    }
}
impl DrugSectionBody for DrugApprovals {
    type Part = DrugApprovalsPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (
            self.identity,
            DrugApprovalsPart {
                products: self.products,
                source_updated: self.source_updated,
                approvals: self.approvals,
                order: self.order,
            },
        )
    }
}
impl DrugSectionBody for DrugRegulatory {
    type Part = DrugRegulatoryPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (
            self.identity,
            DrugRegulatoryPart {
                regulatory: self.regulatory,
                date_roles: self.date_roles,
            },
        )
    }
}
impl DrugSectionBody for DrugInteractions {
    type Part = DrugInteractionsPart;
    fn split(self, _: &DrugGet) -> (DrugHeader, Self::Part) {
        (
            self.identity,
            DrugInteractionsPart {
                interactions: self.interactions,
            },
        )
    }
}

impl ResponseBody for DrugSections {
    const SCHEMA: &'static str = "biomcp.drug-sections/1";
    type Query = DrugGet;
}
