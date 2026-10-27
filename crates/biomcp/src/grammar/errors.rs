//! Recover grammar-owned argument identities without copying rejected values.
use clap::{
    Command,
    error::{ContextKind, ContextValue},
};
use std::ffi::OsString;
pub(crate) fn argument_error(args: &[OsString]) -> Option<String> {
    let mut parser = super::root().ok()?;
    parser.build();
    let error = parser
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .err()?;
    let context = error.get(ContextKind::InvalidArg)?;
    let spellings = match context {
        ContextValue::String(value) => vec![value.as_str()],
        ContextValue::Strings(values) => values.iter().map(String::as_str).collect(),
        _ => return None,
    };
    identity(&parser, &spellings)
}
fn identity(command: &Command, spellings: &[&str]) -> Option<String> {
    command
        .get_arguments()
        .find_map(|argument| {
            let rendered = argument.to_string();
            spellings.contains(&rendered.as_str()).then(|| {
                argument
                    .get_long()
                    .map_or_else(|| argument.get_id().to_string(), |name| format!("--{name}"))
            })
        })
        .or_else(|| {
            command
                .get_subcommands()
                .find_map(|child| identity(child, spellings))
        })
}

/// Report the checked assembly vocabulary without retaining a rejected value.
pub(super) fn normalization_build(args: &[OsString]) -> Option<biomcp_types::ErrorEnvelope> {
    if args.first()?.to_str()? != "variant" || args.get(1)?.to_str()? != "normalize" {
        return None;
    }
    let matches = super::root()
        .ok()?
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .ok()?;
    let matches = matches
        .subcommand_matches("variant")?
        .subcommand_matches("normalize")?;
    let build = matches.get_one::<String>("build")?;
    if build.parse::<biomcp_types::Build>().is_ok_and(|build| {
        matches
            .get_one::<String>("hgvs")
            .is_none_or(|hgvs| biomcp_types::VariantNormalize::supports_build(hgvs, build))
    }) {
        return None;
    }
    let schema: serde_json::Value =
        serde_json::from_str(crate::schema::find("biodata.variant-normalize/3")?).ok()?;
    let values = schema
        .pointer("/$defs/Build/enum")?
        .as_array()?
        .iter()
        .map(|value| biomcp_types::Text::new(value.as_str()?).ok())
        .collect::<Option<Vec<_>>>()?;
    Some(biomcp_types::ErrorEnvelope::argument_values(
        "--build".into(),
        values.into(),
    ))
}
