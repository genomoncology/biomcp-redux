use crate::{PathwayEventsQuery, PathwayGenesQuery, PathwayGet, PathwaySearch, PathwaySection, PoolInfo, ResponseBody};
use biodata::{Pathway, PathwayEventRef, PathwayId, PathwayMember, PathwaySpecies, Text};
use serde::{Deserialize, Serialize};
/// Small selectable native pathway identity, without its potentially long description.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayHeader {
    /// Checked source-owned identity.
    pub id: PathwayId,
    /// Whole native name.
    pub name: Text,
    /// Explicit native human species.
    pub species: PathwaySpecies,
    /// Reactome stable identifier version, separate from identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// WikiPathways revision, separate from identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<Text>,
}
biodata::wire_enum! {
    /// Meaning of the collection total, independent of admitted rows on a page.
    PathwayTotalKind {
        #[doc = "Native source-reported total before exact page admission."] Source => "source",
        #[doc = "Number of observed members in the retained relation."] Observed => "observed"
    }
}
/// Native page position and explicit count provenance; a short admitted page can continue.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayPageInfo {
    /// Native source or retained relation offset.
    pub offset: u32,
    /// Whole admitted rows on this page.
    pub returned: u32,
    /// Source-reported or observed relation total, as qualified by total_kind.
    pub total: u64,
    /// Whether the total comes from the source or this retained collection.
    pub total_kind: PathwayTotalKind,
    /// Next native position, independent of returned rows; absent only at the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<u32>,
    /// Snapshot identity and lifetime when retained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<PoolInfo>,
}
/// Source-ordered header page; class admission never changes the native total.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayPage {
    /// Whole admitted pathway headers in native order.
    pub items: Box<[PathwayHeader]>,
    /// Native position and total provenance.
    pub page: PathwayPageInfo,
}
impl ResponseBody for PathwayPage {
    const SCHEMA: &'static str = "biomcp.pathway_page/1";
    type Query = PathwaySearch;
}
/// One checked source pathway with supported disclosures and whole native text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayCard {
    /// Source-native identity, species, versions and requested description.
    pub pathway: Pathway,
    /// Whether the source itself shortened its description, rather than the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_truncated: Option<bool>,
    /// Source-supported named sections; WikiPathways excludes genes and events.
    pub sections: Box<[PathwaySection]>,
}
impl ResponseBody for PathwayCard {
    const SCHEMA: &'static str = "biomcp.pathway_card/1";
    type Query = PathwayGet;
}
macro_rules! relation_page {
    ($name:ident, $row:ty, $query:ty, $schema:literal) => {
        /// Independent source-ordered page of whole native references.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        pub struct $name {
            /// Exact source pathway owning this relation.
            pub header: PathwayHeader,
            /// Whole native references; unresolved identities remain visible.
            pub items: Box<[$row]>,
            /// Relation-specific count provenance and continuation.
            pub page: PathwayPageInfo,
        }
        impl ResponseBody for $name {
            const SCHEMA: &'static str = $schema;
            type Query = $query;
        }
    };
}
relation_page!(PathwayGenes, PathwayMember, PathwayGenesQuery, "biomcp.pathway_genes/1");
relation_page!(PathwayEvents, PathwayEventRef, PathwayEventsQuery, "biomcp.pathway_events/1");
/// Receipt for one independently validated WikiPathways metadata catalog.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WikiPathwaysCatalogReceipt {
    /// Hash of this catalog's retained native bytes.
    pub sha256: Text,
    /// Bytes in this catalog, not the other catalog or a GPML archive.
    pub bytes: u64,
    /// This catalog's retained revision.
    pub revision: Text,
    /// Source-stated human rows in this catalog.
    pub human_rows: u64,
    /// All native rows in this catalog before human filtering.
    pub total_rows: u64,
}
/// Two independently validated metadata catalogs; no catalog body in stdout.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WikiPathwaysSync {
    /// Search metadata catalog receipt.
    pub search: WikiPathwaysCatalogReceipt,
    /// Exact-get metadata catalog receipt.
    pub get: WikiPathwaysCatalogReceipt,
}
