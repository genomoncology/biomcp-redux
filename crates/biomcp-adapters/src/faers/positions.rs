//! Native row positions retained independently of public report values.
use crate::NativePositions;
use serde::{Deserialize, Serialize};
/// Rejected native rows keep their positions across source snapshot reloads.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Positions {
    /// Native drugs positions when some rows were rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drugs: Option<NativePositions>,
    /// Native reactions positions when some rows were rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<NativePositions>,
}
impl Positions {
    /// Whether the native arrays retain their original dense positions.
    pub fn is_empty(&self) -> bool {
        self.drugs.is_none() && self.reactions.is_none()
    }
    /// Check stored positions against their admitted report rows.
    pub fn check(&self, report: &biodata::FaersReport) -> bool {
        self.drugs
            .as_ref()
            .is_none_or(|p| p.check(report.drugs.len()))
            && self
                .reactions
                .as_ref()
                .is_none_or(|p| p.check(report.reactions.len()))
    }
}
/// Native values the shared report type does not hold.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Extras {
    /// Unchanged `patient.summary.narrativeincludeclinical`; openFDA fills it with the case event date only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_event_date: Option<Box<str>>,
    /// Unchanged `patient.patientonsetageunit`, kept so an off-list unit still shows its code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_unit: Option<Box<str>>,
}
/// Checked report values and their unchanged native row positions.
#[derive(Debug)]
pub(super) struct Report {
    /// Admitted native report values.
    pub data: biodata::FaersReport,
    /// Positions omitted from the admitted row arrays.
    pub positions: Positions,
    /// Native values beside the shared report.
    pub extras: Extras,
}
