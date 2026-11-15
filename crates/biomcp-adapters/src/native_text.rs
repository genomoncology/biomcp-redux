//! Admit native human text without changing identity and protocol parsing.
use biodata::Text;
use serde::{Deserialize, Serialize};

/// Admit source prose. The 1.0.0 `Text` contract holds no control character, and
/// `Text::from_source` would print a line break as the literal `\u{000a}`, so the
/// non-empty trimmed lines join with one space first (0947, 0961). Every other
/// control keeps its visible escape.
pub(crate) fn admit(value: &str) -> Result<Text, biodata::Error> {
    if !value.contains(['\r', '\n']) {
        return Text::from_source(value);
    }
    let joined = value
        .split(['\r', '\n'])
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    Text::from_source(&joined)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
/// Native source prose admitted through the shared checked text constructor.
pub struct NativeText(Text);
impl<'de> Deserialize<'de> for NativeText {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        admit(&value)
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}
impl std::ops::Deref for NativeText {
    type Target = Text;
    fn deref(&self) -> &Text {
        &self.0
    }
}
impl From<NativeText> for Text {
    fn from(value: NativeText) -> Self {
        value.0
    }
}
impl std::fmt::Display for NativeText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub(crate) fn deserialize<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Text, D::Error> {
    NativeText::deserialize(d).map(Into::into)
}
