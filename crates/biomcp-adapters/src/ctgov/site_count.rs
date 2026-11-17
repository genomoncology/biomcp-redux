//! Facilities counted over every page of a registry match, each name exactly as registered.
use crate::raw::Field;
use biodata::Text;
use biomcp_fetch::{Arg, BadReply, Input, Leg, Reply, RequestError};
use biomcp_types::{Source, TrialSearch, TrialSiteCount, TrialSiteCounts};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// Rows per request; the registry served all sites of 1,000 trials in one reply (live 2026-11-17).
const PAGE_SIZE: usize = 1000;
/// Pages read before the count stops; the reply then reads fewer trials than matched.
const MAX_PAGES: usize = 5;
const FIELDS: &str = "NCTId,LocationFacility,LocationCity,LocationCountry";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SitePage {
    total_count: Option<u64>,
    studies: Option<Vec<Field<Study>>>,
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
    protocol_section: Option<Field<Protocol>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Protocol {
    contacts_locations_module: Option<Field<Locations>>,
}
#[derive(Deserialize)]
struct Locations {
    locations: Option<Field<Vec<Field<Site>>>>,
}
#[derive(Deserialize)]
struct Site {
    facility: Option<Field<Box<str>>>,
    city: Option<Field<Box<str>>>,
    country: Option<Field<Box<str>>>,
}
type Key = (Box<str>, Option<Box<str>>, Option<Box<str>>);
impl Study {
    /// Each named site once per trial, however often the trial lists it.
    fn sites(self) -> BTreeSet<Key> {
        self.protocol_section
            .and_then(Field::value)
            .and_then(|p| p.contacts_locations_module)
            .and_then(Field::value)
            .and_then(|m| m.locations)
            .and_then(Field::value)
            .unwrap_or_default()
            .into_iter()
            .filter_map(Field::value)
            .filter_map(|site| {
                let facility = site.facility.and_then(Field::value)?;
                Some((
                    facility,
                    site.city.and_then(Field::value),
                    site.country.and_then(Field::value),
                ))
            })
            .collect()
    }
}

/// Count the facilities of every matched trial, up to the page limit, and keep the `limit` most.
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
    let mut counts = BTreeMap::<Key, u64>::new();
    let mut matched = None;
    let mut read = 0;
    for _ in 0..MAX_PAGES {
        let page = fetch(leg, &params)?;
        matched = matched.or(page.total_count);
        let studies = page.studies.unwrap_or_default();
        read += studies.len() as u64;
        for site in studies
            .into_iter()
            .filter_map(Field::value)
            .flat_map(Study::sites)
        {
            *counts.entry(site).or_default() += 1;
        }
        let Some(token) = page.next_page_token else {
            break;
        };
        params.insert("pageToken".into(), token.as_str().into());
    }
    let matched =
        matched.ok_or_else(|| biomcp_fetch::bad_reply(Source::ClinicalTrialsGov, Some(200)))?;
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
