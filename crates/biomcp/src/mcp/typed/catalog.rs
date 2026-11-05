//! Advertised capabilities are projections of command and execution policy.
#[path = "../../../typed_front_map.rs"]
mod front_map;
use biomcp_types::Source;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

pub(super) fn excluded(context: crate::command::Context<'_>) -> io::Result<BTreeSet<Source>> {
    let mut sources = crate::config::excluded_sources(context.globals, context.settings)
        .map_err(io::Error::other)?;
    sources.extend(crate::command::hosted_use::disabled(
        context.declared_use,
        context.secrets,
    ));
    Ok(sources)
}

pub(super) fn schema(
    verb: &str,
    entity: &str,
    mut schema: Value,
    excluded: &BTreeSet<Source>,
) -> Value {
    let allowed = crate::help::coverage::command_sources(verb, entity, &schema)
        .into_iter()
        .filter(|word| {
            word.parse::<Source>()
                .is_ok_and(|source| !excluded.contains(&source))
        })
        .collect::<Vec<_>>();
    filter(&mut schema, excluded);
    for key in ["source", "sources", "resolver"] {
        if let Some(node) = schema.pointer_mut(&format!("/properties/{key}"))
            && let Some(fields) = node.as_object_mut()
        {
            fields.insert(
                "description".into(),
                json!(
                    format!("Source: {}", allowed.join(","))
                        .chars()
                        .take(80)
                        .collect::<String>()
                ),
            );
        }
    }
    schema
}

fn filter(node: &mut Value, excluded: &BTreeSet<Source>) {
    match node {
        Value::Object(fields) => {
            if let Some(text) = fields.get("description").and_then(Value::as_str)
                && excluded
                    .iter()
                    .any(|source| text.to_lowercase().contains(&source.to_string()))
            {
                fields.insert(
                    "description".into(),
                    json!("See biomcp_help for this option."),
                );
            }
            for key in ["enum", "x-extensible-enum"] {
                if let Some(values) = fields.get_mut(key).and_then(Value::as_array_mut) {
                    values.retain(|value| {
                        value
                            .as_str()
                            .and_then(|word| word.parse::<Source>().ok())
                            .is_none_or(|source| !excluded.contains(&source))
                    });
                }
            }
            for value in fields.values_mut() {
                filter(value, excluded);
            }
        }
        Value::Array(values) => {
            for value in values {
                filter(value, excluded);
            }
        }
        _ => {}
    }
}

pub(in crate::mcp) fn instructions(context: crate::command::Context<'_>) -> io::Result<String> {
    let excluded = excluded(context)?;
    let mut commands = Vec::new();
    let mut schemas = BTreeMap::new();
    for &(verb, entity, id) in crate::grammar::command_paths() {
        let query = super::schema(id)?;
        if crate::command::mcp_policy::always_refused(&query, context.globals.hosted) {
            continue;
        }
        let query = schema(verb, entity, query, &excluded);
        schemas.insert(id.into(), query.to_string());
        commands.push((verb, entity, id));
    }
    front_map::generate(
        &commands,
        &schemas,
        &excluded.iter().map(ToString::to_string).collect(),
    )
    .map_err(|error| io::Error::other(error.to_string()))
}
