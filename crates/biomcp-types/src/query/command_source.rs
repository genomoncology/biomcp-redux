//! Publish a standalone search selector without changing shared request or reply schemas.
use schemars::{JsonSchema, Schema, SchemaGenerator};
use std::{borrow::Cow, marker::PhantomData};

/// Keep a broader wire vocabulary distinct from the command's scoped selector.
pub(crate) struct SharedSource<T>(PhantomData<T>);

impl<T: JsonSchema> JsonSchema for SharedSource<T> {
    fn schema_name() -> Cow<'static, str> {
        format!("Shared{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("Shared{}", T::schema_id()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        T::json_schema(generator)
    }
}

pub(crate) fn scoped<T: JsonSchema>(
    generator: &mut SchemaGenerator,
    shared: &str,
    published: &str,
    sources: &[&str],
) -> Schema {
    let mut schema = T::json_schema(generator);
    // This command is published on its own. Keep its historical definition name for clients.
    generator.definitions_mut().remove(shared);
    generator.definitions_mut().insert(
        published.into(),
        schemars::json_schema!({"type":"string", "enum":sources}).to_value(),
    );
    if let Some(reference) = schema.pointer_mut("/properties/source/$ref") {
        *reference = format!("#/$defs/{published}").into();
    }
    schema
}
