use crate::{AccessionBase, Text};
use serde::{Deserialize, Serialize};
/// A finite decimal that JSON can represent without losing its value to null.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "f64"))]
pub struct FiniteNumber(f64);
impl TryFrom<f64> for FiniteNumber {
    type Error = FiniteNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(FiniteNumberError)
        }
    }
}
impl From<FiniteNumber> for f64 {
    fn from(value: FiniteNumber) -> Self {
        value.0
    }
}
/// Source constraint metrics for the source's preferred transcript.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Constraint {
    /// Probability of loss-of-function intolerance.
    pub pli: FiniteNumber,
    /// Upper confidence bound of the observed-to-expected loss-of-function ratio.
    pub loeuf: FiniteNumber,
    /// Observed-to-expected loss-of-function ratio.
    pub oe_lof: FiniteNumber,
    /// Missense Z score.
    pub mis_z: FiniteNumber,
    /// Synonymous Z score.
    pub syn_z: FiniteNumber,
    /// Source flags, omitted when empty.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub flags: Box<[Text]>,
    /// Source MANE Select transcript, otherwise the source canonical transcript.
    pub transcript: AccessionBase,
    /// Source dataset family, not a minor release identifier.
    pub version: Text,
}

/// A non-finite decimal was refused.
#[derive(Clone, Copy, Debug)]
pub struct FiniteNumberError;
impl std::fmt::Display for FiniteNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("number.non_finite")
    }
}
impl std::error::Error for FiniteNumberError {}
