use biodata::{
    AuthorCard, AuthorCountScope, AuthorCounts, AuthorHeader, AuthorId, SemanticScholarAuthorId,
    Text,
};
use biomcp_fetch::{BadReply, Reply};
use serde::{Deserialize, Serialize};
/// Native source page, with no inferred next offset or result total.
#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    /// Native start offset.
    pub offset: u32,
    /// Native next offset, only when supplied.
    pub next: Option<u32>,
    /// Native matching records, when supplied.
    pub total: Option<u64>,
    /// Native candidate order.
    pub data: Vec<Record>,
}
/// Native author metadata; omitted optional fields remain absent.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    /// Exact provider record identity.
    pub author_id: SemanticScholarAuthorId,
    name: Text,
    affiliations: Option<Box<[Text]>>,
    paper_count: Option<u64>,
    citation_count: Option<u64>,
    h_index: Option<u32>,
    url: Option<Text>,
}
impl Record {
    /// Preserve optional metadata and its provider-record count scope.
    pub fn header(self) -> AuthorHeader {
        let counts =
            (self.paper_count.is_some() || self.citation_count.is_some() || self.h_index.is_some())
                .then_some(AuthorCounts {
                    scope: AuthorCountScope::ProviderRecord,
                    paper_count: self.paper_count,
                    citation_count: self.citation_count,
                    h_index: self.h_index,
                });
        AuthorHeader {
            id: AuthorId::SemanticScholar(self.author_id),
            name: self.name,
            affiliations: self.affiliations,
            counts,
        }
    }
    /// Keep the provider link separate from the candidate metadata.
    pub fn card(self) -> AuthorCard {
        let url = self.url.clone();
        AuthorCard {
            header: self.header(),
            url,
        }
    }
}
impl Reply for Record {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.data.len() > 25
            || self.next.is_some_and(|n| n <= self.offset)
            || self.total.is_some_and(|t| self.data.len() as u64 > t)
        {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
