//! CDC WONDER retains native aggregate cells and independent table metadata.
use crate::ResponseBody;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

biodata::wire_enum! { described
    /// One independent CDC WONDER grouping.
    VaersSection {
        /// Native reported symptom rows.
        Symptoms => "symptoms",
        /// Reports classified by native seriousness.
        Seriousness => "seriousness",
        /// Native age groups.
        Age => "age",
    }
}
impl VaersSection {
    /// Native grouping code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Symptoms => "D8.V13-level2",
            Self::Seriousness => "D8.V10",
            Self::Age => "D8.V1",
        }
    }
}
pub(super) fn vaccine(text: &str) -> Option<&'static str> {
    if text.eq_ignore_ascii_case("MMR") {
        Some("MMR")
    } else if text.eq_ignore_ascii_case("COVID19") || text.eq_ignore_ascii_case("COVID-19 vaccine")
    {
        Some("COVID19")
    } else {
        None
    }
}
/// Original native XML attributes, including label, value, displayed footer and suppression.
pub type VaersCell = BTreeMap<Box<str>, Box<str>>;
/// One whole native row, never reconstructed from another grouping.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VaersRow {
    /// Native label cell; no code is invented when absent.
    pub label: VaersCell,
    /// Native Events Reported cell, preserving suppressed or displayed-total text.
    pub reports: VaersCell,
    /// Native Percent cell, retaining values above one hundred percent.
    pub percent: VaersCell,
}
/// Applied source text, retaining its original content and source role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VaersNotice {
    /// Message, footnote or evaluated caveat.
    pub kind: Box<str>,
    /// Original source text, including embedded publisher markup.
    pub text: Box<str>,
}
/// Bounded page of one independent native vaccine summary table.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VaersSummary {
    /// Native selected vaccine attributes.
    pub vaccine: VaersCell,
    /// Native grouping attributes.
    pub grouping: VaersCell,
    /// Native dataset attributes including processed-through vintage.
    pub dataset: VaersCell,
    /// Source measure definitions, including original names and codes.
    pub measures: Box<[VaersCell]>,
    /// Response-specific measure metadata without denominator inference.
    pub measure_metadata: Box<[VaersCell]>,
    /// Native report-total message, separate from the footer aggregation.
    pub report_total: Box<str>,
    /// Whole native footer cells, never a sum calculated by this program.
    pub footer: VaersRow,
    /// Native row order with local paging.
    pub rows: Box<[VaersRow]>,
    /// Applied native messages, evaluated caveats and footnotes.
    pub notices: Box<[VaersNotice]>,
    /// Reviewed interpretation boundary.
    pub interpretation: Box<str>,
}
impl ResponseBody for VaersSummary {
    const SCHEMA: &'static str = "biomcp.vaers_summary/1";
    type Query = super::AdverseEventSearch;
}
