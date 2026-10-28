//! One registry record request with a typed protocol projection, checked against its requested identity before caching.
mod publications;
mod sites;
use crate::{mapped::Problems, raw::Field};
use biomcp_types::Source;
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
    /// Rejected nonidentity fields retained with this snapshot.
    pub problems: Vec<biomcp_types::Problem>,
    /// Whole sites with attached contacts.
    pub locations: Box<[biodata::TrialLocation]>,
    /// Registry-wide central contacts.
    pub contacts: Box<[biodata::TrialContact]>,
    /// Overall officials, separate from contacts.
    pub officials: Box<[biodata::TrialOfficial]>,
    /// Native publication records.
    pub references: Box<[biodata::TrialReference]>,
    /// Validated posted document manifest.
    pub documents: Box<[biodata::TrialDocument]>,
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
    let id = raw.protocol_section.identification_module.nct_id;
    let mut detail = raw.protocol_section.detail();
    let mut problems = Problems { source: Source::ClinicalTrialsGov, record: crate::mapped::record(id), list: detail.problems };
    detail.documents = problems.field("documentSection", raw.document_section)
        .map_or_else(Box::default, |d| d.mapped(id, &mut problems));
    detail.problems = problems.list;
    Ok((detail, metadata))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Study {
    protocol_section: Protocol,
    document_section: Option<Field<publications::Documents>>,
}
impl Reply for Study {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Protocol {
    references_module: Option<Field<publications::References>>,
    identification_module: Identification,
    status_module: Option<Field<Status>>,
    design_module: Option<Field<Design>>,
    description_module: Option<Field<Description>>,
    conditions_module: Option<Field<Conditions>>,
    sponsor_collaborators_module: Option<Field<Sponsor>>,
    eligibility_module: Option<Field<Eligibility>>,
    outcomes_module: Option<Field<Outcomes>>,
    arms_interventions_module: Option<Field<Arms>>,
    contacts_locations_module: Option<Field<sites::ContactsLocations>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
    brief_title: Option<Field<biodata::Text>>,
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    overall_status: Option<Field<TrialStatus>>,
    last_update_post_date_struct: Option<Field<RawDate>>,
    start_date_struct: Option<Field<RawDate>>,
    primary_completion_date_struct: Option<Field<RawDate>>,
    completion_date_struct: Option<Field<RawDate>>,
}
#[derive(Deserialize)]
struct RawDate {
    date: Option<Field<biodata::PartialDate>>,
    #[serde(rename = "type")]
    kind: Option<Field<biodata::TrialDateKind>>,
}
impl RawDate {
    fn mapped(self, field: &str, problems: &mut Problems) -> Option<TrialDate> {
        let path = format!("{field}.date");
        let date = problems.field(&path, self.date);
        let date = problems.required(&path, date)?;
        Some(TrialDate {
            date,
            kind: problems.field(&format!("{field}.type"), self.kind),
        })
    }
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Design {
    study_type: Option<Field<StudyType>>,
    phases: Option<Field<Vec<Field<TrialPhase>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Description {
    brief_summary: Option<Field<Markdown>>,
}
#[derive(Deserialize)]
struct Conditions {
    conditions: Option<Field<Vec<Field<Box<str>>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sponsor {
    lead_sponsor: Option<Field<Name>>,
}
#[derive(Deserialize)]
struct Name {
    name: Option<Field<Box<str>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Eligibility {
    minimum_age: Option<Field<Box<str>>>,
    maximum_age: Option<Field<Box<str>>>,
    sex: Option<Field<TrialSex>>,
    healthy_volunteers: Option<Field<bool>>,
    eligibility_criteria: Option<Field<Markdown>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Outcomes {
    primary_outcomes: Option<Field<Vec<Field<Outcome>>>>,
    secondary_outcomes: Option<Field<Vec<Field<Outcome>>>>,
    other_outcomes: Option<Field<Vec<Field<Outcome>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Outcome {
    measure: Option<Field<Box<str>>>,
    description: Option<Field<Markdown>>,
    time_frame: Option<Field<Box<str>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Arms {
    arm_groups: Option<Field<Vec<Field<Arm>>>>,
    interventions: Option<Field<Vec<Field<Intervention>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Arm {
    label: Option<Field<Box<str>>>,
    #[serde(rename = "type")]
    kind: Option<Field<biodata::TrialArmKind>>,
    description: Option<Field<Markdown>>,
    intervention_names: Option<Field<Vec<Field<Box<str>>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Intervention {
    name: Option<Field<Box<str>>>,
    #[serde(rename = "type")]
    kind: Option<Field<biodata::TrialInterventionKind>>,
    description: Option<Field<Markdown>>,
    arm_group_labels: Option<Field<Vec<Field<Box<str>>>>>,
    other_names: Option<Field<Vec<Field<Box<str>>>>>,
}
impl Protocol {
    fn detail(self) -> Detail {
        let mut problems = Problems { source: Source::ClinicalTrialsGov, record: crate::mapped::record(self.identification_module.nct_id), list: Vec::new() };
        let status = problems.field("statusModule", self.status_module).unwrap_or_default();
        let design = problems.field("designModule", self.design_module).unwrap_or_default();
        let arms = problems.field("armsInterventionsModule", self.arms_interventions_module);
        let (arm_rows, interventions) = arms.map_or_else(|| (Box::default(), Box::default()), |arms| arms.rows(&mut problems));
        let (locations, contacts, officials) = problems.field("contactsLocationsModule", self.contacts_locations_module).map_or_else(
            || (Box::default(), Box::default(), Box::default()),
            |sites| sites.rows(&mut problems),
        );
        let card = TrialCard {
            nct_id: self.identification_module.nct_id,
            url: format!(
                "https://clinicaltrials.gov/study/{}",
                self.identification_module.nct_id
            )
            .into(),
            title: problems.field("briefTitle", self.identification_module.brief_title).map(|v| v.as_str().into()),
            status: { let value = problems.field("overallStatus", status.overall_status); problems.required("overallStatus", value) },
            phases: problems.fields("phases", design.phases),
            study_type: problems.field("studyType", design.study_type),
            conditions: problems.field("conditionsModule", self.conditions_module)
                .map_or_else(Box::default, |v| problems.fields("conditions", v.conditions)),
            interventions: interventions.iter().filter_map(|v| v.name.clone()).collect(),
            sponsor: problems.field("sponsorCollaboratorsModule", self.sponsor_collaborators_module)
                .and_then(|s| problems.field("leadSponsor", s.lead_sponsor))
                .and_then(|s| problems.field("leadSponsor.name", s.name)),
            start: problems.field("startDateStruct", status.start_date_struct).and_then(|d| d.mapped("startDateStruct", &mut problems)),
            primary_completion: problems.field("primaryCompletionDateStruct", status.primary_completion_date_struct).and_then(|d| d.mapped("primaryCompletionDateStruct", &mut problems)),
            completion: problems.field("completionDateStruct", status.completion_date_struct).and_then(|d| d.mapped("completionDateStruct", &mut problems)),
            updated: problems.field("lastUpdatePostDateStruct", status.last_update_post_date_struct).and_then(|d| d.mapped("lastUpdatePostDateStruct", &mut problems)),
            brief_summary: problems.field("descriptionModule", self.description_module).and_then(|d| problems.field("briefSummary", d.brief_summary)),
        };
        Detail {
            references: problems.field("referencesModule", self.references_module)
                .map_or_else(Box::default, |r| r.rows(&mut problems)),
            documents: Box::default(),
            locations,
            contacts,
            officials,
            card,
            eligibility: problems.field("eligibilityModule", self.eligibility_module).map(|v| TrialEligibility {
                minimum_age: age(problems.field("minimumAge", v.minimum_age)),
                maximum_age: age(problems.field("maximumAge", v.maximum_age)),
                sex: problems.field("sex", v.sex),
                healthy_volunteers: problems.field("healthyVolunteers", v.healthy_volunteers),
                criteria: problems.field("eligibilityCriteria", v.eligibility_criteria),
            }),
            outcomes: problems.field("outcomesModule", self.outcomes_module).map_or_else(Box::default, |o| o.rows(&mut problems)),
            problems: problems.list,
            arms: arm_rows,
            interventions,
        }
    }
}
impl Outcomes {
    fn rows(self, problems: &mut Problems) -> Box<[TrialOutcome]> {
        [
            (
                TrialOutcomeKind::Primary,
                problems.fields("primaryOutcomes", self.primary_outcomes),
            ),
            (
                TrialOutcomeKind::Secondary,
                problems.fields("secondaryOutcomes", self.secondary_outcomes),
            ),
            (
                TrialOutcomeKind::Other,
                problems.fields("otherOutcomes", self.other_outcomes),
            ),
        ]
        .into_iter()
        .flat_map(|(kind, rows)| {
            rows.into_vec().into_iter().map(move |v| (kind, v))
        })
        .map(|(kind, v)| TrialOutcome {
                kind,
                measure: problems.field("outcomes.measure", v.measure),
                description: problems.field("outcomes.description", v.description),
                time_frame: problems.field("outcomes.timeFrame", v.time_frame),
            })
        .collect()
    }
}
fn age(value: Option<Box<str>>) -> TrialAgeBound {
    value.map_or(TrialAgeBound::NoLimit, |raw| TrialAgeBound::Source { raw })
}

impl Arms {
    fn rows(self, problems: &mut Problems) -> (Box<[TrialArm]>, Box<[TrialIntervention]>) {
        (
            problems.fields("armGroups", self.arm_groups)
                .into_vec()
                .into_iter()
                .map(|v| TrialArm {
                    label: problems.field("armGroups.label", v.label),
                    kind: problems.field("armGroups.type", v.kind),
                    description: problems.field("armsInterventions.description", v.description),
                    intervention_names: problems.fields("armGroups.interventionNames", v.intervention_names),
                })
                .collect(),
            problems.fields("interventions", self.interventions)
                .into_vec()
                .into_iter()
                .map(|v| TrialIntervention {
                    name: problems.field("interventions.name", v.name),
                    kind: problems.field("interventions.type", v.kind),
                    description: problems.field("armsInterventions.description", v.description),
                    arm_group_labels: problems.fields("interventions.armGroupLabels", v.arm_group_labels),
                    other_names: problems.fields("interventions.otherNames", v.other_names),
                })
                .collect(),
        )
    }
}
