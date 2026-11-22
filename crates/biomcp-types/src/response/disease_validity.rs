//! Source-native assertions and coverage for a disease validity read.
use crate::{ClinGenValidity, DiseaseGet, GenccSubmission, ResponseBody, Source};
use biodata::{GeneSymbol, HgncId, PartialDate};
use serde::{Deserialize, Serialize};

/// One curation keyed by the gene identity in its publisher's export.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "source")]
pub enum DiseaseCuration {
    /// ClinGen expert panel assessment.
    #[serde(rename = "clingen")]
    ClinGen {
        /// Gene in the source assertion.
        #[serde(flatten)]
        gene: DiseaseCurationGene,
        /// The publisher's validity assertion.
        assertion: ClinGenValidity,
    },
    /// One GenCC contributor's submission.
    #[serde(rename = "gencc")]
    Gencc {
        /// Gene in the source submission.
        #[serde(flatten)]
        gene: DiseaseCurationGene,
        /// The publisher's submission.
        assertion: GenccSubmission,
    },
}

/// Gene identified by a native curation row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseCurationGene {
    /// HGNC identity supplied by the publisher.
    pub gene_id: HgncId,
    /// Gene symbol supplied by the publisher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene_symbol: Option<GeneSymbol>,
}

/// Coverage of one local source copy that could be read.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseCurationCoverage {
    /// Export publisher.
    pub source: Source,
    /// Rows indexed from the source copy.
    pub export_rows: u64,
    /// Checked disease matches before applying classification and submitter filters.
    pub matched: u64,
    /// UTC retrieval date of the local copy, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved: Option<PartialDate>,
}

/// A page of separate source assertions with coverage per readable export.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseValidity {
    /// Curations on this page in source order.
    #[serde(default)]
    pub items: Box<[DiseaseCuration]>,
    /// Coverage and retrieval dates for readable copies.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub coverage: Box<[DiseaseCurationCoverage]>,
}

impl ResponseBody for DiseaseValidity {
    const SCHEMA: &'static str = "biomcp.disease-validity/1";
    type Query = DiseaseGet;
}
