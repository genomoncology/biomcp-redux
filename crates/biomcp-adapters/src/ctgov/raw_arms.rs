//! Search projection of registry arms, using the registry's intervention links.
use crate::{Refusal, mapped::Problems, raw::Field};
use biodata::{Text, TrialArm};
use serde::Deserialize;

use super::super::{detail::arms::Arm, refusals::records};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct DesignInfo {
    primary_purpose: Option<Field<Box<str>>>,
}

pub(super) fn primary_purpose(
    design: Option<Field<DesignInfo>>,
    problems: &mut Problems,
) -> Option<Box<str>> {
    let design = problems.field("designInfo", design)?;
    problems.native_text("designInfo.primaryPurpose", design.primary_purpose)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Interventions {
    pub(super) interventions: Option<Field<Vec<Field<Intervention>>>>,
    arm_groups: Option<Field<Vec<Field<Arm>>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Intervention {
    pub(super) name: Option<Field<Text>>,
    arm_group_labels: Option<Field<Vec<Field<Box<str>>>>>,
    other_names: Option<Field<Vec<Field<Box<str>>>>>,
}

impl Interventions {
    pub(super) fn rows(
        self,
        arms_for: Option<&Text>,
        problems: &mut Problems,
        refusals: &mut Refusal,
    ) -> (Vec<Text>, Box<[TrialArm]>) {
        let mut matched = Matched::default();
        let names = records(problems, refusals, "interventions", self.interventions)
            .into_vec()
            .into_iter()
            .filter_map(|intervention| {
                let name = problems.field("interventions.name", intervention.name);
                if let Some(term) = arms_for {
                    let aliases =
                        problems.native_texts("interventions.otherNames", intervention.other_names);
                    let labels = problems.native_texts(
                        "interventions.armGroupLabels",
                        intervention.arm_group_labels,
                    );
                    if name
                        .iter()
                        .map(Text::as_str)
                        .chain(aliases.iter().map(AsRef::as_ref))
                        .any(|value| same_name(value, term.as_str()))
                    {
                        matched.labels.extend(labels);
                        matched.names.extend(name.iter().cloned());
                    }
                }
                name
            })
            .collect();
        let arms = arms_for.map_or_else(Box::default, |term| {
            records(problems, refusals, "armGroups", self.arm_groups)
                .into_vec()
                .into_iter()
                .map(|arm| arm.row(problems))
                .filter(|arm| matched.includes(arm, term))
                .collect()
        });
        (names, arms)
    }
}

#[derive(Default)]
struct Matched {
    labels: Vec<Box<str>>,
    names: Vec<Text>,
}

impl Matched {
    fn includes(&self, arm: &TrialArm, term: &Text) -> bool {
        arm.label
            .as_ref()
            .is_some_and(|label| self.labels.contains(label))
            || arm.intervention_names.iter().any(|name| {
                // The registry prefixes an arm's intervention names with their intervention type.
                let bare = name
                    .split_once(": ")
                    .map_or(name.as_ref(), |(_, bare)| bare);
                same_name(name, term.as_str())
                    || same_name(bare, term.as_str())
                    || self
                        .names
                        .iter()
                        .any(|matched| same_name(bare, matched.as_str()))
            })
    }
}

fn same_name(left: &str, right: &str) -> bool {
    left.to_lowercase() == right.to_lowercase()
}
