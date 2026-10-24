use crate::{EnsemblTranscript, Error, PartialDate, RefSeqTranscript, Text};
use super::SummarySource;
use serde::{Deserialize, Serialize};

/// A MANE Select pair whose two accessions both state their versions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ManePair")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from, deny_unknown_fields))]
pub struct ManeSelect {
    refseq: RefSeqTranscript,
    ensembl: EnsemblTranscript,
}
impl ManeSelect {
    /// Require both transcript versions at the MANE boundary.
    pub fn new(refseq: RefSeqTranscript, ensembl: EnsemblTranscript) -> Result<Self, Error> {
        if refseq.version().is_none() || ensembl.version().is_none() || !(refseq.base().starts_with("NM_") || refseq.base().starts_with("NR_")) { return Err(Error::ManeVersionRequired); }
        Ok(Self { refseq, ensembl })
    }
    /// Borrow the versioned RefSeq transcript.
    pub fn refseq(&self) -> &RefSeqTranscript { &self.refseq }
    /// Borrow the versioned Ensembl transcript.
    pub fn ensembl(&self) -> &EnsemblTranscript { &self.ensembl }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManePair { refseq: RefSeqTranscript, ensembl: EnsemblTranscript }
impl TryFrom<ManePair> for ManeSelect {
    type Error = Error;
    fn try_from(value: ManePair) -> Result<Self, Error> { Self::new(value.refseq, value.ensembl) }
}

/// A whole source-provided summary with attribution and stated date precision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Summary {
    /// Entire summary text.
    pub text: Text,
    /// The summary's attribution, independent of the fetching provider.
    pub source: SummarySource,
    /// Date stated by the source, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dated: Option<PartialDate>,
}
