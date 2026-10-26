//! Europe PMC's bounded metadata-only search window.
mod core;
use crate::quotas;
use biodata::{Doi, EuropePmcId, PartialDate, Pmcid, Pmid, Text};

use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, Reply, RequestError, SourceSpec,
};
use biomcp_types::ArticleHeader;
use biomcp_types::{ArticleItem, ArticleKey, ArticleSort, Source};
pub use core::{core, read};
use serde::Deserialize;
use std::time::Duration;
/// Public search policy with a conservative local quota.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "Europe PMC copyright; record-specific licenses",
        url: "https://europepmc.org/help#copyright",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::EuropePmc,
    base: "https://www.ebi.ac.uk/europepmc/webservices/rest",
    key: KeyUse::None,
    quota: &quotas::EUROPEPMC,
    deadline: Duration::from_secs(15),
};
const SEARCH: Endpoint = SPEC.get("/search").fresh(Freshness::Days(1));
/// Original row positions and metadata, before any duplicate suppression.
#[derive(Debug)]
pub struct Search {
    /// Source-reported whole result count.
    pub count: u64,
    /// The actual fetched rows, including duplicates in their original positions.
    pub items: Vec<ArticleItem>,
}
/// Fetch one metadata page in the requested provider order.
pub fn search(leg: &Leg<'_>, term: &str, sort: ArticleSort) -> Result<Search, RequestError> {
    let mut args = vec![
        ("query", Arg::Text(term)),
        ("format", Arg::Text("json")),
        ("resultType", Arg::Text("lite")),
        ("pageSize", Arg::Num(100)),
    ];
    if sort == ArticleSort::Date {
        args.push(("sort", Arg::Text("FIRST_PDATE_D desc")));
    }
    let raw = leg.json::<Raw>(&SEARCH, &args)?;
    Ok(Search {
        count: raw.hit_count,
        items: raw
            .result_list
            .result
            .into_iter()
            .map(Row::item)
            .collect::<Result<_, _>>()?,
    })
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Raw {
    hit_count: u64,
    result_list: Rows,
}
#[derive(Deserialize)]
struct Rows {
    result: Vec<Row>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    id: String,
    source: String,
    pmid: Option<Pmid>,
    pmcid: Option<Pmcid>,
    doi: Option<Doi>,
    title: Option<Text>,
    journal_title: Option<Text>,
    first_publication_date: Option<PartialDate>,
    author_string: Option<Text>,
}
impl Reply for Raw {
    fn check(&self) -> Result<(), BadReply> {
        if self.result_list.result.len() > 100
            || self.result_list.result.len() as u64 > self.hit_count
            || self
                .result_list
                .result
                .iter()
                .any(|r| r.local_id().is_err())
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
impl Row {
    fn local_id(&self) -> Result<EuropePmcId, biodata::Error> {
        let id = if self.id.starts_with(&self.source) {
            self.id.clone()
        } else {
            format!("{}{}", self.source, self.id)
        };
        id.parse()
    }
    fn item(self) -> Result<ArticleItem, RequestError> {
        let pmid = self.pmid.or_else(|| {
            (self.source == "MED")
                .then(|| self.id.parse().ok())
                .flatten()
        });
        // Reply validation checked the fallback before this owned mapping.
        let local = self.local_id().map_err(|_| biomcp_fetch::SourceError {
            source: Source::EuropePmc,
            reason: biomcp_types::Reason::BadReply,
            status: Some(200),
            retry_after: None,
        })?;
        let key = if let Some(id) = pmid {
            ArticleKey::Pubmed(id)
        } else if let Some(id) = self.pmcid {
            ArticleKey::Pmc(id)
        } else if let Some(id) = self.doi.clone() {
            ArticleKey::Doi(id)
        } else {
            ArticleKey::Europepmc(local.clone())
        };
        let header = if pmid.is_none() {
            self.title.map(|title| ArticleHeader {
                pmid: None,
                europepmc_id: Some(local),
                doi: self.doi,
                pmcid: self.pmcid,
                title,
                journal: self.journal_title,
                date: self.first_publication_date,
                electronic_date: None,
                types: Box::default(),
                types_unknown: true,
                first_author: self
                    .author_string
                    .as_ref()
                    .and_then(|s| s.as_str().split(',').next())
                    .and_then(|s| Text::new(s.trim()).ok()),
                first_author_id: None,
                author_count: self
                    .author_string
                    .as_ref()
                    .map_or(0, |s| s.as_str().split(',').count() as u32),
            })
        } else {
            None
        };
        Ok(ArticleItem {
            key: Some(key),
            pmid,
            header,
            cited_by: None,
            intents: None,
            influential: None,
            contexts: None,
            found_by: Default::default(),
            sentence: None,
        })
    }
}
