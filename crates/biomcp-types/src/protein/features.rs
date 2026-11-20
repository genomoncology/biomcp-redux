//! Features of a selected UniProt entry share its attribution.
use crate::{ProteinFeaturesQuery, ResponseBody};
use biodata::{Attribution, ProteinCoordinateConvention, ProteinFeature, UniProtFeature};
use serde::{Deserialize, Serialize};

/// Native UniProt feature rows, preserving exact labels and uncertain bounds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFeatures {
    /// Entry provenance shared by this feature list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
    /// Whole native rows in source order; totals apply only to this collection.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub items: Box<[ProteinFeatureRow]>,
    /// How to read the spans, present when any row is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_note: Option<Box<str>>,
}

impl ProteinFeatures {
    /// Collect features for one entry. The UniProt adapter gives them the same attribution.
    pub fn for_entry(features: impl IntoIterator<Item = ProteinFeature>) -> Self {
        let mut features = features.into_iter().peekable();
        let attribution = features.peek().map(|row| row.attribution.clone());
        Self {
            coordinate_note: super::coordinate_note(attribution.is_some()),
            attribution,
            items: features.map(ProteinFeatureRow::from).collect(),
        }
    }
}

impl ResponseBody for ProteinFeatures {
    const SCHEMA: &'static str = "biomcp.protein-features/7";
    type Query = ProteinFeaturesQuery;
}

/// A feature with its own coordinate accession and original convention.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFeatureRow {
    /// Feature data, including source-specific sequence coordinates and residues.
    pub feature: UniProtFeature,
    /// Coordinate convention in the source data.
    pub source_coordinates: ProteinCoordinateConvention,
}

impl From<ProteinFeature> for ProteinFeatureRow {
    fn from(row: ProteinFeature) -> Self {
        Self {
            feature: row.feature,
            source_coordinates: row.source_coordinates,
        }
    }
}

/// First page of features embedded in the selected protein card.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFeaturePage {
    /// Entry provenance shared by the section's features.
    pub attribution: Attribution,
    /// Native feature count before the section limit.
    pub total: u64,
    /// Rows in source order.
    pub items: Box<[ProteinFeatureRow]>,
}
