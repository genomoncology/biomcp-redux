//! NIH RePORTER award rows as RePORTER publishes them.
use super::ResponseBody;
use crate::GrantSearch;
use biodata::Text;
use serde::{Deserialize, Serialize};

#[doc = " NIH RePORTER award rows in RePORTER's order: one per award and fiscal year, plus one per \
    component of a multi-part award. `page.total` counts these rows, not distinct projects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrantPage {
    /// How to read the rows: what the total counts, how to add amounts and how names match.
    pub hint: Box<str>,
    /// Award rows on this page.
    #[serde(default)]
    pub items: Box<[Grant]>,
}

/// One NIH award, or one component of a multi-part award, for one fiscal year.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Grant {
    /// Full NIH project number for this year, such as 5P50CA265826-04; a parent and its
    /// components share it.
    pub project: Text,
    /// Component ID within a multi-part award; absent on the parent and on single awards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subproject: Option<Text>,
    /// Core project number shared by every year of the project, such as P50CA265826.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core_project: Option<Text>,
    /// NIH fiscal year of this award.
    pub fiscal_year: u32,
    /// Project title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    /// NIH activity code, such as R01, R35 or P50.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<Text>,
    /// Administering NIH institute or center.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institute: Option<GrantInstitute>,
    /// Funded organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<GrantOrganization>,
    /// This fiscal year's total cost in US dollars, direct plus indirect, as RePORTER states it.
    /// A parent row's amount already includes its components' amounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_amount: Option<u64>,
    /// Project start date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<Text>,
    /// Project end date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Text>,
    /// Principal investigators as RePORTER names them.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub investigators: Box<[GrantInvestigator]>,
    /// RePORTER project details page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Text>,
}

/// An NIH institute or center.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrantInstitute {
    /// Abbreviation, such as NCI.
    pub abbreviation: Text,
    /// Full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
}

/// A funded organization as RePORTER spells it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrantOrganization {
    /// Organization name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Text>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<Text>,
    /// State.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Text>,
    /// Country.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<Text>,
}

/// A principal investigator on an award row.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrantInvestigator {
    /// Name as RePORTER gives it.
    pub name: Text,
    /// The contact principal investigator.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub contact: bool,
    /// RePORTER person profile ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<u64>,
}

/// The hint every grant page carries.
pub const GRANT_HINT: &str = "page.total counts award rows: one per award and fiscal year, plus \
    one per component of a multi-part award (rows with subproject). A parent's award_amount \
    includes its components, so do not add both. Each investigator name word matches any part \
    of any investigator's name; check investigators[].profile_id and search again with \
    investigator_id (--investigator-id on the command line).";

impl ResponseBody for GrantPage {
    const SCHEMA: &'static str = "biomcp.grant-page/1";
    type Query = GrantSearch;
}
