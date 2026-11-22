//! FDA Purple Book rows keep the agency's designation at presentation level.
use serde::{Deserialize, Serialize};

biodata::canonical_enum! {
    /// FDA's published 351(k) license type for this presentation.
    PurpleBookLicense, SourceUnknown {
        /// FDA lists the presentation as biosimilar.
        Biosimilar => "351(k) Biosimilar",
        /// FDA lists the presentation as interchangeable.
        Interchangeable => "351(k) Interchangeable",
    }
}

/// One product presentation in a dated FDA Purple Book release.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PurpleBookPresentation {
    /// The biosimilar application's BLA number.
    pub bla: biodata::DrugId,
    /// FDA product number, distinguishing presentations within a BLA.
    pub product_number: Box<str>,
    /// The FDA proprietary name.
    pub name: Box<str>,
    /// The FDA proper name.
    pub proper_name: Box<str>,
    /// Applicant on this FDA presentation row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicant: Option<Box<str>>,
    /// The FDA reference product's proprietary name.
    pub reference_name: Box<str>,
    /// The FDA reference product's proper name.
    pub reference_proper_name: Box<str>,
    /// BLA of the unique 351(a) row family with the reported reference name pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_bla: Option<biodata::DrugId>,
    /// BLA candidates when the FDA's reference names occur under multiple applications.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub reference_bla_candidates: Box<[biodata::DrugId]>,
    /// FDA strength cell, including units.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<Box<str>>,
    /// FDA product presentation, such as a pre-filled syringe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<str>>,
    /// FDA route of administration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<str>>,
    /// FDA's own license type for this presentation.
    pub license_type: PurpleBookLicense,
    /// Whether this row's FDA license type is Interchangeable.
    pub interchangeable: bool,
    /// FDA approval date as printed in the CSV.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<Box<str>>,
    /// FDA interchangeability approval date as printed, if published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interchangeability_date: Option<Box<str>>,
    /// FDA first interchangeable exclusivity expiration date as printed, if published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_interchangeable_exclusivity_expiration: Option<Box<str>>,
}

/// Paged presentations from one release, including the date on a zero-match search.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugPurpleBookPage {
    /// FDA release month, not the download or import date.
    pub release_month: Box<str>,
    /// Number of matching presentation rows.
    pub total: u64,
    /// FDA rows in BLA and product number order.
    #[serde(default)]
    pub items: Box<[PurpleBookPresentation]>,
}

impl crate::ResponseBody for DrugPurpleBookPage {
    const SCHEMA: &'static str = "biomcp.drug-purple-book-page/1";
    type Query = super::DrugSearch;
}
