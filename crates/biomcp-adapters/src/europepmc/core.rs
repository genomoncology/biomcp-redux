//! Checked Europe PMC core records and their JATS document boundary.
use super::SPEC;
use crate::{jats, pmc_oa};
use biodata::{AbstractPart, Article, Doi, EuropePmcId, PartialDate, Pmid, Text};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, Reply, ReplyMetadata, RequestError};
use serde::Deserialize;
const CORE: Endpoint = SPEC.get("/").fresh(Freshness::Days(1));
const FULLTEXT: Endpoint = SPEC.get("/").fresh(Freshness::Days(7));
#[derive(Deserialize)]
struct Raw {
    result: Record,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    id: EuropePmcId,
    source: String,
    pmid: Option<Pmid>,
    doi: Option<Doi>,
    title: Text,
    first_publication_date: Option<PartialDate>,
    author_list: Authors,
    abstract_text: Option<String>,
    pub_type_list: Types,
    license: Option<String>,
    book_or_report_details: Option<Publisher>,
}
#[derive(Deserialize)]
struct Authors {
    author: Vec<Author>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Author {
    full_name: Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Types {
    pub_type: Vec<String>,
}
#[derive(Deserialize)]
struct Publisher {
    publisher: Option<Text>,
}
impl Reply for Raw {
    fn check(&self) -> Result<(), BadReply> {
        if self.result.source != "PPR" || !self.result.id.to_string().starts_with("PPR") {
            return Err(BadReply);
        }
        if let Some(xml) = &self.result.abstract_text {
            jats::markdown(&format!("<abstract>{xml}</abstract>"))?;
        }
        Ok(())
    }
}
fn bad(_: impl std::fmt::Debug) -> RequestError {
    biomcp_fetch::SourceError {
        source: biodata::Source::EuropePmc,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
fn record(leg: &Leg<'_>, id: &EuropePmcId) -> Result<(Record, ReplyMetadata), RequestError> {
    let endpoint = CORE.clone().with_path(format!("/article/PPR/{id}")).map_err(bad)?;
    let (raw, meta) = leg.json_received::<Raw>(&endpoint, &[
        ("format", Arg::Text("json")), ("resultType", Arg::Text("core")),
    ])?;
    if raw.result.id != *id {
        return Err(bad(BadReply));
    }
    Ok((raw.result, meta))
}
/// Resolve a preprint directly from its checked core metadata record.
pub fn core(leg: &Leg<'_>, id: &EuropePmcId) -> Result<(Article, ReplyMetadata), RequestError> {
    let (raw, meta) = record(leg, id)?;
    let abstract_parts = raw.abstract_text.as_ref().filter(|s| !s.trim().is_empty()).map(|xml| {
        jats::markdown(&format!("<abstract>{xml}</abstract>")).map(|text| AbstractPart { label: None, text })
    }).transpose().map_err(bad)?.into_iter().collect();
    Ok((Article {
        europepmc_id: Some(raw.id), pmid: raw.pmid, doi: raw.doi, pmcid: None,
        title: raw.title, journal: raw.book_or_report_details.and_then(|p| p.publisher),
        date: raw.first_publication_date,
        types: raw.pub_type_list.pub_type.iter().filter_map(|s| crate::pubmed::publication_type(s)).collect(),
        authors: raw.author_list.author.into_iter().map(|a| a.full_name).collect(),
        abstract_parts, corrections: Box::default(),
    }, meta))
}
/// Read JATS with the license owned by its core record, rather than inferring license XML.
pub fn read(leg: &Leg<'_>, id: &EuropePmcId) -> Result<pmc_oa::Document, RequestError> {
    let (raw, _) = record(leg, id)?;
    let endpoint = FULLTEXT.clone().with_path(format!("/{id}/fullTextXML")).map_err(bad)?;
    let body = leg.xml::<jats::Body>(&endpoint, &[])?;
    Ok(pmc_oa::Document {
        license: pmc_oa::license(raw.license.as_deref().unwrap_or("")),
        version: None, manuscript: None, body,
    })
}
