use crate::Error;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Bounded nonempty text with surrounding Unicode whitespace removed.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from, transparent))]
pub struct Text(
    #[cfg_attr(
        feature = "schema",
        schemars(
            length(min = 1, max = Text::MAX_CHARS),
            regex(
                pattern = r"^(?![\s\S]*[\x00-\x1f\x7f-\x9f\u061c\u200e\u200f\u202a-\u202e\u2066-\u2069])"
            )
        )
    )]
    Box<str>,
);

impl Text {
    /// Maximum Unicode scalar count after trimming or source escaping.
    pub const MAX_CHARS: usize = 1_048_576;

    /// Trim whitespace and reject empty, oversized or unsafe control text.
    /// C0, DEL, C1 and Unicode bidi controls must use visible source escapes.
    pub fn new(value: &str) -> Result<Self, Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(Error::TextEmpty)
        } else if value.chars().any(unsafe_control) {
            Err(Error::TextControl)
        } else if trimmed.chars().count() > Self::MAX_CHARS {
            Err(Error::TextTooLong)
        } else {
            Ok(Self(trimmed.into()))
        }
    }

    /// Admit source text without dropping a record for unsafe controls.
    /// Controls become literal `\\u{xxxx}` escapes before constructor checks.
    pub fn from_source(value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::TextEmpty);
        }
        let mut escaped = String::new();
        for ch in value
            .trim_matches(|ch: char| ch.is_whitespace() && !unsafe_control(ch))
            .chars()
        {
            if unsafe_control(ch) {
                use std::fmt::Write;
                write!(&mut escaped, "\\u{{{:04x}}}", u32::from(ch))
                    .map_err(|_| Error::TextControl)?;
            } else {
                escaped.push(ch);
            }
            if escaped.len() > Self::MAX_CHARS * 4 {
                return Err(Error::TextTooLong);
            }
        }
        Self::new(&escaped)
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

pub(crate) fn unsafe_control(ch: char) -> bool {
    ch.is_control()
        || matches!(ch, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}
