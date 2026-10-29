//! Native source timestamps may include an offset or leave their timezone unstated.
use super::{SourceLocalTimestampValue, SourceTimestampTimezone, SourceTimestampValue};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Checked native timestamp text preserving explicit or missing timezone information.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SourceDateTimeValue {
    /// RFC 3339 timestamp with its original offset.
    Timestamp(SourceTimestampValue),
    /// Calendar and clock text whose source states no timezone.
    Local(SourceLocalTimestampValue),
}
impl FromStr for SourceDateTimeValue {
    type Err = biodata::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = value.parse::<SourceTimestampValue>() {
            return Ok(Self::Timestamp(value));
        }
        value.parse().map(Self::Local)
    }
}
/// Native timestamp with the existing explicit timezone disclosure.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SourceDateTime {
    /// Unchanged checked source text.
    pub value: SourceDateTimeValue,
    /// Whether the native timestamp states its timezone.
    pub timezone: SourceTimestampTimezone,
}
impl FromStr for SourceDateTime {
    type Err = biodata::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.parse()?;
        let timezone = match &value {
            SourceDateTimeValue::Timestamp(_) => SourceTimestampTimezone::InValue,
            SourceDateTimeValue::Local(_) => SourceTimestampTimezone::SourceGaveNone,
        };
        Ok(Self { value, timezone })
    }
}
