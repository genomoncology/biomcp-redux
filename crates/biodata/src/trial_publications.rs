//! Registry publication and posted-document metadata, without interpretation.
use crate::{License, Pmid, Text};
use serde::{Deserialize, Serialize};
/// An unchanged registry reference in source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialReference {
    /// Registry-supplied PubMed identity, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmid: Option<Pmid>,
    /// Native reference classification, such as RESULT or BACKGROUND.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<str>>,
    /// Original citation text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation: Option<Box<str>>,
}
/// A posted source document; a URL never comes from caller-supplied filename text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocument {
    /// Exact advertised portable source filename.
    pub filename: Text,
    /// Fixed registry CDN URL for this manifest entry.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
    /// Native document type abbreviation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<str>>,
    /// Source label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,
    /// Source document date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::PartialDate>,
    /// Original upload timestamp, without guessing a timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<Box<str>>,
    /// Source byte count, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// Source says this contains a protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protocol: Option<bool>,
    /// Source says this contains a statistical analysis plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_sap: Option<bool>,
    /// Source says this contains an informed consent form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_icf: Option<bool>,
    /// No blanket reuse license is inferred from registry availability.
    pub license: License,
}
