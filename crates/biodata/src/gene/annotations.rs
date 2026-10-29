use super::SummarySource;
use crate::{Accession, Error, PartialDate, Text};
use serde::{Deserialize, Serialize};
crate::canonical_enum! {
    /// The source-stated MANE transcript status.
    ManeStatus, ManeStatusUnknown {
        /// MANE Select transcript pair.
        Select => "select",
        /// MANE Plus Clinical transcript pair.
        PlusClinical => "plus_clinical",
    }
}
/// A source-stated MANE pair with versioned sequence accessions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ManeFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from))]
pub struct Mane {
    /// Source-stated transcript status.
    pub status: ManeStatus,
    /// Versioned RefSeq accession.
    #[cfg_attr(feature = "schema", schemars(schema_with = "mane_refseq_schema"))]
    refseq: Accession,
    /// Versioned Ensembl accession.
    #[cfg_attr(feature = "schema", schemars(schema_with = "mane_ensembl_schema"))]
    ensembl: Accession,
}
impl Mane {
    /// Preserve a source-stated transcript pair and its status.
    pub fn new(status: ManeStatus, refseq: Accession, ensembl: Accession) -> Result<Self, Error> {
        if !matches!(
            refseq.0,
            crate::variant::AccessionKind::RefSeq {
                prefix: crate::RefSeqPrefix::Nm | crate::RefSeqPrefix::Nr,
                ..
            }
        ) || !matches!(
            ensembl.0,
            crate::variant::AccessionKind::Ensembl {
                feature: crate::EnsemblFeature::Transcript,
                ..
            }
        ) {
            return Err(Error::AccessionInvalid);
        }
        refseq.to_string().parse::<Accession>()?;
        ensembl.to_string().parse::<Accession>()?;
        Ok(Self {
            status,
            refseq,
            ensembl,
        })
    }
    /// Borrow the versioned RefSeq accession.
    pub fn refseq(&self) -> &Accession {
        &self.refseq
    }
    /// Borrow the versioned Ensembl accession.
    pub fn ensembl(&self) -> &Accession {
        &self.ensembl
    }
}
/// A whole source-provided summary with attribution and stated date precision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Deserialize)]
struct ManeFields {
    status: ManeStatus,
    refseq: Accession,
    ensembl: Accession,
}
impl TryFrom<ManeFields> for Mane {
    type Error = Error;
    fn try_from(f: ManeFields) -> Result<Self, Error> {
        Self::new(f.status, f.refseq, f.ensembl)
    }
}
#[cfg(feature = "schema")]
fn mane_refseq_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"type":"string", "pattern":r"^(NM_|NR_)([0-9]{6}|[0-9]{9})\.[1-9][0-9]*$"})
}
#[cfg(feature = "schema")]
fn mane_ensembl_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"type":"string", "pattern":r"^ENST[0-9]{11}\.[1-9][0-9]*$"})
}
