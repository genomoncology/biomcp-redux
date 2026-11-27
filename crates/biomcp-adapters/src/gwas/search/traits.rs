//! Bounded catalog trait lookup for an empty association search.
use super::super::{SPEC, raw};
use biodata::{GwasTraitId, Text};
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Read, Reply, RequestError};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

const RESOURCE: &str = "efo-traits";
const PAGE_SIZE: u32 = 10;
const CANDIDATES: usize = 3;

/// A name and ontology ID supplied by GWAS Catalog.
#[derive(Debug, Deserialize)]
pub struct TraitCandidate {
    /// Checked catalog ontology ID.
    #[serde(rename = "efo_id")]
    pub id: GwasTraitId,
    /// Whole catalog name.
    #[serde(rename = "efo_trait")]
    pub name: Text,
}

impl Reply for TraitCandidate {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}

/// The exact catalog name or the nearest names on the bounded source page.
#[derive(Debug)]
pub enum TraitMatch {
    /// A catalog trait has the whole name.
    Exact,
    /// No exact name appeared on the source page.
    Similar {
        /// The page covered all catalog matches.
        exhaustive: bool,
        /// Closest names on that page.
        candidates: Vec<TraitCandidate>,
    },
}

#[derive(Debug, Deserialize)]
struct TraitPage {
    #[serde(rename = "_embedded")]
    embedded: Option<Embedded>,
    #[serde(rename = "_links")]
    links: raw::Links,
    page: raw::Page,
}

#[derive(Debug, Deserialize)]
struct Embedded {
    efo_traits: Vec<TraitCandidate>,
}

impl Reply for TraitPage {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}

/// Confirm an ID against its own resource, where a missing ID returns a source 404.
pub fn trait_exists(leg: &Leg<'_>, id: &GwasTraitId) -> Result<(), RequestError> {
    let endpoint = SPEC
        .get("")
        .with_path(format!("/{RESOURCE}/{id}"))?
        .fresh(Freshness::Days(1));
    leg.call(
        &endpoint,
        Input::Params(&[]),
        Read::<TraitCandidate>::json().check(&|row| (&row.id == id).then_some(()).ok_or(BadReply)),
    )?;
    Ok(())
}

/// Read catalog label matches without reading associations for any suggested trait.
pub fn trait_matches(leg: &Leg<'_>, name: &Text) -> Result<TraitMatch, RequestError> {
    let endpoint = SPEC.get("/efo-traits").fresh(Freshness::Days(1));
    let values: BTreeMap<String, String> = BTreeMap::from([
        ("efo_trait".into(), name.to_string()),
        ("page".into(), "0".into()),
        ("size".into(), PAGE_SIZE.to_string()),
    ]);
    let params: Vec<_> = values
        .iter()
        .map(|(key, value)| (key.as_str(), Arg::Text(value)))
        .collect();
    let folded = name.as_str().to_lowercase();
    let reply = leg
        .call(
            &endpoint,
            Input::Params(&params),
            Read::<TraitPage>::json().check(&|reply| admission(reply, &values, &folded)),
        )?
        .value;
    let mut candidates = reply.embedded.map(|e| e.efo_traits).unwrap_or_default();
    if candidates
        .iter()
        .any(|row| row.name.as_str().to_lowercase() == folded)
    {
        return Ok(TraitMatch::Exact);
    }
    candidates.sort_by_key(|row| row.name.as_str().chars().count());
    candidates.truncate(CANDIDATES);
    Ok(TraitMatch::Similar {
        exhaustive: reply.page.total_elements <= u64::from(PAGE_SIZE),
        candidates,
    })
}

fn admission(
    reply: &TraitPage,
    values: &BTreeMap<String, String>,
    folded: &str,
) -> Result<(), BadReply> {
    let rows = reply
        .embedded
        .as_ref()
        .map(|e| e.efo_traits.as_slice())
        .unwrap_or_default();
    let mut seen = BTreeSet::new();
    if reply.page.number == 0
        && reply.page.size == PAGE_SIZE
        && rows.len() as u64 == reply.page.total_elements.min(u64::from(PAGE_SIZE))
        && super::identity::resource_link(reply.links.current.href.as_str(), RESOURCE, values)
        && rows.iter().all(|row| {
            row.name.as_str().to_lowercase().contains(folded) && seen.insert(row.id.to_string())
        })
    {
        Ok(())
    } else {
        Err(BadReply)
    }
}
