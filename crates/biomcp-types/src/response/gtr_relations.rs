//! Relation pages from one exact, versioned GTR test.
use super::DiagnosticCountUnit;
use crate::DiagnosticGet;
use biodata::{
    Curie, GtrConditionReference, GtrConditionRelation, GtrGeneReference, GtrGeneRelation,
    GtrTestId, GtrTestType,
};
use serde::{Deserialize, Serialize};

/// Native gene relation from the reply's test.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticGeneRow {
    /// Native source test type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_type: Option<GtrTestType>,
    /// Source-given GeneID and gene symbol.
    pub gene: GtrGeneReference,
    /// Uninterpreted source ID when it is not a checked GeneID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_gene_or_snomed_ct_id: Option<Box<str>>,
    /// Independent native MedGen identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_medgen: Option<Curie>,
    /// Native MIM cell, including empty.
    pub mim_number: Box<str>,
    /// Full source object name.
    pub object_name: Box<str>,
}

impl From<GtrGeneRelation> for DiagnosticGeneRow {
    fn from(row: GtrGeneRelation) -> Self {
        Self {
            test_type: row.test_type,
            gene: row.gene,
            native_gene_or_snomed_ct_id: row.native_gene_or_snomed_ct_id,
            native_medgen: row.native_medgen,
            mim_number: row.mim_number,
            object_name: row.object_name,
        }
    }
}

/// Native condition relation from the reply's test.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiagnosticConditionRow {
    /// Native source test type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_type: Option<GtrTestType>,
    /// Source-given MedGen concept.
    pub condition: GtrConditionReference,
    /// Native MIM cell, including empty.
    pub mim_number: Box<str>,
    /// Native SNOMED CT cell, including empty.
    pub snomed_ct_id: Box<str>,
}

impl From<GtrConditionRelation> for DiagnosticConditionRow {
    fn from(row: GtrConditionRelation) -> Self {
        Self {
            test_type: row.test_type,
            condition: row.condition,
            mim_number: row.mim_number,
            snomed_ct_id: row.snomed_ct_id,
        }
    }
}

page!(
    diagnostic,
    DiagnosticGenes,
    DiagnosticGeneRow,
    DiagnosticGet,
    "biomcp.diagnostic-genes/7",
    "Native GTR gene relation rows without variant-coverage inference.",
    [
        /// Exact owning test accession and version for every row.
        pub test_id: GtrTestId,
    ]
);
page!(
    diagnostic,
    DiagnosticConditions,
    DiagnosticConditionRow,
    DiagnosticGet,
    "biomcp.diagnostic-conditions/7",
    "Native GTR condition rows without ontology or clinical conversion.",
    [
        /// Exact owning test accession and version for every row.
        pub test_id: GtrTestId,
    ]
);
