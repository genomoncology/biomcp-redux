use crate::Error;
use serde::{Deserialize, Serialize};
/// A bounded source range; equal min and max denote an exact boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Bounds")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Bounds"))]
pub struct Bound(Bounds);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("anyOf" = [{"required":["min"]},{"required":["max"]}])))]
struct Bounds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max: Option<u32>,
}
impl TryFrom<Bounds> for Bound {
    type Error = Error;
    fn try_from(v: Bounds) -> Result<Self, Error> {
        Self::new(v.min, v.max)
    }
}
impl Bound {
    /// Require at least one side and ordered supplied sides.
    pub fn new(min: Option<u32>, max: Option<u32>) -> Result<Self, Error> {
        if (min.is_none() && max.is_none()) || min.zip(max).is_some_and(|(a, b)| a > b) {
            return Err(Error::SpanOrder);
        }
        Ok(Self(Bounds { min, max }))
    }
    /// Lowest possible boundary, when stated.
    pub fn min(self) -> Option<u32> {
        self.0.min
    }
    /// Highest possible boundary, when stated.
    pub fn max(self) -> Option<u32> {
        self.0.max
    }
    fn preceding(self) -> Result<Self, Error> {
        let subtract = |v: u32| v.checked_sub(1).ok_or(Error::PositionZero);
        Self::new(
            self.min().map(subtract).transpose()?,
            self.max().map(subtract).transpose()?,
        )
    }
}
/// Zero-based interbase span retaining uncertain boundary ranges.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SpanFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SpanFields"))]
pub struct UncertainSpan0(SpanFields);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SpanFields {
    start: Bound,
    end: Bound,
}
impl TryFrom<SpanFields> for UncertainSpan0 {
    type Error = Error;
    fn try_from(v: SpanFields) -> Result<Self, Error> {
        Self::new(v.start, v.end)
    }
}
impl UncertainSpan0 {
    /// Refuse a definitely reversed interval without claiming exact unknown bounds.
    pub fn new(start: Bound, end: Bound) -> Result<Self, Error> {
        if start.min().zip(end.max()).is_some_and(|(a, b)| a > b) {
            return Err(Error::SpanOrder);
        }
        Ok(Self(SpanFields { start, end }))
    }
    /// Convert native one-based inclusive bounds without collapsing uncertainty.
    pub fn from_inclusive(start: Bound, end: Bound) -> Result<Self, Error> {
        if [start.min(), start.max(), end.min(), end.max()].contains(&Some(0)) {
            return Err(Error::PositionZero);
        }
        if start.min().zip(end.max()).is_some_and(|(a, b)| a > b) {
            return Err(Error::SpanOrder);
        }
        Self::new(start.preceding()?, end)
    }
    /// Inclusive start boundary range.
    pub fn start(self) -> Bound {
        self.0.start
    }
    /// Exclusive end boundary range.
    pub fn end(self) -> Bound {
        self.0.end
    }
}
