//! One registry record request with a typed protocol projection, checked against its requested identity before caching.
mod dates;
mod publications;
mod sites;
use crate::{mapped::Problems, raw::Field};
use biodata::{
    Markdown, NctId, StudyType, TrialAgeBound, TrialArm, TrialCard, TrialEligibility,
    TrialIntervention, TrialOutcome, TrialOutcomeKind, TrialPhase, TrialSex, TrialStatus,
};
use biomcp_types::Source;

use biomcp_fetch::{BadReply, Freshness, Leg, Reply, ReplyMetadata, RequestError};
use serde::{Deserialize, Serialize};
/// Whole source protocol projection; query owns section paging.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Detail {
    /// Native nested records refused during this checked mapping.
    pub dropped: bool,
    /// Manifest records refused while retaining independent study sections.
    pub documents_dropped: bool,
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
    let mut problems = Problems {
        source: Source::ClinicalTrialsGov,
        record: crate::mapped::record(id),
        list: detail.problems,
    };
    detail.documents = problems
        .field("documentSection", raw.document_section)
        .map_or_else(Box::default, |d| {
            d.mapped(id, &mut problems, &mut detail.documents_dropped)
        });
    detail.dropped |= detail.documents_dropped;
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
    brief_title: Option<Field<Box<str>>>,
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
    fn detail(mut self) -> Detail {
        let mut dropped = false;
        let mut problems = Problems {
            source: Source::ClinicalTrialsGov,
            record: crate::mapped::record(self.identification_module.nct_id),
            list: Vec::new(),
        };
        let arms = problems.field(
            "armsInterventionsModule",
            self.arms_interventions_module.take(),
        );
        let (arm_rows, interventions) = arms.map_or_else(
            || (Box::default(), Box::default()),
            |arms| arms.rows(&mut problems, &mut dropped),
        );
        let (locations, contacts, officials) = problems
            .field(
                "contactsLocationsModule",
                self.contacts_locations_module.take(),
            )
            .map_or_else(
                || (Box::default(), Box::default(), Box::default()),
                |sites| sites.rows(&mut problems, &mut dropped),
            );
        let card = self.card(&interventions, &mut problems);
        Detail {
            references: problems
                .field("referencesModule", self.references_module)
                .map_or_else(Box::default, |r| r.rows(&mut problems, &mut dropped)),
            documents: Box::default(),
            documents_dropped: false,
            locations,
            contacts,
            officials,
            card,
            eligibility: problems
                .field("eligibilityModule", self.eligibility_module)
                .map(|v| v.mapped(&mut problems)),
            outcomes: problems
                .field("outcomesModule", self.outcomes_module)
                .map_or_else(Box::default, |o| o.rows(&mut problems, &mut dropped)),
            problems: problems.list,
            dropped,
            arms: arm_rows,
            interventions,
        }
    }
    fn card(&mut self, interventions: &[TrialIntervention], problems: &mut Problems) -> TrialCard {
        let status = problems
            .field("statusModule", self.status_module.take())
            .unwrap_or_default();
        let design = problems
            .field("designModule", self.design_module.take())
            .unwrap_or_default();
        let updated = RawDate::field(
            "lastUpdatePostDateStruct",
            status.last_update_post_date_struct,
            problems,
        );
        TrialCard {
            nct_id: self.identification_module.nct_id,
            url: format!(
                "https://clinicaltrials.gov/study/{}",
                self.identification_module.nct_id
            )
            .into(),
            title: problems
                .required_native_text("briefTitle", self.identification_module.brief_title.take()),
            status: problems.required_field("overallStatus", status.overall_status),
            phases: problems.fields("phases", design.phases),
            study_type: problems.required_field("studyType", design.study_type),
            conditions: problems
                .field("conditionsModule", self.conditions_module.take())
                .map_or_else(Box::default, |v| {
                    problems.native_texts("conditions", v.conditions)
                }),
            interventions: interventions
                .iter()
                .filter_map(|v| v.name.clone())
                .collect(),
            sponsor: problems
                .field(
                    "sponsorCollaboratorsModule",
                    self.sponsor_collaborators_module.take(),
                )
                .and_then(|s| s.name(problems)),
            start: RawDate::field("startDateStruct", status.start_date_struct, problems),
            primary_completion: RawDate::field(
                "primaryCompletionDateStruct",
                status.primary_completion_date_struct,
                problems,
            ),
            completion: RawDate::field(
                "completionDateStruct",
                status.completion_date_struct,
                problems,
            ),
            updated: updated.as_ref().map(|date| date.date),
            updated_kind: updated.and_then(|date| date.kind),
            brief_summary: problems
                .field("descriptionModule", self.description_module.take())
                .and_then(|d| problems.field("briefSummary", d.brief_summary)),
        }
    }
}
impl Sponsor {
    fn name(self, problems: &mut Problems) -> Option<Box<str>> {
        problems
            .field("leadSponsor", self.lead_sponsor)
            .and_then(|s| problems.native_text("leadSponsor.name", s.name))
    }
}
impl Eligibility {
    fn mapped(self, problems: &mut Problems) -> TrialEligibility {
        TrialEligibility {
            minimum_age: age("minimumAge", self.minimum_age, problems),
            maximum_age: age("maximumAge", self.maximum_age, problems),
            sex: problems.field("sex", self.sex),
            healthy_volunteers: problems.field("healthyVolunteers", self.healthy_volunteers),
            criteria: problems.field("eligibilityCriteria", self.eligibility_criteria),
        }
    }
}
impl Outcomes {
    fn rows(self, problems: &mut Problems, dropped: &mut bool) -> Box<[TrialOutcome]> {
        [
            (
                TrialOutcomeKind::Primary,
                problems.record_fields("primaryOutcomes", self.primary_outcomes, dropped),
            ),
            (
                TrialOutcomeKind::Secondary,
                problems.record_fields("secondaryOutcomes", self.secondary_outcomes, dropped),
            ),
            (
                TrialOutcomeKind::Other,
                problems.record_fields("otherOutcomes", self.other_outcomes, dropped),
            ),
        ]
        .into_iter()
        .flat_map(|(kind, rows)| rows.into_vec().into_iter().map(move |v| (kind, v)))
        .map(|(kind, v)| TrialOutcome {
            kind,
            measure: problems.required_native_text("outcomes.measure", v.measure),
            description: problems.field("outcomes.description", v.description),
            time_frame: problems.native_text("outcomes.timeFrame", v.time_frame),
        })
        .collect()
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

impl Arms {
    fn rows(
        self,
        problems: &mut Problems,
        dropped: &mut bool,
    ) -> (Box<[TrialArm]>, Box<[TrialIntervention]>) {
        (
            problems
                .record_fields("armGroups", self.arm_groups, dropped)
                .into_vec()
                .into_iter()
                .map(|v| TrialArm {
                    label: problems.required_native_text("armGroups.label", v.label),
                    kind: problems.field("armGroups.type", v.kind),
                    description: problems.field("armsInterventions.description", v.description),
                    intervention_names: problems
                        .native_texts("armGroups.interventionNames", v.intervention_names),
                })
                .collect(),
            problems
                .record_fields("interventions", self.interventions, dropped)
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
