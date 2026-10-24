use crate::{Error, Span0, macros::text_value, source::canonical_enum};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
canonical_enum! {
    /// The assembly stated by the source; no default is inferred.
    Build, BuildUnknown {
        /// Genome Reference Consortium human build 37.
        Grch37 => "GRCh37",
        /// Genome Reference Consortium human build 38.
        Grch38 => "GRCh38",
        /// Source did not state an assembly.
        Unstated => "unstated",
    }
}
canonical_enum! {
    /// Source-stated forward or reverse strand.
    Strand, StrandUnknown {
        /// Forward strand.
        Forward => "+",
        /// Reverse strand.
        Reverse => "-",
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
    format!(r"^[\x21-\x7e]{{1,{}}}$", Self::MAX_LEN)
);

/// An assembly-qualified zero-based half-open source interval.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenomicInterval {
    /// Source-stated assembly, explicitly unstated when absent.
    pub build: Build,
    /// Source chromosome or scaffold name.
    pub contig: Contig,
    /// Checked zero-based half-open bounds; an empty span remains valid.
    pub span: Span0,
    /// Source-stated strand, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strand: Option<Strand>,
}
