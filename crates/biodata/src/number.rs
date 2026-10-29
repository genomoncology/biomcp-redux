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
pub struct Fraction(f64);
impl Fraction {
    const MIN: f64 = 0.0;
    const MAX: f64 = 1.0;
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for Fraction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Fraction".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"number", "minimum":Self::MIN, "maximum":Self::MAX})
    }
}
impl TryFrom<f64> for Fraction {
    type Error = crate::Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && (Self::MIN..=Self::MAX).contains(&value) {
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

/// A source number with its explicitly stated unit; missing units stay absent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct WithUnit<V, U = crate::Text> {
    /// Unchanged numeric source value.
    pub value: V,
    /// Publisher unit or code, without conversion or inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<U>,
}
