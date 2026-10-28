//! Checked native registry replies and their small discovery projection.
use biodata::{NctId, PartialDate, Text, TrialPhase, TrialStatus};

use biomcp_fetch::{BadReply, Reply};
use biomcp_types::TrialHeader;
use serde::Deserialize;
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
    pub(super) data_timestamp: Text,
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
    pub(super) studies: Option<Vec<Study>>,
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
    status_module: Status,
    design_module: Design,
    conditions_module: Option<Conditions>,
    arms_interventions_module: Option<Interventions>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
    brief_title: Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Status {
    overall_status: TrialStatus,
    last_update_post_date_struct: Updated,
}
#[derive(Deserialize)]
struct Updated {
    date: PartialDate,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Design {
    phases: Option<Box<[TrialPhase]>>,
}
#[derive(Deserialize)]
struct Conditions {
    conditions: Option<Box<[Text]>>,
}
#[derive(Deserialize)]
struct Interventions {
    interventions: Option<Vec<Intervention>>,
}
#[derive(Deserialize)]
struct Intervention {
    name: Text,
}
impl Protocol {
    pub(super) fn contains_condition_words(&self, condition: &str) -> bool {
        let requested = words(condition);
        let present: Vec<String> = self
            .conditions_module
            .as_ref()
            .and_then(|module| module.conditions.as_deref())
            .unwrap_or_default()
            .iter()
            .flat_map(|value| words(value.as_str()))
            .collect();
        requested.iter().all(|word| present.contains(word))
    }

    pub(super) fn header(self) -> Result<TrialHeader, BadReply> {
        Ok(TrialHeader {
            nct_id: self.identification_module.nct_id,
            url: Text::new(&format!(
                "https://clinicaltrials.gov/study/{}",
                self.identification_module.nct_id
            ))
            .map_err(|_| BadReply)?,
            title: self.identification_module.brief_title,
            status: self.status_module.overall_status,
            phases: self.design_module.phases.unwrap_or_default(),
            conditions: names(
                self.conditions_module
                    .map_or_else(Vec::new, |v| v.conditions.unwrap_or_default().into_vec()),
            ),
            interventions: names(self.arms_interventions_module.map_or_else(Vec::new, |v| {
                v.interventions
                    .unwrap_or_default()
                    .into_iter()
                    .map(|i| i.name)
                    .collect()
            })),
            updated: self.status_module.last_update_post_date_struct.date,
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
