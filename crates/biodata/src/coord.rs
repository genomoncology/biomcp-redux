use crate::Error;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// A positive one-based position, without an assembly or biological meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from, !into, transparent))]
pub struct Position1(#[cfg_attr(feature = "schema", schemars(range(max = u32::MAX)))] NonZeroU32);

impl Position1 {
    /// Check that a position is positive.
    pub fn new(value: u32) -> Result<Self, Error> {
        NonZeroU32::new(value).map(Self).ok_or(Error::PositionZero)
    }

    /// Return the one-based position.
    pub fn get(self) -> u32 {
        self.0.get()
    }
}

impl TryFrom<u32> for Position1 {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        Self::new(value)
    }
}

impl From<Position1> for u32 {
    fn from(value: Position1) -> u32 {
        value.get()
    }
}

/// A zero-based half-open span with ordered bounds; empty spans are valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(deny_unknown_fields))]
pub struct Span0 {
    #[cfg_attr(feature = "schema", schemars(range(max = u32::MAX)))]
    start: u32,
    #[cfg_attr(feature = "schema", schemars(range(max = u32::MAX)))]
    end: u32,
}

impl Span0 {
    /// Check that the end is at or after the start.
    pub fn new(start: u32, end: u32) -> Result<Self, Error> {
        if start > end {
            Err(Error::SpanOrder)
        } else {
            Ok(Self { start, end })
        }
    }

    /// Return the inclusive zero-based start.
    pub fn start(self) -> u32 {
        self.start
    }
    /// Return the exclusive zero-based end.
    pub fn end(self) -> u32 {
        self.end
    }
}

#[derive(Deserialize)]
struct SpanBounds {
    start: u32,
    end: u32,
}

impl TryFrom<SpanBounds> for Span0 {
    type Error = Error;
    fn try_from(value: SpanBounds) -> Result<Self, Error> {
        Self::new(value.start, value.end)
    }
}

impl<'de> Deserialize<'de> for Span0 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Span0;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a span object with start and end bounds")
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, map: A) -> Result<Span0, A::Error> {
                let bounds =
                    SpanBounds::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                Span0::try_from(bounds).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

mod genomic;
pub use genomic::{Build, Contig, GenomicInterval, Strand};

mod chromosome;
pub use chromosome::Chromosome;
