//! Product summaries retain presentation-level differences and the reference identity evidence.
use super::PurpleBookLicense;
use biodata::DrugId;
use serde::{Deserialize, Serialize};

/// How many matching presentations have this FDA designation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PurpleBookDesignationCount {
    /// Published FDA designation.
    pub license_type: PurpleBookLicense,
    /// Presentations carrying this designation.
    pub count: u32,
}

/// One reference product named by FDA for the matching presentations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PurpleBookReference {
    /// FDA proprietary name of the reference product.
    pub name: Box<str>,
    /// FDA proper name of the reference product.
    pub proper_name: Box<str>,
    /// BLA when the FDA name pair identifies a single reference application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bla: Option<DrugId>,
    /// Candidate BLAs when multiple applications share the published names.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub bla_candidates: Box<[DrugId]>,
}

/// One biosimilar application with counts and dates from its matching presentations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PurpleBookProduct {
    /// Biosimilar application number.
    pub bla: DrugId,
    /// Proprietary name on the first matching FDA product number.
    pub name: Box<str>,
    /// Other proprietary names FDA lists under this application.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub other_names: Box<[Box<str>]>,
    /// Proper name on the first matching FDA product number.
    pub proper_name: Box<str>,
    /// Other proper names FDA lists under this application.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub other_proper_names: Box<[Box<str>]>,
    /// FDA applicant, when recorded in the active snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicant: Option<Box<str>>,
    /// Other applicants FDA lists on matching presentations.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub other_applicants: Box<[Box<str>]>,
    /// Reference names and application evidence for these presentations.
    pub references: Box<[PurpleBookReference]>,
    /// Number of matching presentations.
    pub presentation_count: u32,
    /// Count of presentations under each FDA designation.
    pub designations: Box<[PurpleBookDesignationCount]>,
    /// Earliest recognized FDA approval date among the matching presentations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earliest_approval_date: Option<Box<str>>,
    /// Distinct approval date cells when presentations differ or a cell cannot be ordered.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub approval_dates: Box<[Box<str>]>,
    /// Presentations with no approval date, when other presentations supply one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_date_unreported: Option<u32>,
    /// Earliest recognized FDA interchangeability approval date among the matching presentations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earliest_interchangeability_date: Option<Box<str>>,
    /// Distinct interchangeability date cells when presentations differ or a cell cannot be ordered.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub interchangeability_dates: Box<[Box<str>]>,
    /// Presentations with no interchangeability date, when other presentations supply one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interchangeability_date_unreported: Option<u32>,
    /// Distinct FDA first interchangeable exclusivity expiration dates, where published.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub first_interchangeable_exclusivity_expirations: Box<[Box<str>]>,
    /// Presentations without an expiration date, when other presentations supply one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_interchangeable_exclusivity_expiration_unreported: Option<u32>,
}

/// One complete roster of matching biosimilar applications from the active FDA release.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DrugPurpleBookProducts {
    /// FDA release month.
    pub release_month: Box<str>,
    /// Number of matching biosimilar applications.
    pub total: u64,
    /// Biosimilar products in BLA order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[PurpleBookProduct]>,
}

impl crate::ResponseBody for DrugPurpleBookProducts {
    const SCHEMA: &'static str = "biomcp.drug-purple-book-products/1";
    type Query = super::DrugSearch;
}
