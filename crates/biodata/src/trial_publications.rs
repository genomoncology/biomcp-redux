//! Registry publication and posted-document metadata, without interpretation.
use crate::{License, Pmid, Text};
use serde::{Deserialize, Serialize};
/// An unchanged registry reference in source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialReference {
    /// Registry-supplied PubMed identity, when present.
    pub pmid: Option<Pmid>,
    /// Native reference classification, such as RESULT or BACKGROUND.
    pub kind: Option<Box<str>>,
    /// Original citation text.
    pub citation: Option<Box<str>>,
}
/// A posted source document; a URL never comes from caller-supplied filename text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocument {
    /// Exact advertised portable source filename.
    pub filename: Text,
    /// Fixed registry CDN URL for this manifest entry.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
    /// Native document type abbreviation.
    pub kind: Option<Box<str>>,
    /// Source label.
    pub label: Option<Box<str>>,
    /// Source document date.
    pub date: Option<crate::PartialDate>,
    /// Original upload timestamp, without guessing a timezone.
    pub uploaded: Option<Box<str>>,
    /// Source byte count, when supplied.
    pub size: Option<u64>,
    /// Source says this contains a protocol.
    pub has_protocol: Option<bool>,
    /// Source says this contains a statistical analysis plan.
    pub has_sap: Option<bool>,
    /// Source says this contains an informed consent form.
    pub has_icf: Option<bool>,
    /// No blanket reuse license is inferred from registry availability.
    pub license: License,
}
