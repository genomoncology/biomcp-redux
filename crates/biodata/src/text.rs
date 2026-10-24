use crate::Error;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Nonempty text with surrounding Unicode whitespace removed.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from, transparent))]
pub struct Text(#[cfg_attr(feature = "schema", schemars(length(min = 1)))] Box<str>);

impl Text {
    /// Trim surrounding whitespace and reject an empty result.
    pub fn new(value: &str) -> Result<Self, Error> {
        let value = value.trim();
        if value.is_empty() {
            Err(Error::TextEmpty)
        } else {
            Ok(Self(value.into()))
        }
    }

    /// Borrow the checked text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Text {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for Text {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl fmt::Display for Text {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
