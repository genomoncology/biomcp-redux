use crate::{Error, Span0, canonical_enum, text_value};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
canonical_enum! {
    /// The assembly stated by the source; no default is inferred.
    Build, BuildUnknown {
        /// Genome Reference Consortium human build 37.
        GRCh37 => "GRCh37",
        /// Genome Reference Consortium human build 38.
        GRCh38 => "GRCh38",
    }
}
canonical_enum! {
    /// Source-stated forward or reverse strand.
    Strand, StrandUnknown {
        /// Forward strand.
        Plus => "+",
        /// Reverse strand.
        Minus => "-",
    }
}
/// A source chromosome or scaffold name without spelling normalization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Contig(Box<str>);
impl Contig {
    /// Maximum accepted source name length in bytes.
    pub const MAX_LEN: usize = 255;
    /// Check a nonempty printable ASCII name without inferring assembly membership.
    pub fn new(value: &str) -> Result<Self, Error> {
        if value.is_empty()
            || value.len() > Self::MAX_LEN
            || !value.bytes().all(|ch| ch.is_ascii_graphic())
        {
            return Err(Error::ContigInvalid);
        }
        Ok(Self(value.into()))
    }
    /// Borrow the preserved source name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for Contig {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::new(value)
    }
}
impl fmt::Display for Contig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    Contig,
    "Contig",
    format!(r"^[\x21-\x7e]{{1,{}}}$", Self::MAX_LEN),
    "A source chromosome or scaffold name without spelling normalization."
);

/// An assembly-qualified zero-based half-open source interval.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenomicInterval {
    /// Source-stated assembly, omitted when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    /// Canonical primary chromosome.
    pub chromosome: crate::Chromosome,
    /// Checked zero-based half-open bounds; an empty span remains valid.
    pub span: Span0,
    /// Source-stated strand, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strand: Option<Strand>,
}

impl GenomicInterval {
    /// Compare regions only when both sources state the same assembly and chromosome.
    pub fn contains(&self, other: &Self) -> Result<bool, Error> {
        let (Some(build), Some(other_build)) = (self.build, other.build) else {
            return Err(Error::IntervalBuildAbsent);
        };
        if build != other_build {
            return Err(Error::IntervalBuildMismatch);
        }
        if self.chromosome != other.chromosome {
            return Err(Error::IntervalChromosomeMismatch);
        }
        Ok(self.span.start() <= other.span.start() && self.span.end() >= other.span.end())
    }
}
