use crate::{Error, Text};
use serde::{Deserialize, Serialize};
/// Nonempty source Markdown retained as record data.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Markdown(Box<str>);
impl Markdown {
    /// Check the same nonempty boundary as ordinary text.
    pub fn new(value: &str) -> Result<Self, Error> {
        Text::new(value)?;
        Ok(Self(value.into()))
    }
    /// Borrow the source Markdown.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Markdown {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Markdown".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = <Text as schemars::JsonSchema>::json_schema(generator);
        schema.insert("format".into(), "markdown".into());
        schema
    }
}

impl TryFrom<String> for Markdown {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        Self::new(&value)
    }
}
