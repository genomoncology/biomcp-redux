//! NIH RePORTER project-years as RePORTER publishes them.
use super::ResponseBody;
use crate::GrantSearch;
use biodata::Text;
use serde::{Deserialize, Serialize};

#[doc = " NIH RePORTER project-years in RePORTER's order. Each row is one project's award for one \
    fiscal year; `page.total` counts project-years, not distinct projects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrantPage {
    /// Project-years on this page.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[Grant]>,
}

/// One NIH project's award for one fiscal year.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Grant {
    /// Full NIH project number for this year, such as 5P50CA265826-04.
    pub project: Text,
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

/// A principal investigator on a project-year.
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

impl ResponseBody for GrantPage {
    const SCHEMA: &'static str = "biomcp.grant-page/1";
    type Query = GrantSearch;
}
