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
        if message == ErrorMessage::ArgumentUnknown
            && let Some(option) = option.filter(|name| {
                !name.is_empty()
                    && name.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')
                    })
            })
        {
            return Self::NamedOption(format!(
                "Unknown option {option}. Use an option listed in valid_values or follow the suggested call."
            ));
        }
        message.into()
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
