use super::{PathwayId, QueryInvalid, Text, default_limit, native_id, reactome, valid_limit};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct RelationFields {
    /// Checked human Reactome identity; WikiPathways relations are not yet available.
    #[serde(with = "native_id")]
    #[cfg_attr(feature = "schema", schemars(with = "str", regex(pattern = "^R-HSA-[1-9][0-9]*$"), extend("x-positional" = 0)))]
    id: PathwayId,
    /// Five complete relation rows by default, at most fifty.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Offset within the retained selected relation.
    #[serde(default)]
    offset: u32,
    /// Immutable relation snapshot.
    #[serde(default, deserialize_with = "super::super::gene::present", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
}
macro_rules! relation {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(try_from = "RelationFields")]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(with = "RelationFields"))]
        pub struct $name(RelationFields);
        impl TryFrom<RelationFields> for $name {
            type Error = QueryInvalid;
            fn try_from(v: RelationFields) -> Result<Self, Self::Error> {
                if !valid_limit(v.limit) || !reactome(&v.id) { return Err(QueryInvalid); }
                Ok(Self(v))
            }
        }
        impl $name {
            /// Checked source identity owning this relation.
            pub fn id(&self) -> &PathwayId { &self.0.id }
            /// Complete relation rows per page.
            pub fn limit(&self) -> u32 { self.0.limit }
            /// Relation offset, independent of other sections.
            pub fn offset(&self) -> u32 { self.0.offset }
            /// Immutable relation snapshot.
            pub fn pool(&self) -> Option<&Text> { self.0.pool.as_ref() }
            /// Continue without changing relation identity or page size.
            pub fn at_offset(&self, offset: u32) -> Self {
                let mut v = self.clone(); v.0.offset = offset; v
            }
        }
    };
}
relation!(PathwayGenesQuery, "Continue native namespaced pathway membership, without name-based gene extraction.");
relation!(PathwayEventsQuery, "Continue Reactome event objects and unresolved native integer references.");
