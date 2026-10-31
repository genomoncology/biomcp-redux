//! Recover grammar-owned argument identities without copying rejected values.
use clap::{
    Command,
    error::{ContextKind, ContextValue},
};
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

/// Preserve parser-owned identities and command guidance from the first parse.
pub(super) fn parse_invalid(command: &Command, error: &clap::Error) -> super::ParseInvalid {
    let usage = error
        .get(ContextKind::Usage)
        .map(ToString::to_string)
        .unwrap_or_default();
    let message = if usage.contains("biomcp search gwas") {
        Some(biomcp_types::ErrorMessage::GwasSearchArguments)
    } else if usage.contains("biomcp get gwas") {
        Some(biomcp_types::ErrorMessage::GwasGetArguments)
    } else {
        None
    };
    let argument = error.get(ContextKind::InvalidArg).and_then(|context| {
        let spellings = match context {
            ContextValue::String(value) => vec![value.as_str()],
            ContextValue::Strings(values) => values.iter().map(String::as_str).collect(),
            _ => return None,
        };
        identity(command, &spellings)
    });
    super::ParseInvalid {
        argument,
        usage: message.map(biomcp_types::UsageError::message),
    }
}

/// Carry checked constructor guidance without reading caller arguments again.
pub(super) fn checked_invalid(
    verb: &str,
    entity: &str,
    matches: &clap::ArgMatches,
    cause: &biomcp_types::QueryInvalid,
) -> super::ParseInvalid {
    let normalization = (verb, entity) == ("variant", "normalize")
        && cause
            .arguments()
            .iter()
            .any(|name| name.as_ref() == "build");
    let usage = if normalization {
        normalization_build()
    } else {
        cause.message().map(biomcp_types::UsageError::message)
    };
    super::ParseInvalid {
        argument: if normalization && usage.is_some() {
            Some("--build".into())
        } else {
            super::validation::argument_names(verb, entity, matches, cause)
        },
        usage,
    }
}

fn normalization_build() -> Option<biomcp_types::UsageError> {
    let schema: serde_json::Value =
        serde_json::from_str(crate::schema::find("biomcp.variant-normalize/5")?).ok()?;
    let values = schema
        .pointer("/$defs/Build/enum")?
        .as_array()?
        .iter()
        .map(|value| biodata::Text::new(value.as_str()?).ok())
        .collect::<Option<Vec<_>>>()?;
    Some(biomcp_types::UsageError::argument_values(
        "--build".into(),
        values.into(),
    ))
}
