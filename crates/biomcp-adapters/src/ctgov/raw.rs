//! Checked native registry replies and their small discovery projection.
use crate::{
    mapped::{Mapped, Problems},
    raw::Field,
};
use biodata::{NctId, PartialDate, Text, TrialPhase, TrialStatus};
use biomcp_types::Source;

use super::detail::RawDate;
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::TrialHeader;
use serde::Deserialize;
#[path = "raw_dates.rs"]
mod dates;
#[path = "raw_gene.rs"]
mod gene_context;
use dates::Dates;
#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct FirstPage(pub(super) RawPage);
impl Reply for FirstPage {
    fn discovery(&self) -> Option<biomcp_fetch::Identities> {
        self.0.discovery()
    }
    fn check(&self) -> Result<(), BadReply> {
        self.0.check()?;
        self.0.total_count.ok_or(BadReply).map(|_| ())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Version {
    pub(super) data_timestamp: Option<Field<Box<str>>>,
}
impl Reply for Version {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RawPage {
    pub(super) total_count: Option<u64>,
    pub(super) studies: Option<Vec<Field<Study>>>,
    pub(super) next_page_token: Option<Text>,
}
impl Reply for RawPage {
    fn discovery(&self) -> Option<biomcp_fetch::Identities> {
        biomcp_fetch::Identities::bounded(
            "ctgov-page",
            vec![self.next_page_token.as_ref()?.as_str().to_owned()],
        )
    }
    fn input_parameter() -> Option<(&'static str, &'static str)> {
        Some(("ctgov-page", "pageToken"))
    }
    fn check(&self) -> Result<(), BadReply> {
        // The registry omits studies for a successful zero count. An absent array
        // without that explicit zero is an incomplete reply, never an empty page.
        let studies = self.studies.as_deref().unwrap_or_default();
        let mut ids = std::collections::BTreeSet::new();
        if (self.studies.is_none() && self.total_count != Some(0))
            || studies.len() > 100
            || self
                .total_count
                .is_some_and(|total| studies.len() as u64 > total)
            || studies
                .iter()
                .filter_map(Field::as_ref)
                .any(|r| !ids.insert(r.protocol_section.identification_module.nct_id))
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Study {
    pub(super) protocol_section: Protocol,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Protocol {
    identification_module: Identification,
    status_module: Option<Field<Status>>,
    design_module: Option<Field<Design>>,
    sponsor_collaborators_module: Option<Field<Sponsor>>,
    conditions_module: Option<Field<Conditions>>,
    arms_interventions_module: Option<Field<Interventions>>,
    contacts_locations_module: Option<Field<CountryLocations>>,
    description_module: Option<Field<Description>>,
    eligibility_module: Option<Field<Eligibility>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
    brief_title: Option<Field<Text>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    overall_status: Option<Field<TrialStatus>>,
    start_date_struct: Option<Field<RawDate>>,
    primary_completion_date_struct: Option<Field<RawDate>>,
    last_update_post_date_struct: Option<Field<Updated>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sponsor {
    lead_sponsor: Option<Field<SponsorName>>,
}
#[derive(Deserialize)]
struct SponsorName {
    name: Option<Field<Text>>,
}
#[derive(Deserialize)]
struct Updated {
    date: Option<Field<PartialDate>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Design {
    phases: Option<Field<Vec<Field<TrialPhase>>>>,
}
#[derive(Deserialize)]
struct Conditions {
    conditions: Option<Field<Vec<Field<Text>>>>,
    keywords: Option<Field<Vec<Field<Box<str>>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Description {
    brief_summary: Option<Field<Box<str>>>,
    detailed_description: Option<Field<Box<str>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Eligibility {
    eligibility_criteria: Option<Field<Box<str>>>,
}
#[derive(Deserialize)]
struct Interventions {
    interventions: Option<Field<Vec<Field<Intervention>>>>,
}
#[derive(Deserialize)]
struct Intervention {
    name: Option<Field<Text>>,
}
#[derive(Deserialize)]
struct CountryLocations {
    locations: Option<Field<Vec<Field<CountryLocation>>>>,
}
#[derive(Deserialize)]
struct CountryLocation {
    country: Option<Field<Box<str>>>,
}
impl Protocol {
    pub(super) fn has_country(&self, countries: &[Box<str>]) -> bool {
        self.contacts_locations_module
            .as_ref()
            .and_then(Field::as_ref)
            .and_then(|module| module.locations.as_ref())
            .and_then(Field::as_ref)
            .into_iter()
            .flatten()
            .filter_map(Field::as_ref)
            .filter_map(|site| site.country.as_ref().and_then(Field::as_ref))
            .any(|country| countries.contains(country))
    }

    pub(super) fn contains_condition_words(&self, condition: &str) -> bool {
        let requested = words(condition);
        let present: Vec<String> = self
            .conditions_module
            .as_ref()
            .and_then(Field::as_ref)
            .and_then(|v| v.conditions.as_ref())
            .and_then(Field::as_ref)
            .into_iter()
            .flatten()
            .filter_map(Field::as_ref)
            .flat_map(|value| words(value.as_str()))
            .collect();
        requested.iter().all(|word| present.contains(word))
    }

    pub(super) fn header(self) -> Result<Mapped<TrialHeader>, BadReply> {
        let mut refusals = crate::Refusal::None;
        let mut problems = Problems {
            source: Source::ClinicalTrialsGov,
            record: crate::mapped::record(self.identification_module.nct_id),
            list: Vec::new(),
        };
        let dates = Dates::read(self.status_module, &mut problems);
        let sponsor = problems
            .field(
                "sponsorCollaboratorsModule",
                self.sponsor_collaborators_module,
            )
            .and_then(|s| problems.field("leadSponsor", s.lead_sponsor))
            .and_then(|s| problems.field("leadSponsor.name", s.name));
        let phases = problems
            .field("designModule", self.design_module)
            .map_or_else(Box::default, |d| problems.fields("phases", d.phases));
        let value = TrialHeader {
            nct_id: self.identification_module.nct_id,
            url: Text::new(&format!(
                "https://clinicaltrials.gov/study/{}",
                self.identification_module.nct_id
            ))
            .map_err(|_| BadReply)?,
            title: problems.required_field("briefTitle", self.identification_module.brief_title),
            status: dates.status,
            phases,
            conditions: names(
                problems
                    .field("conditionsModule", self.conditions_module)
                    .map_or_else(Vec::new, |v| {
                        problems.fields("conditions", v.conditions).into_vec()
                    }),
            ),
            interventions: names(
                problems
                    .field("armsInterventionsModule", self.arms_interventions_module)
                    .map_or_else(Vec::new, |v| {
                        super::refusals::records(
                            &mut problems,
                            &mut refusals,
                            "interventions",
                            v.interventions,
                        )
                        .into_vec()
                    })
                    .into_iter()
                    .filter_map(|v| problems.field("interventions.name", v.name))
                    .collect(),
            ),
            sponsor,
            start: dates.start,
            primary_completion: dates.primary_completion,
            updated: dates.updated,
        };
        Ok(Mapped {
            refusals,
            value,
            problems: problems.list,
        })
    }
}

// Header lists stop here; get and its sections retain the full source record.
const HEADER_NAMES: usize = 3;
fn names(mut values: Vec<Text>) -> biomcp_types::Capped<Text> {
    let total = values.len() as u64;
    values.truncate(HEADER_NAMES);
    biomcp_types::Capped {
        items: values.into(),
        total,
    }
}

fn words(value: &str) -> Vec<String> {
    value
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}
