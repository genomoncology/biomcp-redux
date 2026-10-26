use biodata::{
    AuthorByline, AuthorPaperAccess, AuthorPaperDetail, AuthorPaperExternalId, AuthorPaperHeader,
    PartialDate, SemanticScholarId, Text,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// Native paper fields; optional omissions and collection multiplicity stay distinct.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paper {
    paper_id: SemanticScholarId,
    external_ids: Option<BTreeMap<String, Option<NativeId>>>,
    title: Text,
    year: Option<u32>,
    venue: Option<String>,
    #[serde(rename = "abstract")]
    abstract_text: Option<Text>,
    publication_date: Option<PartialDate>,
    reference_count: Option<u64>,
    citation_count: Option<u64>,
    influential_citation_count: Option<u64>,
    is_open_access: Option<bool>,
    open_access_pdf: Option<Access>,
    fields_of_study: Option<Box<[Text]>>,
    publication_types: Option<Box<[Text]>>,
    authors: Option<Box<[Byline]>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum NativeId {
    Text(String),
    Number(u64),
}
impl std::fmt::Display for NativeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(v) => v.fmt(f),
            Self::Number(v) => v.fmt(f),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Byline {
    #[serde(rename = "authorId")]
    id: Option<biodata::SemanticScholarAuthorId>,
    name: Text,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Access {
    url: Option<String>,
    status: Option<Text>,
    license: Option<Text>,
    disclaimer: Option<Text>,
}
impl Access {
    fn checked(self) -> AuthorPaperAccess {
        AuthorPaperAccess {
            url: self.url.and_then(|s| Text::new(&s).ok()),
            status: self.status,
            license: self.license,
            disclaimer: self.disclaimer,
        }
    }
}
impl Paper {
    /// Convert exactly the source identifiers and metadata, with no title matching.
    pub fn detail(self) -> Result<AuthorPaperDetail, biomcp_fetch::BadReply> {
        let mut header = AuthorPaperHeader {
            paper_id: self.paper_id,
            pmid: None,
            pmcid: None,
            doi: None,
            external_ids: Box::default(),
            title: self.title,
            year: self.year,
            venue: self.venue.and_then(|v| Text::new(&v).ok()),
        };
        let mut external = Vec::new();
        for (namespace, value) in self.external_ids.into_iter().flatten() {
            let Some(value) = value else {
                continue;
            };
            let text = value.to_string();
            match namespace.as_str() {
                "PubMed" => header.pmid = Some(text.parse().map_err(|_| biomcp_fetch::BadReply)?),
                "DOI" => header.doi = Some(text.parse().map_err(|_| biomcp_fetch::BadReply)?),
                "PubMedCentral" => {
                    let id = if text.starts_with("PMC") {
                        text
                    } else {
                        format!("PMC{text}")
                    };
                    header.pmcid = Some(id.parse().map_err(|_| biomcp_fetch::BadReply)?);
                }
                _ => external.push(AuthorPaperExternalId {
                    id: format!("{namespace}:{text}")
                        .parse()
                        .map_err(|_| biomcp_fetch::BadReply)?,
                }),
            }
        }
        header.external_ids = external.into();
        Ok(AuthorPaperDetail {
            header,
            abstract_text: self.abstract_text,
            publication_date: self.publication_date,
            reference_count: self.reference_count,
            citation_count: self.citation_count,
            influential_citation_count: self.influential_citation_count,
            is_open_access: self.is_open_access,
            open_access_pdf: self.open_access_pdf.map(Access::checked),
            fields_of_study: self.fields_of_study,
            publication_types: self.publication_types,
            authors: self.authors.map(|rows| {
                rows.into_iter()
                    .map(|r| AuthorByline {
                        id: r.id,
                        name: r.name,
                    })
                    .collect()
            }),
        })
    }
}
