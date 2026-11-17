//! NIH RePORTER project search by topic, investigator, organization and fiscal year.
use crate::QueryInvalid;
use biodata::Text;
use serde::{Deserialize, Serialize};

#[doc = " Search NIH-funded research projects in NIH RePORTER by topic, investigator, organization \
    and fiscal year. Each row is one project's award for one fiscal year, so a project funded for \
    five years gives five rows."]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct GrantSearch(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Words that must all appear in the project title, abstract or terms, such as "KRAS G12C".
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text", extend("x-positional" = 0)))]
    query: Option<Text>,
    #[doc = " Principal investigator, first and last name, such as \"Pasi Janne\"; partial names \
        match more people."]
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    investigator: Option<Text>,
    /// Funded organization name or part of it, as RePORTER spells it, such as "DANA-FARBER".
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    organization: Option<Text>,
    /// NIH fiscal years; repeat the option for several, such as 2024 and 2025. All by default.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(length(max = 50), extend("x-comma-separated" = true)))]
    fiscal_year: Box<[u32]>,
    /// Ten project-years by default, one to fifty.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Project-years to skip in RePORTER's order; RePORTER pages at most 15,000 deep.
    #[serde(default)]
    #[cfg_attr(feature = "schema", schemars(range(max = 14999)))]
    offset: u32,
}
fn default_limit() -> u32 {
    10
}
crate::query::validation::checked_query!(GrantSearch, Fields);
impl TryFrom<Fields> for GrantSearch {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if v.query.is_none() && v.investigator.is_none() && v.organization.is_none() {
            return Err(QueryInvalid::new(&[
                "query",
                "investigator",
                "organization",
            ]));
        }
        if v.fiscal_year.len() > 50
            || v.fiscal_year
                .iter()
                .any(|year| !(1985..=2100).contains(year))
        {
            return Err(QueryInvalid::new(&["fiscal_year"]));
        }
        if !(1..=50).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if v.offset > 14_999 {
            return Err(QueryInvalid::new(&["offset"]));
        }
        Ok(Self(v))
    }
}
impl GrantSearch {
    /// Words matched in title, abstract and terms.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Principal investigator name.
    pub fn investigator(&self) -> Option<&Text> {
        self.0.investigator.as_ref()
    }
    /// Funded organization name.
    pub fn organization(&self) -> Option<&Text> {
        self.0.organization.as_ref()
    }
    /// NIH fiscal years; empty means all.
    pub fn fiscal_year(&self) -> &[u32] {
        &self.0.fiscal_year
    }
    /// Project-years per page.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Project-years skipped.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// The same search at another offset.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut query = self.clone();
        query.0.offset = offset;
        query
    }
}
