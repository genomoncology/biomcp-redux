//! Schema discovery entries shipped with an executable.
use serde::Serialize;
/// Discoverable contracts shipped with this executable.
#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SchemaCatalog {
    /// Every artifact in canonical identity order.
    pub schemas: Vec<SchemaEntry>,
}
impl super::ResponseBody for SchemaCatalog {
    const SCHEMA: &'static str = "biomcp.schema-catalog/1";
    type Query = ();
}
/// One schema identity with its own artifact documentation.
#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SchemaEntry {
    /// Canonical versioned identity accepted by schema ID.
    pub id: String,
    /// One line of artifact documentation.
    pub description: String,
}
