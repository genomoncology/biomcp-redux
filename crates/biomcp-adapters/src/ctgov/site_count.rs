//! Facilities counted over every page of a registry match, each name exactly as registered.
use biodata::{NctId, Text};
use biomcp_fetch::{Arg, BadReply, Input, Leg, Reply, RequestError};
use biomcp_types::{Source, TrialSearch, TrialSiteCount, TrialSiteCounts};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// Rows per request; the registry served all sites of 1,000 trials in one reply (live 2026-11-17).
const PAGE_SIZE: usize = 1000;
const FIELDS: &str = "NCTId,LocationFacility,LocationCity,LocationCountry";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SitePage {
    total_count: Option<u64>,
    studies: Option<Vec<Study>>,
    next_page_token: Option<Text>,
}
impl Reply for SitePage {
    fn check(&self) -> Result<(), BadReply> {
        // Only an explicit zero total may leave out the study list.
        let studies = self.studies.as_deref().unwrap_or_default();
        if (self.studies.is_none() && self.total_count != Some(0)) || studies.len() > PAGE_SIZE {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Study {
    protocol_section: Protocol,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Protocol {
    identification_module: Identification,
    contacts_locations_module: Option<Locations>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
}
#[derive(Deserialize)]
struct Locations {
    locations: Option<Vec<Site>>,
}
#[derive(Deserialize)]
struct Site {
    facility: Option<Box<str>>,
    city: Option<Box<str>>,
    country: Option<Box<str>>,
}
type Key = (Box<str>, Option<Box<str>>, Option<Box<str>>);
impl Study {
    /// Each named site once per trial, however often the trial lists it.
    fn sites(self) -> BTreeSet<Key> {
        self.protocol_section
            .contacts_locations_module
            .and_then(|module| module.locations)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|site| site.facility.map(|name| (name, site.city, site.country)))
            .collect()
    }
}

/// Count the facilities of every matched trial and keep the `limit` most.
pub fn site_count(
    leg: &Leg<'_>,
    query: &TrialSearch,
    gene: Option<&str>,
    conditions: &[Text],
) -> Result<TrialSiteCounts, RequestError> {
    let mut params = super::parameters(query, gene);
    if query.condition_id().is_some()
        && let Some(condition) = conditions.first()
    {
        params.insert("query.cond".into(), format!("({condition})"));
    }
    params.insert("fields".into(), FIELDS.into());
    params.insert("pageSize".into(), PAGE_SIZE.to_string());
    let (counts, matched, read) = read_pages(leg, params)?;
    let facilities_total = counts.len() as u64;
    let mut rows: Vec<_> = counts.into_iter().collect();
    // The key order already sorts by name, city and country; a stable sort keeps it within a count.
    rows.sort_by_key(|(_, trials)| std::cmp::Reverse(*trials));
    rows.truncate(query.limit() as usize);
    Ok(TrialSiteCounts {
        trials_matched: matched,
        trials_read: read,
        facilities_total,
        facilities: rows
            .into_iter()
            .map(|((facility, city, country), trials)| TrialSiteCount {
                facility,
                city,
                country,
                trials,
            })
            .collect(),
    })
}

/// Each site's trial count, the registry's matched total and the trials read, page by page.
fn read_pages(
    leg: &Leg<'_>,
    mut params: BTreeMap<String, String>,
) -> Result<(BTreeMap<Key, u64>, u64, u64), RequestError> {
    let mut counts = BTreeMap::<Key, u64>::new();
    let mut matched = None;
    let mut seen_trials = BTreeSet::new();
    let mut seen_tokens = BTreeSet::new();
    loop {
        let page = fetch(leg, &params)?;
        if let Some(total) = page.total_count {
            if matched.is_some_and(|earlier| earlier != total) {
                return Err(bad_page());
            }
            matched = Some(total);
        }
        for study in page.studies.unwrap_or_default() {
            if !seen_trials.insert(study.protocol_section.identification_module.nct_id) {
                return Err(bad_page());
            }
            for site in study.sites() {
                *counts.entry(site).or_default() += 1;
            }
        }
        let Some(token) = page.next_page_token else {
            break;
        };
        if !seen_tokens.insert(token.as_str().to_owned()) {
            return Err(bad_page());
        }
        params.insert("pageToken".into(), token.as_str().into());
    }
    let read = seen_trials.len() as u64;
    let matched = matched.filter(|&total| total == read).ok_or_else(bad_page)?;
    Ok((counts, matched, read))
}

fn bad_page() -> RequestError {
    biomcp_fetch::bad_reply(Source::ClinicalTrialsGov, Some(200))
}

fn fetch(leg: &Leg<'_>, params: &BTreeMap<String, String>) -> Result<SitePage, RequestError> {
    let args: Vec<_> = params
        .iter()
        .map(|(k, v)| (k.as_str(), Arg::Text(v)))
        .collect();
    leg.call(
        &super::SEARCH,
        Input::Params(&args),
        biomcp_fetch::Read::<SitePage>::json(),
    )
    .map(|received| received.value)
}
