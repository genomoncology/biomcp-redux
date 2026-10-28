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
/// A non-finite decimal was refused.
#[derive(Clone, Copy, Debug)]
pub struct FiniteNumberError;
impl std::fmt::Display for FiniteNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(crate::Error::NumberNonFinite.code())
    }
}
impl std::error::Error for FiniteNumberError {}

/// A finite source fraction in the closed interval zero to one.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(transparent))]
pub struct Fraction(#[cfg_attr(feature = "schema", schemars(range(min = 0.0, max = 1.0)))] f64);
impl TryFrom<f64> for Fraction {
    type Error = crate::Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::Error::PopulationFrequencyRange)
        }
    }
}
impl From<Fraction> for f64 {
    fn from(value: Fraction) -> Self {
        value.0
    }
}
