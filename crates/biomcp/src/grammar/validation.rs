//! Retain grammar-owned identities from checked input failures.
use biomcp_types::QueryInvalid;
use serde_json::{Map, Value};

pub(super) fn argument_names(
    verb: &str,
    entity: &str,
    matches: &clap::ArgMatches,
    cause: &QueryInvalid,
) -> Option<String> {
    let id = super::command_paths()
        .iter()
        .find(|(v, e, _)| *v == verb && *e == entity)?
        .2;
    let schema: Value = serde_json::from_str(crate::schema::find(id)?).ok()?;
    let properties = schema.get("properties")?.as_object()?;
    let names: Vec<_> = cause
        .arguments()
        .iter()
        .filter_map(|name| {
            if name.as_ref() == "query"
                && matches
                    .try_get_one::<String>("query_option")
                    .ok()
                    .flatten()
                    .is_some()
            {
                Some("--query".into())
            } else {
                identity(name, properties)
            }
        })
        .collect();
    (!names.is_empty()).then(|| names.join(", "))
}

fn identity(name: &str, properties: &Map<String, Value>) -> Option<String> {
    let field = properties.get(name)?;
    Some(if field.get("x-positional").is_some() {
        name.to_owned()
    } else {
        format!("--{}", name.replace('_', "-"))
    })
}
