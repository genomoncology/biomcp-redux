use super::{PhaseFilter, TrialSource};
use crate::{NciPhaseFilter, TrialStatus};
use serde::{Deserialize, Serialize};

// Publisher OpenAPI components.parameters.current_trial_status, checked 2026-10-29.
// https://clinicaltrialsapi.cancer.gov/api/v2/docs
biodata::wire_enum! {
    /// Exact NCI current_trial_status values, without registry translation.
    NciStatusFilter {
        /// Publisher Active label.
        Active => "Active",
        /// Publisher Administratively Complete label.
        AdministrativelyComplete => "Administratively Complete",
        /// Publisher Approved label.
        Approved => "Approved",
        /// Publisher Closed to Accrual label.
        ClosedToAccrual => "Closed to Accrual",
        /// Publisher Closed to Accrual and Intervention label.
        ClosedToAccrualAndIntervention => "Closed to Accrual and Intervention",
        /// Publisher Complete label.
        Complete => "Complete",
        /// Publisher INACTIVE label.
        Inactive => "INACTIVE",
        /// Publisher Enrolling by Invitation label.
        EnrollingByInvitation => "Enrolling by Invitation",
        /// Publisher Temporarily Closed to Accrual label.
        TemporarilyClosedToAccrual => "Temporarily Closed to Accrual",
        /// Publisher Temporarily Closed to Accrual and Intervention label.
        TemporarilyClosedToAccrualAndIntervention => "Temporarily Closed to Accrual and Intervention",
        /// Publisher Withdrawn label.
        Withdrawn => "Withdrawn",
    }
}

/// Source-qualified status input with the source checked by TrialSearch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialStatusFilter {
    /// ClinicalTrials.gov vocabulary.
    Registry(TrialStatus),
    /// NCI vocabulary.
    Nci(NciStatusFilter),
}
impl TrialStatusFilter {
    pub(super) fn valid_for(&self, source: TrialSource) -> bool {
        matches!(
            (self, source),
            (Self::Registry(_), TrialSource::Ctgov) | (Self::Nci(_), TrialSource::Nci)
        )
    }
}

/// Source-qualified phase input with the source checked by TrialSearch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TrialPhaseFilter {
    /// ClinicalTrials.gov vocabulary.
    Registry(PhaseFilter),
    /// NCI vocabulary.
    Nci(NciPhaseFilter),
}
impl TrialPhaseFilter {
    pub(super) fn valid_for(&self, source: TrialSource) -> bool {
        matches!(
            (self, source),
            (Self::Registry(_), TrialSource::Ctgov) | (Self::Nci(_), TrialSource::Nci)
        )
    }
}
