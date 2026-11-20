//! FDA companion diagnostic rows that state each of FDA's cells once, under one name in both tables.
use biodata::{DiagnosticCellLink, DiagnosticDeviceId, FdaDrugApplication, PartialDate};
use serde::{Deserialize, Serialize};

/// A physical companion-table row; the two table layouts cannot acquire each other’s cells.
/// Each of FDA's cells is a named field holding its text once; both kinds lead with the row date
/// and the names both tables share.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum CompanionDiagnosticRow {
    /// One complete native individual table row.
    Individual {
        /// Row date and the cells both tables share.
        #[serde(flatten)]
        shared: CompanionShared,
        /// FDA's "Biomarker(s)" cell text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        biomarker: Option<Box<str>>,
        /// FDA's "Biomarker(s) (Details)" cell text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        biomarker_details: Option<Box<str>>,
        /// With `--biomarker`: true when the biomarker cell names it only as "Not NAME", such as
        /// "Not MSI-High", so the row is for patients without it; absent otherwise.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        biomarker_negated: bool,
        /// Row position, links, name match and the identifiers this row's cells own.
        #[serde(flatten)]
        owned: CompanionOwned,
    },
    /// One complete native group table row.
    Group {
        /// Row date and the cells both tables share.
        #[serde(flatten)]
        shared: CompanionShared,
        /// Row position, links, name match and the identifiers this row's cells own.
        #[serde(flatten)]
        owned: CompanionOwned,
    },
}
/// Fields both FDA companion tables carry under one name.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CompanionShared {
    /// Newest date in this row’s device cell, YYYY-MM-DD; absent when a date there is malformed or missing, and the row is then named in problems.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_date: Option<PartialDate>,
    /// FDA's diagnostic name and manufacturer cell text, as one line: line breaks and runs of spaces read as one space, so both tables name a test alike.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic: Option<Box<str>>,
    /// FDA's indication and sample type cell text, as one line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indication: Option<Box<str>>,
    /// FDA's device cell text: the device number and date (a PMA, 510(k) or De Novo number); not the test's name, which is in diagnostic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<str>>,
    /// FDA's drug cell text; in the group table, the drug group cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drug: Option<Box<str>>,
}
/// What a row owns besides its cell text.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CompanionOwned {
    /// Zero-based physical data-row index in its table.
    pub index: u32,
    /// Links inside FDA's cells, under the name of the cell that holds them.
    #[serde(default, skip_serializing_if = "CompanionLinks::is_empty")]
    pub links: CompanionLinks,
    /// The cells whose text holds the `--name` text; absent without `--name`.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub name_match: Box<[CompanionColumn]>,
    /// Device facts from this row’s own dedicated device cell only.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub device_ids: Box<[DiagnosticDeviceId]>,
    /// Unambiguous typed applications owned by this row’s drug cell; each one's link is in links.drug.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub drug_applications: Box<[FdaDrugApplication]>,
}
/// Each cell's links in FDA's order; a cell without links is absent.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CompanionLinks {
    /// Links in the diagnostic cell.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub diagnostic: Box<[DiagnosticCellLink]>,
    /// Links in the indication cell.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub indication: Box<[DiagnosticCellLink]>,
    /// Links in the device cell.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub device: Box<[DiagnosticCellLink]>,
    /// Links in the drug cell.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub drug: Box<[DiagnosticCellLink]>,
    /// Links in the individual table's biomarker cell.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub biomarker: Box<[DiagnosticCellLink]>,
    /// Links in the individual table's biomarker details cell.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub biomarker_details: Box<[DiagnosticCellLink]>,
}
impl CompanionLinks {
    /// Whether no cell of the row holds a link.
    pub fn is_empty(&self) -> bool {
        [
            &self.diagnostic,
            &self.indication,
            &self.device,
            &self.drug,
            &self.biomarker,
            &self.biomarker_details,
        ]
        .iter()
        .all(|links| links.is_empty())
    }
}
biodata::wire_enum! {
    /// A companion row cell that `--name` searches.
    CompanionColumn {
        /// The diagnostic name and manufacturer cell.
        Diagnostic => "diagnostic",
        /// The drug cell; in the group table, the drug group cell.
        Drug => "drug"
    }
}
impl CompanionDiagnosticRow {
    /// What this row owns besides its cell text.
    pub fn owned(&self) -> &CompanionOwned {
        match self {
            Self::Individual { owned, .. } | Self::Group { owned, .. } => owned,
        }
    }
}
