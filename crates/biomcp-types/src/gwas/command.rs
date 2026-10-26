//! CLI grammar derived from the standalone checked association search.
/// Search command grammar omits the fixed source order.
#[cfg(feature = "schema")]
#[derive(Debug)]
pub struct GwasSearchCommand;
#[cfg(feature = "schema")]
impl schemars::JsonSchema for GwasSearchCommand {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GwasSearchCommand".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = super::GwasSearch::json_schema(generator);
        if let Some(fields) = schema.get_mut("properties").and_then(|v| v.as_object_mut()) {
            fields.remove("order");
            if let Some(gene) = fields.get("gene").cloned() {
                let mut query = gene;
                if let Some(object) = query.as_object_mut() {
                    object.insert("x-positional".into(), 0.into());
                }
                fields.insert("query".into(), query);
            }
        }
        schema
    }
}

/// Get command grammar selects an association or study by its checked native identifier.
#[cfg(feature = "schema")]
#[derive(Debug, schemars::JsonSchema)]
pub struct GwasGetCommand {
    /// Qualified association ID or exact GCST study accession.
    #[schemars(extend("x-positional" = 0))]
    pub id: biodata::Text,
    /// Association loci or study associations and ancestries.
    #[schemars(with = "GwasCommandSection", extend("x-positional" = 1))]
    pub section: Option<GwasCommandSection>,
    /// Maximum native relation rows.
    #[schemars(with = "u32", range(min = 1, max = 50))]
    pub section_limit: Option<u32>,
    /// Native relation row offset.
    #[schemars(with = "u32")]
    pub section_offset: Option<u32>,
    /// Retained ancestry reply for a local page.
    #[schemars(with = "biodata::Text")]
    pub snapshot: Option<biodata::Text>,
}
biodata::wire_enum! {
    described
    /// Native context section grammar.
    GwasCommandSection {
        /// Native association loci.
        Loci => "loci",
        /// Native study association pages.
        Associations => "associations",
        /// Native study ancestry rows.
        Ancestries => "ancestries"
    }
}
