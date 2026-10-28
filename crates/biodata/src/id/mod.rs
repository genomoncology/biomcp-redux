use crate::{Error, text_value};
use std::{fmt, num::NonZeroU32, str::FromStr};

/// An HGNC identifier with canonical namespace and positive numeric value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HgncId(NonZeroU32);

impl HgncId {
    const PREFIX: &str = "HGNC:";

    /// Construct an identifier from a positive namespace-local number.
    pub fn new(value: u32) -> Result<Self, Error> {
        NonZeroU32::new(value).map(Self).ok_or(Error::HgncIdInvalid)
    }

    /// Return the namespace-local number.
    pub fn get(self) -> u32 {
        self.0.get()
    }
}

impl FromStr for HgncId {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Error> {
        let number = value
            .strip_prefix(Self::PREFIX)
            .ok_or(Error::HgncIdInvalid)?;
        if number.starts_with('0') || !number.bytes().all(|ch| ch.is_ascii_digit()) {
            return Err(Error::HgncIdInvalid);
        }
        Self::new(number.parse().map_err(|_| Error::HgncIdInvalid)?)
    }
}

impl fmt::Display for HgncId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", Self::PREFIX, self.get())
    }
}

text_value!(
    HgncId,
    "HgncId",
    format!("^{}[1-9][0-9]*$", Self::PREFIX),
    "An HGNC identifier with canonical namespace and positive numeric value."
);

mod article;
pub use article::{Doi, EuropePmcId, Pmcid, Pmid, SemanticScholarId};
mod gene;
mod transcript;
pub use gene::{Cytoband, GeneSymbol, NcbiGeneId, OmimId, UniProtAcc};
pub use transcript::EnsemblGeneId;
