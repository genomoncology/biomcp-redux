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
        schema.insert("description".into(), "Retrieve a drug by source ID, brand or generic name, with explicit source-scoped sections.".into());
        let who = super::DrugWhoGet::json_schema(generator);
        if let Some(properties) = schema.get_mut("properties").and_then(|v| v.as_object_mut())
            && let Some(input) = who.pointer("/properties/input")
        {
            let mut input = input.clone();
            if let Some(object) = input.as_object_mut() {
                object.insert(
                    "description".into(),
                    concat!(
                        "Use a source ID or full brand or generic name. Keep spaces in WHO references. ",
                        "Exact brands rank first. Drugs@FDA ranks NDA/BLA before ANDA, then earliest approval. ",
                        "EMA uses earliest authorisation. Others use search order. Replies show up to five alternatives and their total. ",
                        "Labels also take application numbers (e.g. NDA216340). ",
                        "fda-dsc requires the communication UUID from search drug NAME --source fda-dsc."
                    ).into(),
                );
            }
            properties.insert("input".into(), input);
        }
        if let Some(input) = schema
            .get_mut("properties")
            .and_then(|p| p.get_mut("input"))
        {
            let original = input["description"].as_str().unwrap_or_default();
            input["description"] = format!(
                concat!(
                    "{original} Generic names accept one trailing salt or ester term: {}. ",
                    "Brands identical to their ingredient count as generic. USAN: {}"
                ),
                super::DrugNameMatch::SALT_OR_ESTER_WORDS.join(", "),
                super::DrugNameMatch::NAMING_GUIDANCE,
                original = original
            )
            .into();
        }
        if let Some(source) = schema
            .get_mut("properties")
            .and_then(|p| p.get_mut("source"))
        {
            source["description"] = "Omit to infer the source from the section. Identity and mixed sections default to MyChem.".into();
        }
        if let Some(required) = schema.get_mut("required").and_then(|r| r.as_array_mut()) {
            required.retain(|name| name != "source");
        }
        schema.insert(
            "x-reply-query-titles".into(),
            vec![
                <super::DrugWhoGet as schemars::JsonSchema>::schema_name().into_owned(),
                <super::DrugMarketsGet as schemars::JsonSchema>::schema_name().into_owned(),
            ]
            .into(),
        );
        schema
    }
}
