//! Registry identities and source-native trial values.
use crate::{PartialDate, Text};
use serde::{Deserialize, Serialize};
/// Canonical ClinicalTrials.gov identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NctId(u32);
impl std::str::FromStr for NctId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digits = value
            .strip_prefix("NCT")
            .ok_or(crate::Error::TrialIdInvalid)?;
        if digits.len() != 8 || !digits.bytes().all(|c| c.is_ascii_digit()) {
            return Err(crate::Error::TrialIdInvalid);
        }
        let number = digits.parse().map_err(|_| crate::Error::TrialIdInvalid)?;
        if number == 0 {
            return Err(crate::Error::TrialIdInvalid);
        }
        Ok(Self(number))
    }
}
impl std::fmt::Display for NctId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NCT{:08}", self.0)
    }
}
crate::macros::text_value!(NctId, "NctId", r"^NCT(?!00000000)[0-9]{8}$");
crate::macros::wire_enum! {
    /// Source-native registry value.
    TrialPhase {
        /// Registry EARLY_PHASE1 value.
        Early1 => "EARLY_PHASE1",
        /// Registry PHASE1 value.
        Phase1 => "PHASE1",
        /// Registry PHASE2 value.
        Phase2 => "PHASE2",
        /// Registry PHASE3 value.
        Phase3 => "PHASE3",
        /// Registry PHASE4 value.
        Phase4 => "PHASE4",
        /// Registry NA value.
        NotApplicable => "NA",
    }
}
crate::macros::wire_enum! {
    /// Source-native registry value.
    TrialStatus {
        /// Registry ACTIVE_NOT_RECRUITING value.
        ActiveNotRecruiting => "ACTIVE_NOT_RECRUITING",
        /// Registry COMPLETED value.
        Completed => "COMPLETED",
        /// Registry ENROLLING_BY_INVITATION value.
        EnrollingByInvitation => "ENROLLING_BY_INVITATION",
        /// Registry NOT_YET_RECRUITING value.
        NotYetRecruiting => "NOT_YET_RECRUITING",
        /// Registry RECRUITING value.
        Recruiting => "RECRUITING",
        /// Registry SUSPENDED value.
        Suspended => "SUSPENDED",
        /// Registry TERMINATED value.
        Terminated => "TERMINATED",
        /// Registry WITHDRAWN value.
        Withdrawn => "WITHDRAWN",
        /// Registry AVAILABLE value.
        Available => "AVAILABLE",
        /// Registry NO_LONGER_AVAILABLE value.
        NoLongerAvailable => "NO_LONGER_AVAILABLE",
        /// Registry APPROVED_FOR_MARKETING value.
        ApprovedForMarketing => "APPROVED_FOR_MARKETING",
        /// Registry WITHHELD value.
        Withheld => "WITHHELD",
        /// Registry UNKNOWN value.
        Unknown => "UNKNOWN",
    }
}
crate::macros::wire_enum! {
    /// Source-native registry value.
    StudyType {
        /// Registry INTERVENTIONAL value.
        Interventional => "INTERVENTIONAL",
        /// Registry OBSERVATIONAL value.
        Observational => "OBSERVATIONAL",
        /// Registry EXPANDED_ACCESS value.
        ExpandedAccess => "EXPANDED_ACCESS",
    }
}
crate::macros::wire_enum! {
    /// Source-native registry value.
    TrialSex {
        /// Registry ALL value.
        All => "ALL",
        /// Registry FEMALE value.
        Female => "FEMALE",
        /// Registry MALE value.
        Male => "MALE",
    }
}
/// Small source-attributed trial discovery record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialHeader {
    /// Exact registry identity.
    pub nct_id: NctId,
    /// Complete brief title.
    pub title: Text,
    /// Registry's overall status, independent of site status.
    pub status: TrialStatus,
    /// Source phases; absent phases remain absent.
    pub phases: Box<[TrialPhase]>,
    /// Source condition names.
    pub conditions: crate::Capped<Text>,
    /// Source intervention names.
    pub interventions: crate::Capped<Text>,
    /// Last posted registry update.
    pub updated: PartialDate,
    /// Publisher's canonical record URL.
    pub url: Text,
}
