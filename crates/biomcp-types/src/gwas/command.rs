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
