//! One registry record request with a typed protocol projection, checked against its requested identity before caching.
mod sites;
use biodata::{
    Markdown, NctId, StudyType, TrialAgeBound, TrialArm, TrialCard, TrialDate, TrialEligibility,
    TrialIntervention, TrialOutcome, TrialOutcomeKind, TrialPhase, TrialSex, TrialStatus,
};
use biomcp_fetch::{BadReply, Freshness, Leg, Reply, ReplyMetadata, RequestError};
use serde::{Deserialize, Serialize};
/// Whole source protocol projection; query owns section paging.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Detail {
    /// Whole sites with attached contacts.
    pub locations: Box<[biodata::TrialLocation]>,
    /// Registry-wide central contacts.
    pub contacts: Box<[biodata::TrialContact]>,
    /// Overall officials, separate from contacts.
    pub officials: Box<[biodata::TrialOfficial]>,
    /// Whole card.
    pub card: TrialCard,
    /// Complete source eligibility module.
    pub eligibility: Option<TrialEligibility>,
    /// All outcomes in native collection and record order.
    pub outcomes: Box<[TrialOutcome]>,
    /// Whole source arm rows.
    pub arms: Box<[TrialArm]>,
    /// Whole source intervention rows.
    pub interventions: Box<[TrialIntervention]>,
}
/// Read one native protocol and preserve that reply's timestamp.
pub fn get(leg: &Leg<'_>, id: NctId) -> Result<(Detail, ReplyMetadata), RequestError> {
    let endpoint = super::SPEC
        .get("/studies")
        .fresh(Freshness::Days(1))
        .with_segments(&[biomcp_fetch::PathSegment::new(&id.to_string())?]);
    let (raw, metadata) = leg.json_received_checked::<Study>(&endpoint, &[], |raw| {
        if raw.protocol_section.identification_module.nct_id == id {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    Ok((raw.protocol_section.detail(), metadata))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Study {
    protocol_section: Protocol,
}
impl Reply for Study {
    fn check(&self) -> Result<(), BadReply> {
        let p = &self.protocol_section;
        if p.identification_module.brief_title.trim().is_empty() {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Protocol {
    identification_module: Identification,
    status_module: Status,
    design_module: Design,
    description_module: Option<Description>,
    conditions_module: Option<Conditions>,
    sponsor_collaborators_module: Option<Sponsor>,
    eligibility_module: Option<Eligibility>,
    outcomes_module: Option<Outcomes>,
    arms_interventions_module: Option<Arms>,
    contacts_locations_module: Option<sites::ContactsLocations>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
    brief_title: Box<str>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    overall_status: TrialStatus,
    last_update_post_date_struct: RawDate,
    start_date_struct: Option<RawDate>,
    primary_completion_date_struct: Option<RawDate>,
    completion_date_struct: Option<RawDate>,
}
#[derive(Deserialize)]
struct RawDate {
    date: biodata::PartialDate,
    #[serde(rename = "type")]
    kind: Option<biodata::TrialDateKind>,
}
impl RawDate {
    fn mapped(self) -> TrialDate {
        TrialDate {
            date: self.date,
            kind: self.kind,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Design {
    study_type: StudyType,
    phases: Option<Box<[TrialPhase]>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Description {
    brief_summary: Option<Markdown>,
}
#[derive(Deserialize)]
struct Conditions {
    conditions: Option<Box<[Box<str>]>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sponsor {
    lead_sponsor: Option<Name>,
}
#[derive(Deserialize)]
struct Name {
    name: Box<str>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Eligibility {
    minimum_age: Option<Box<str>>,
    maximum_age: Option<Box<str>>,
    sex: Option<TrialSex>,
    healthy_volunteers: Option<bool>,
    eligibility_criteria: Option<Markdown>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Outcomes {
    primary_outcomes: Option<Vec<Outcome>>,
    secondary_outcomes: Option<Vec<Outcome>>,
    other_outcomes: Option<Vec<Outcome>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Outcome {
    measure: Box<str>,
    description: Option<Markdown>,
    time_frame: Option<Box<str>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Arms {
    arm_groups: Option<Box<[Arm]>>,
    interventions: Option<Box<[Intervention]>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Arm {
    label: Box<str>,
    #[serde(rename = "type")]
    kind: Option<biodata::TrialArmKind>,
    description: Option<Markdown>,
    intervention_names: Option<Box<[Box<str>]>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Intervention {
    name: Box<str>,
    #[serde(rename = "type")]
    kind: Option<biodata::TrialInterventionKind>,
    description: Option<Markdown>,
    arm_group_labels: Option<Box<[Box<str>]>>,
    other_names: Option<Box<[Box<str>]>>,
}
impl Protocol {
    fn detail(self) -> Detail {
        let arms = self.arms_interventions_module;
        let (locations, contacts, officials) = self.contacts_locations_module.map_or_else(
            || (Box::default(), Box::default(), Box::default()),
            sites::ContactsLocations::rows,
        );
        let card = TrialCard {
            nct_id: self.identification_module.nct_id,
            url: format!(
                "https://clinicaltrials.gov/study/{}",
                self.identification_module.nct_id
            )
            .into(),
            title: self.identification_module.brief_title,
            status: self.status_module.overall_status,
            phases: self.design_module.phases.unwrap_or_default(),
            study_type: self.design_module.study_type,
            conditions: self
                .conditions_module
                .and_then(|v| v.conditions)
                .unwrap_or_default(),
            interventions: arms.as_ref().map_or_else(Box::default, |v| {
                v.interventions
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .map(|v| v.name.clone())
                    .collect()
            }),
            sponsor: self
                .sponsor_collaborators_module
                .and_then(|s| s.lead_sponsor.map(|s| s.name)),
            start: self.status_module.start_date_struct.map(RawDate::mapped),
            primary_completion: self
                .status_module
                .primary_completion_date_struct
                .map(RawDate::mapped),
            completion: self
                .status_module
                .completion_date_struct
                .map(RawDate::mapped),
            updated: self.status_module.last_update_post_date_struct.mapped(),
            brief_summary: self.description_module.and_then(|d| d.brief_summary),
        };
        let (arm_rows, interventions) =
            arms.map_or_else(|| (Box::default(), Box::default()), Arms::rows);
        Detail {
            locations,
            contacts,
            officials,
            card,
            eligibility: self.eligibility_module.map(|v| TrialEligibility {
                minimum_age: age(v.minimum_age),
                maximum_age: age(v.maximum_age),
                sex: v.sex,
                healthy_volunteers: v.healthy_volunteers,
                criteria: v.eligibility_criteria,
            }),
            outcomes: self
                .outcomes_module
                .map_or_else(Box::default, Outcomes::rows),
            arms: arm_rows,
            interventions,
        }
    }
}
impl Outcomes {
    fn rows(self) -> Box<[TrialOutcome]> {
        [
            (
                TrialOutcomeKind::Primary,
                self.primary_outcomes.unwrap_or_default(),
            ),
            (
                TrialOutcomeKind::Secondary,
                self.secondary_outcomes.unwrap_or_default(),
            ),
            (
                TrialOutcomeKind::Other,
                self.other_outcomes.unwrap_or_default(),
            ),
        ]
        .into_iter()
        .flat_map(|(kind, rows)| {
            rows.into_iter().map(move |v| TrialOutcome {
                kind,
                measure: v.measure,
                description: v.description,
                time_frame: v.time_frame,
            })
        })
        .collect()
    }
}
fn age(value: Option<Box<str>>) -> TrialAgeBound {
    value.map_or(TrialAgeBound::NoLimit, |raw| TrialAgeBound::Source { raw })
}

impl Arms {
    fn rows(self) -> (Box<[TrialArm]>, Box<[TrialIntervention]>) {
        (
            self.arm_groups
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .map(|v| TrialArm {
                    label: v.label,
                    kind: v.kind,
                    description: v.description,
                    intervention_names: v.intervention_names.unwrap_or_default(),
                })
                .collect(),
            self.interventions
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .map(|v| TrialIntervention {
                    name: v.name,
                    kind: v.kind,
                    description: v.description,
                    arm_group_labels: v.arm_group_labels.unwrap_or_default(),
                    other_names: v.other_names.unwrap_or_default(),
                })
                .collect(),
        )
    }
}
