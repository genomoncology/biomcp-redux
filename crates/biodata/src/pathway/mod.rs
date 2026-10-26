use crate::{HgncId, Text};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
mod ids;
pub use ids::{
    PathwayId, PathwayIdInvalid, PathwayRef, ReactomeId, ReactomeIdInvalid, WikiPathwaysId,
};
crate::wire_enum! {
    /// Source-stated human species; no species inference from a title.
    PathwaySpecies { #[doc = "Source states human species."] Human => "Homo sapiens" }
}
/// Neutral pathway facts, without display sections or source calls.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Pathway {
    /// Checked source-owned identity.
    pub id: PathwayId,
    /// Whole native pathway name.
    pub name: Text,
    /// Explicit source-stated human species.
    pub species: PathwaySpecies,
    /// Complete native description when present; empty is distinct from absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Reactome stable identifier version, separate from identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// WikiPathways native revision, separate from identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<Text>,
}
/// An open source namespace paired with its complete native identifier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwaySourceRef {
    /// Native namespace; never discarded during a gene join.
    pub namespace: Text,
    /// Complete namespace-local identifier, including native isoform suffixes.
    pub value: Text,
}
/// One source membership reference, without name-based gene inference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayMember {
    /// Namespaced native reference.
    pub reference: PathwaySourceRef,
    /// Native label, never interpreted as a gene symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// Exact resolved gene identifier, absent when no checked join exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gene: Option<HgncId>,
    /// Native reference schema class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_class: Option<Text>,
}
/// An event object or unresolved integer reference; never the parent's inferred ID.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PathwayEventRef {
    /// Native unresolved integer database reference.
    Database {
        /// Positive native database reference.
        db_id: NonZeroU64,
    },
    /// Native event object; fields are absent only when omitted by the source.
    Object {
        /// Native object database identity.
        db_id: NonZeroU64,
        /// Native stable identity when present, never copied from the pathway.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<ReactomeId>,
        /// Whole native display name.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<Text>,
        /// Native class; a reaction is not automatically a pathway.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        schema_class: Option<Text>,
    },
}
