//! Fixed refusal prose and the checked message that names an unknown option.
use super::ErrorMessage;
use serde::{Deserialize, Serialize};

const PREFIX: &str = "Unknown option ";
const SUFFIX: &str = ". Use an option listed in valid_values or follow the suggested call.";
const EMPTY: &str = "\"\"";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum WireMessage {
    Fixed(ErrorMessage),
    NamedOption(NamedOption),
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
        let mut text = String::from(PREFIX);
        for character in option.chars() {
            if option_char(character) {
                text.push(character);
            } else {
                text.extend(character.escape_unicode());
            }
        }
        if option.is_empty() {
            text.push_str(EMPTY);
        }
        text.push_str(SUFFIX);
        Self::NamedOption(NamedOption(text))
    }
}

fn option_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct NamedOption(String);

impl<'de> Deserialize<'de> for NamedOption {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        let valid = text
            .strip_prefix(PREFIX)
            .and_then(|text| text.strip_suffix(SUFFIX))
            .is_some_and(valid_option);
        valid
            .then_some(Self(text))
            .ok_or_else(|| serde::de::Error::custom("invalid unknown-option message"))
    }
}

fn valid_option(name: &str) -> bool {
    if name == EMPTY {
        return true;
    }
    let mut parts = name.split(r"\u{");
    !name.is_empty()
        && parts
            .next()
            .is_some_and(|plain| plain.chars().all(option_char))
        && parts.all(|part| {
            part.split_once('}').is_some_and(|(hex, plain)| {
                (1..=6).contains(&hex.len())
                    && hex
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                    && plain.chars().all(option_char)
            })
        })
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for WireMessage {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        <ErrorMessage as schemars::JsonSchema>::schema_name()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = <ErrorMessage as schemars::JsonSchema>::json_schema(generator);
        let mut pattern = String::from("^(?:");
        if let Some(values) = schema.ensure_object().remove("enum")
            && let Some(values) = values.as_array()
        {
            for literal in values.iter().filter_map(|value| value.as_str()) {
                escape_pattern(&mut pattern, literal);
                pattern.push('|');
            }
        }
        escape_pattern(&mut pattern, PREFIX);
        pattern.push_str(r"(?:(?:[A-Za-z0-9_.-]|\\u\{[0-9a-f]{1,6}\})+|");
        escape_pattern(&mut pattern, EMPTY);
        pattern.push(')');
        escape_pattern(&mut pattern, SUFFIX);
        pattern.push_str(")$");
        schema.insert("pattern".into(), pattern.into());
        schema.insert(
            "description".into(),
            "Error guidance. An unknown-option message can include the rejected option name."
                .into(),
        );
        schema
    }
}

#[cfg(feature = "schema")]
fn escape_pattern(pattern: &mut String, literal: &str) {
    for character in literal.chars() {
        if r"\^$.*+?()[]{}|".contains(character) {
            pattern.push('\\');
        }
        pattern.push(character);
    }
}
