//! Refusal prose can include the caller's rejected option name.
use super::ErrorMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum WireMessage {
    Fixed(ErrorMessage),
    NamedOption(String),
}

impl From<ErrorMessage> for WireMessage {
    fn from(message: ErrorMessage) -> Self {
        Self::Fixed(message)
    }
}

impl WireMessage {
    pub(super) fn for_argument(message: ErrorMessage, option: Option<&str>) -> Self {
        let Some(option) = option.filter(|_| message == ErrorMessage::ArgumentUnknown) else {
            return message.into();
        };
        let mut name = String::with_capacity(option.len());
        for character in option.chars() {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                name.push(character);
            } else {
                name.extend(character.escape_unicode());
            }
        }
        if name.is_empty() {
            name.push_str("\"\"");
        }
        Self::NamedOption(format!(
            "Unknown option {name}. Use an option listed in valid_values or follow the suggested call."
        ))
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for WireMessage {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        <ErrorMessage as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = <ErrorMessage as schemars::JsonSchema>::json_schema(generator);
        if let Some(values) = schema.ensure_object().remove("enum") {
            schema.insert("x-extensible-enum".into(), values);
        }
        schema.insert(
            "description".into(),
            "Error guidance. An unknown-option message can include the rejected option name."
                .into(),
        );
        schema
    }
}
