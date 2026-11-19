//! Reach a row offset without a page token (1061): the registry pages only by token, so walk
//! its own pages of trial IDs alone up to the offset and start the asked page from there.
use biodata::{NctId, Text};
use biomcp_fetch::{Arg, BadReply, Input, Leg, Reply, RequestError};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// Rows per skipped page; the registry served 1,000 trials in one reply (live 2026-11-17).
const PAGE_SIZE: u32 = 1000;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IdPage {
    total_count: Option<u64>,
    studies: Option<Vec<Study>>,
    next_page_token: Option<Text>,
}
impl Reply for IdPage {
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
        // Only an explicit zero total may leave out the study list.
        let studies = self.studies.as_deref().unwrap_or_default();
        if (self.studies.is_none() && self.total_count != Some(0))
            || studies.len() > PAGE_SIZE as usize
        {
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
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Identification {
    nct_id: NctId,
}

/// Where the asked page starts.
pub(super) enum Start {
    /// The registry token for the row at the offset.
    Token(String),
    /// The search ends before the offset; the registry's total, when it stated one.
    Ended(Option<u64>),
}

/// Walk the search's pages of IDs until `offset` rows have passed.
pub(super) fn start(
    leg: &Leg<'_>,
    params: &BTreeMap<String, String>,
    offset: u32,
) -> Result<Start, RequestError> {
    let mut params = params.clone();
    params.insert("fields".into(), "NCTId".into());
    let mut left = offset;
    let mut total = None;
    let mut seen = BTreeSet::new();
    while left > 0 {
        let size = left.min(PAGE_SIZE);
        params.insert("pageSize".into(), size.to_string());
        let page = fetch(leg, &params)?;
        total = total.or(page.total_count);
        if total.is_some_and(|total| u64::from(offset) >= total) {
            return Ok(Start::Ended(total));
        }
        let studies = page.studies.unwrap_or_default();
        if studies
            .iter()
            .any(|study| !seen.insert(study.protocol_section.identification_module.nct_id))
        {
            return Err(biomcp_fetch::bad_reply(
                biomcp_types::Source::ClinicalTrialsGov,
                Some(200),
            ));
        }
        let Some(token) = page
            .next_page_token
            .filter(|_| studies.len() as u32 == size)
        else {
            return Ok(Start::Ended(total));
        };
        params.insert("pageToken".into(), token.as_str().into());
        left -= size;
    }
    Ok(params
        .remove("pageToken")
        .map_or(Start::Ended(total), Start::Token))
}

/// An empty page past the end of the search, with the registry's total.
pub(super) fn ended(total: Option<u64>) -> super::SearchPage {
    super::SearchPage {
        refusals: crate::Refusal::None,
        received: 0,
        items: Vec::new(),
        problems: Vec::new(),
        condition_word_mismatch: false,
        gene_contexts: Vec::new(),
        codon_only: Vec::new(),
        codon_error: None,
        total,
        count_error: None,
        count_request: None,
        next: None,
    }
}

fn fetch(leg: &Leg<'_>, params: &BTreeMap<String, String>) -> Result<IdPage, RequestError> {
    let args: Vec<_> = params
        .iter()
        .map(|(k, v)| (k.as_str(), Arg::Text(v)))
        .collect();
    leg.call(
        &super::SEARCH,
        Input::Params(&args),
        biomcp_fetch::Read::<IdPage>::json(),
    )
    .map(|received| received.value)
}
