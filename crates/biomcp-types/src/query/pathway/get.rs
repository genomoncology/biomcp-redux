use super::{PathwayId, PathwaySection, QueryInvalid, Text, default_limit, native_id, reactome, valid_limit};
use serde::{Deserialize, Serialize};
/// One exact pathway card or source-supported named disclosure.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct PathwayGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Exact human Reactome or WikiPathways identity without version suffixes.
    #[serde(with = "native_id")]
    #[cfg_attr(feature = "schema", schemars(with = "str", regex(pattern = "^(R-HSA-|WP)[1-9][0-9]*$"), extend("x-positional" = 0)))]
    id: PathwayId,
    /// Omit for the card; WikiPathways genes await the GPML implementation.
    #[serde(default, deserialize_with = "super::super::gene::present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "PathwaySection", extend("x-positional" = 1)))]
    section: Option<PathwaySection>,
    /// Bounded initial section size; each all section has its own first page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    section_limit: u32,
    /// Native relation offset for a selected genes or events section.
    #[serde(default)]
    section_offset: u32,
    /// Retained snapshot, when continuing a source relation.
    #[serde(default, deserialize_with = "super::super::gene::present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
    /// Request a new source observation.
    #[serde(default)]
    fresh: bool,
}
impl TryFrom<GetFields> for PathwayGet {
    type Error = QueryInvalid;
    fn try_from(v: GetFields) -> Result<Self, Self::Error> {
        let relation = matches!(v.section, Some(PathwaySection::Genes | PathwaySection::Events));
        if !valid_limit(v.section_limit) || (relation && !reactome(&v.id))
            || (!relation && v.section_offset != 0) {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl PathwayGet {
    /// Checked source identity.
    pub fn id(&self) -> &PathwayId { &self.0.id }
    /// Selected section or the default card.
    pub fn section(&self) -> Option<PathwaySection> { self.0.section }
    /// Initial section page size.
    pub fn section_limit(&self) -> u32 { self.0.section_limit }
    /// Selected relation offset.
    pub fn section_offset(&self) -> u32 { self.0.section_offset }
    /// Retained relation snapshot.
    pub fn pool(&self) -> Option<&Text> { self.0.pool.as_ref() }
    /// Whether to refresh the observation.
    pub fn fresh(&self) -> bool { self.0.fresh }
}
