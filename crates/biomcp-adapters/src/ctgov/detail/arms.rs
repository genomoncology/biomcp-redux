//! Registered arms and interventions retain their original relationships.
use super::{Arms, Field, Problems, TrialArm, TrialIntervention};

impl Arms {
    pub(super) fn rows(
        self,
        problems: &mut Problems,
        refusals: &mut crate::Refusal,
    ) -> (Box<[TrialArm]>, Box<[TrialIntervention]>) {
        (
            super::super::refusals::records(problems, refusals, "armGroups", self.arm_groups)
                .into_vec()
                .into_iter()
                .map(|arm| arm.row(problems))
                .collect(),
            super::super::refusals::records(
                problems,
                refusals,
                "interventions",
                self.interventions,
            )
            .into_vec()
            .into_iter()
            .map(|v| TrialIntervention {
                name: problems.required_native_text("interventions.name", v.name),
                kind: problems.field("interventions.type", v.kind),
                description: problems.field("armsInterventions.description", v.description),
                arm_group_labels: problems
                    .native_texts("interventions.armGroupLabels", v.arm_group_labels),
                other_names: problems.native_texts("interventions.otherNames", v.other_names),
            })
            .collect(),
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::ctgov) struct Arm {
    label: Option<Field<Box<str>>>,
    #[serde(rename = "type")]
    kind: Option<Field<biodata::TrialArmKind>>,
    description: Option<Field<biodata::Markdown>>,
    intervention_names: Option<Field<Vec<Field<Box<str>>>>>,
}

impl Arm {
    pub(in crate::ctgov) fn row(self, problems: &mut Problems) -> TrialArm {
        TrialArm {
            label: problems.required_native_text("armGroups.label", self.label),
            kind: problems.field("armGroups.type", self.kind),
            description: problems.field("armsInterventions.description", self.description),
            intervention_names: problems
                .native_texts("armGroups.interventionNames", self.intervention_names),
        }
    }
}
