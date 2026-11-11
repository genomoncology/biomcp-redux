//! Structured eligibility fields; a missing age bound is the registry saying no limit.
use super::{Eligibility, Problems};
use crate::raw::Field;
use biodata::{TrialAgeBound, TrialEligibility};

impl Eligibility {
    pub(super) fn mapped(self, problems: &mut Problems) -> TrialEligibility {
        TrialEligibility {
            minimum_age: age("minimumAge", self.minimum_age, problems),
            maximum_age: age("maximumAge", self.maximum_age, problems),
            sex: problems.field("sex", self.sex),
            healthy_volunteers: problems.field("healthyVolunteers", self.healthy_volunteers),
            criteria: problems.field("eligibilityCriteria", self.eligibility_criteria),
        }
    }
}
fn age(
    field: &str,
    value: Option<Field<Box<str>>>,
    problems: &mut Problems,
) -> Option<TrialAgeBound> {
    match value {
        None => Some(TrialAgeBound::NoLimit),
        Some(value) => problems
            .native_text(field, Some(value))
            .map(|raw| TrialAgeBound::Source { raw }),
    }
}
