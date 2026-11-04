//! Schema-only grammar union; each selected source validates its own checked request.
/// Existing exact-ID drug gets and native WHO references share one public command.
#[derive(Debug)]
pub struct DrugGetCommand;
impl schemars::JsonSchema for DrugGetCommand {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DrugGetCommand".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = super::DrugGet::json_schema(generator);
        let who = super::DrugWhoGet::json_schema(generator);
        if let Some(properties) = schema.get_mut("properties").and_then(|v| v.as_object_mut())
            && let Some(input) = who.pointer("/properties/input")
        {
            let mut input = input.clone();
            if let Some(object) = input.as_object_mut() {
                object.insert(
                    "description".into(),
                    "Source-native ID or complete brand or generic name; WHO references retain native spaces. Exact brands rank before generics; Drugs@FDA prefers NDA/BLA before ANDA, then earliest approval. Other sources retain search order.".into(),
                );
            }
            properties.insert("input".into(), input);
        }
        if let Some(required) = schema.get_mut("required").and_then(|r| r.as_array_mut()) {
            required.retain(|name| name != "source");
        }
        schema.insert(
            "x-reply-query-titles".into(),
            vec![<super::DrugWhoGet as schemars::JsonSchema>::schema_name().into_owned()].into(),
        );
        schema
    }
}
