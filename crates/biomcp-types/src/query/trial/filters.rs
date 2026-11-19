use super::TrialSource;
use crate::{NciPhaseFilter, TrialPhase, TrialStatus};
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
/// Registry rows print phases as PHASE3 or EARLY_PHASE1, so those spellings read back as the
/// same registry filter (1063).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TrialPhaseFilter {
    /// ClinicalTrials.gov vocabulary.
    Registry(PhaseFilter),
    /// NCI vocabulary.
    Nci(NciPhaseFilter),
}
impl<'de> Deserialize<'de> for TrialPhaseFilter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse()
            .map(Self::Registry)
            .or_else(|_| text.parse().map(Self::Nci))
            .or_else(|_| text.parse().map(|phase| Self::Registry(PhaseFilter::printed(phase))))
            .map_err(|_| serde::de::Error::custom("unknown trial phase"))
    }
}
impl TrialPhaseFilter {
    /// The registry prints NA for not applicable, which NCI also spells NA.
    pub(super) fn for_source(self, source: TrialSource) -> Self {
        match (self, source) {
            (Self::Nci(NciPhaseFilter::NotApplicable), TrialSource::Ctgov) => {
                Self::Registry(PhaseFilter::NotApplicable)
            }
            (phase, _) => phase,
        }
    }
    pub(super) fn valid_for(&self, source: TrialSource) -> bool {
        matches!(
            (self, source),
            (Self::Registry(_), TrialSource::Ctgov) | (Self::Nci(_), TrialSource::Nci)
        )
    }
}

#[cfg(feature = "schema")]
fn filter_schema<A: schemars::JsonSchema, B: schemars::JsonSchema>(
    printed: &[&str],
) -> schemars::Schema {
    let mut values: Vec<serde_json::Value> = Vec::new();
    let mut by_source = schemars::json_schema!({});
    for (source, schema, extra) in [
        (TrialSource::Ctgov, schemars::schema_for!(A), printed),
        (TrialSource::Nci, schemars::schema_for!(B), &[][..]),
    ] {
        if let Some(choices) = schema.get("enum").and_then(|value| value.as_array()) {
            let mut choices = choices.clone();
            choices.extend(extra.iter().map(|value| (*value).into()));
            for choice in &choices {
                if !values.contains(choice) {
                    values.push(choice.clone());
                }
            }
            by_source.insert(source.as_str().into(), choices.into());
        }
    }
    schemars::json_schema!({
        "type": "string", "enum": values,
        "x-enum-by-field": {
            "field": "source", "default": TrialSource::default(), "choices": by_source
        }
    })
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for TrialStatusFilter {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TrialStatusFilter".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        filter_schema::<TrialStatus, NciStatusFilter>(&[])
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for TrialPhaseFilter {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TrialPhaseFilter".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        filter_schema::<PhaseFilter, NciPhaseFilter>(
            &PhaseFilter::PRINTED.map(TrialPhase::as_str),
        )
    }
}
biodata::wire_enum! {
    /// Registry funding category facet.
    SponsorType {
        /// National Institutes of Health.
        Nih => "nih",
        /// Other United States federal agency.
        Fed => "fed",
        /// Industry.
        Industry => "industry",
        /// Other sponsor category.
        Other => "other",
    }
}
biodata::wire_enum! {
    /// Sponsor roles a sponsor name may match.
    SponsorRole {
        /// Lead sponsor only.
        Lead => "lead",
        /// Lead sponsor or any collaborator.
        Any => "any",
    }
}
biodata::wire_enum! {
    /// Explicit source phase selection, including the registry's combined phases.
    PhaseFilter {
        /// Early phase 1.
        Early1 => "early-1",
        /// Phase 1.
        One => "1",
        /// Phase 2.
        Two => "2",
        /// Phase 3.
        Three => "3",
        /// Phase 4.
        Four => "4",
        /// Not applicable.
        NotApplicable => "not-applicable",
        /// Combined phase 1 and phase 2.
        OneTwo => "1/2",
        /// Combined phase 2 and phase 3.
        TwoThree => "2/3",
    }
}
impl PhaseFilter {
    /// Every phase as registry rows print it.
    #[cfg(feature = "schema")]
    const PRINTED: [TrialPhase; 6] = [
        TrialPhase::Early1,
        TrialPhase::Phase1,
        TrialPhase::Phase2,
        TrialPhase::Phase3,
        TrialPhase::Phase4,
        TrialPhase::NotApplicable,
    ];
    /// The filter a printed registry phase reads back as.
    const fn printed(phase: TrialPhase) -> Self {
        match phase {
            TrialPhase::Early1 => Self::Early1,
            TrialPhase::Phase1 => Self::One,
            TrialPhase::Phase2 => Self::Two,
            TrialPhase::Phase3 => Self::Three,
            TrialPhase::Phase4 => Self::Four,
            TrialPhase::NotApplicable => Self::NotApplicable,
        }
    }
    /// Exact registry phases required together.
    pub fn phases(self) -> &'static [crate::TrialPhase] {
        use crate::TrialPhase as P;
        match self {
            Self::Early1 => &[P::Early1],
            Self::One => &[P::Phase1],
            Self::Two => &[P::Phase2],
            Self::Three => &[P::Phase3],
            Self::Four => &[P::Phase4],
            Self::NotApplicable => &[P::NotApplicable],
            Self::OneTwo => &[P::Phase1, P::Phase2],
            Self::TwoThree => &[P::Phase2, P::Phase3],
        }
    }
}
