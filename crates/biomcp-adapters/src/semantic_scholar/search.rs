use crate::native_text::NativeText;
use biodata::{Doi, PartialDate, Pmcid, Pmid, SemanticScholarId, Text};

use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, Reply, RequestError};
use biomcp_types::ArticleHeader;
use biomcp_types::{ArticleItem, ArticleKey};
use serde::Deserialize;
const SEARCH: Endpoint = super::SPEC.get("/paper/search").fresh(Freshness::Days(1));
/// A native search window, before duplicate suppression or DOI resolution.
#[derive(Debug, Deserialize)]
pub struct Search {
    /// Whole source-reported result count.
    pub total: u64,
    /// Native result order.
    pub data: Vec<Paper>,
}
/// Common paper metadata shared by search and graph expansion.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paper {
    /// Stable source identity, never a title-derived fallback.
    pub paper_id: Option<SemanticScholarId>,
    /// Identifiers explicitly reported by the source.
    pub external_ids: Option<ExternalIds>,
    /// Whole title when supplied.
    pub title: Option<NativeText>,
    /// Source venue.
    pub venue: Option<String>,
    /// Publication year when full date is absent.
    pub year: Option<u16>,
    /// Publication date at source precision.
    pub publication_date: Option<PartialDate>,
    /// Authors in source order.
    pub authors: Option<Vec<Author>>,
}
/// External identities, preserving absence independently of paper identity.
#[derive(Debug)]
pub struct ExternalIds {
    /// PubMed identity.
    pub pubmed: Option<Pmid>,
    /// PMC identity.
    pub pmc: Option<Pmcid>,
    /// DOI spelling reported by the source.
    pub doi: Option<Doi>,
}
impl<'de> Deserialize<'de> for ExternalIds {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "PubMed")]
            pubmed: Option<Pmid>,
            #[serde(rename = "PubMedCentral")]
            pmc: Option<String>,
            #[serde(rename = "DOI")]
            doi: Option<Doi>,
        }
        let raw = Raw::deserialize(deserializer)?;
        let pmc = raw
            .pmc
            .map(|value| {
                if value.starts_with("PMC") {
                    value
                } else {
                    format!("PMC{value}")
                }
            })
            .map(|value| value.parse())
            .transpose()
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            pubmed: raw.pubmed,
            pmc,
            doi: raw.doi,
        })
    }
}
/// One source author.
#[derive(Debug, Deserialize)]
pub struct Author {
    /// Exact native author identity when this reply supplies one.
    #[serde(rename = "authorId")]
    pub id: Option<biodata::SemanticScholarAuthorId>,
    /// Source display name.
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub name: biodata::Text,
}
impl Reply for Search {
    fn check(&self) -> Result<(), BadReply> {
        if self.data.len() > 100 || self.data.len() as u64 > self.total {
            return Err(BadReply);
        }
        Ok(())
    }
}
/// Fetch the first relevance-ordered page using only checked mappings.
pub fn search(
    leg: &Leg<'_>,
    term: &str,
    year: Option<&str>,
    open_access: bool,
) -> Result<Search, RequestError> {
    let mut args = vec![
        ("query", Arg::Text(term)),
        ("limit", Arg::Num(100)),
        (
            "fields",
            Arg::Text("externalIds,title,venue,year,publicationDate,authors"),
        ),
    ];
    if let Some(year) = year {
        args.push(("year", Arg::Text(year)));
    }
    if open_access {
        args.push(("openAccessPdf", Arg::Text("")));
    }
    leg.json(&SEARCH, &args)
}
impl Paper {
    /// Map identifiers and source metadata without joining another provider.
    pub fn item(self) -> ArticleItem {
        let ids = self.external_ids;
        let pmid = ids.as_ref().and_then(|v| v.pubmed);
        let pmcid = ids.as_ref().and_then(|v| v.pmc);
        let doi = ids.and_then(|v| v.doi);
        let key = if let Some(id) = pmid {
            Some(ArticleKey::Pubmed(id))
        } else if let Some(id) = pmcid {
            Some(ArticleKey::Pmc(id))
        } else if let Some(id) = doi.clone() {
            Some(ArticleKey::Doi(id))
        } else {
            self.paper_id.map(ArticleKey::SemanticScholar)
        };
        let header = self.title.map(|title| ArticleHeader {
            pmid,
            pmcid,
            doi,
            europepmc_id: None,
            title: Some(title.into()),
            journal: self.venue.and_then(|s| Text::from_source(&s).ok()),
            electronic_date: None,
            date: self
                .publication_date
                .or_else(|| self.year.and_then(|v| v.to_string().parse().ok())),
            types: Box::default(),
            types_unknown: true,
            first_author: self
                .authors
                .as_ref()
                .and_then(|a| a.first())
                .map(|a| a.name.clone()),
            first_author_id: self
                .authors
                .as_ref()
                .and_then(|a| a.first())
                .and_then(|a| a.id.clone()),
            author_count: self.authors.as_ref().map_or(0, |a| a.len() as u32),
        });
        ArticleItem {
            key,
            pmid,
            header,
            cited_by: None,
            sentence: None,
            intents: None,
            influential: None,
            contexts: None,
            found_by: Default::default(),
        }
    }
}
