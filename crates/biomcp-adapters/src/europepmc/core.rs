//! Checked Europe PMC core records and their JATS document boundary.
use super::SPEC;
use crate::native_text::NativeText;
use crate::{jats, pmc_oa};
use biodata::{Doi, EuropePmcId, PartialDate, Pmid, Text};

use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, Reply, ReplyMetadata, RequestError};
use biomcp_types::{AbstractPart, Article};
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
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    title: biodata::Text,
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
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    full_name: biodata::Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Types {
    pub_type: Vec<String>,
}
#[derive(Deserialize)]
struct Publisher {
    publisher: Option<NativeText>,
}
impl Reply for Raw {
    fn check(&self) -> Result<(), BadReply> {
        if self.result.source != "PPR" || !self.result.id.to_string().starts_with("PPR") {
            return Err(BadReply);
        }
        if let Some(xml) = &self.result.abstract_text {
            jats::markdown(&fragment(xml)?)?;
        }
        Ok(())
    }
}
fn bad(_: impl std::fmt::Debug) -> RequestError {
    biomcp_fetch::SourceError {
        source: biomcp_types::Source::EuropePmc,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
fn record(leg: &Leg<'_>, id: &EuropePmcId) -> Result<(Record, ReplyMetadata), RequestError> {
    let endpoint = CORE
        .clone()
        .with_path(format!("/article/PPR/{id}"))
        .map_err(bad)?;
    let (raw, meta) = leg.json_received::<Raw>(
        &endpoint,
        &[
            ("format", Arg::Text("json")),
            ("resultType", Arg::Text("core")),
        ],
    )?;
    if raw.result.id != *id {
        return Err(bad(BadReply));
    }
    Ok((raw.result, meta))
}
/// Resolve a preprint directly from its checked core metadata record.
pub fn core(leg: &Leg<'_>, id: &EuropePmcId) -> Result<(Article, ReplyMetadata), RequestError> {
    let (raw, meta) = record(leg, id)?;
    let abstract_parts = raw
        .abstract_text
        .as_ref()
        .filter(|s| !s.trim().is_empty())
        .map(|xml| {
            fragment(xml)
                .and_then(|xml| jats::markdown(&xml))
                .map(|text| AbstractPart { label: None, text })
        })
        .transpose()
        .map_err(bad)?
        .into_iter()
        .collect();
    Ok((
        Article {
            europepmc_id: Some(raw.id),
            pmid: raw.pmid,
            doi: raw.doi,
            pmcid: None,
            title: Some(raw.title),
            journal: raw
                .book_or_report_details
                .and_then(|p| p.publisher)
                .map(Into::into),
            date: raw.first_publication_date,
            electronic_date: None,
            types: raw
                .pub_type_list
                .pub_type
                .iter()
                .filter_map(|s| crate::pubmed::publication_type(s))
                .collect(),
            authors: raw
                .author_list
                .author
                .into_iter()
                .map(|a| a.full_name)
                .collect(),
            abstract_parts,
            corrections: Box::default(),
        },
        meta,
    ))
}
/// Read JATS with the license owned by its core record, rather than inferring license XML.
pub fn read(leg: &Leg<'_>, id: &EuropePmcId) -> Result<pmc_oa::Document, RequestError> {
    let (raw, _) = record(leg, id)?;
    let endpoint = FULLTEXT
        .clone()
        .with_path(format!("/{id}/fullTextXML"))
        .map_err(bad)?;
    let mut body = leg.xml::<jats::Body>(&endpoint, &[])?;
    if body.title.is_none() {
        body.title = Some(raw.title);
    }
    Ok(pmc_oa::Document {
        url: Text::new(&format!("{}/{id}/fullTextXML", SPEC.base)).map_err(bad)?,
        license: pmc_oa::license(raw.license.as_deref().unwrap_or("")),
        version: None,
        manuscript: None,
        supplements: Default::default(),
        body,
    })
}

// Europe PMC abstractText is an HTML fragment. A comparison such as p<0.001 is
// text, not a tag; escape that literal before the checked XML boundary. Broken
// tags still fail that boundary, and no markup or decoded text is discarded.
fn fragment(mut html: &str) -> Result<String, BadReply> {
    let mut xml = String::from("<abstract>");
    while let Some(start) = html.find('<') {
        xml.push_str(&html[..start]);
        html = &html[start..];
        if !html
            .as_bytes()
            .get(1)
            .is_some_and(|c| c.is_ascii_alphabetic() || matches!(c, b'/' | b'!' | b'?'))
        {
            xml.push_str("&lt;");
            html = &html[1..];
            continue;
        }
        // Consume a whole markup event, so comparisons in CDATA stay literal
        // and malformed attributes cannot be repaired into accepted markup.
        let mut reader = quick_xml::Reader::from_str(html);
        reader.config_mut().check_end_names = false;
        reader.config_mut().allow_unmatched_ends = true;
        match reader.read_event().map_err(|_| BadReply)? {
            quick_xml::events::Event::Start(tag) | quick_xml::events::Event::Empty(tag) => {
                for attribute in tag.attributes() {
                    if attribute.map_err(|_| BadReply)?.value.contains('<') {
                        return Err(BadReply);
                    }
                }
            }
            _ => {}
        }
        let end = reader.buffer_position() as usize;
        xml.push_str(&html[..end]);
        html = &html[end..];
    }
    xml.push_str(html);
    xml.push_str("</abstract>");
    Ok(xml)
}
