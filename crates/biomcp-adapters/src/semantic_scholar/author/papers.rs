use super::super::SPEC;
use biodata::SemanticScholarAuthorId;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, RequestError};
use biomcp_types::AuthorPapersQuery;
use serde::{Deserialize, Serialize};
mod row;
pub use row::Paper;
const COMPACT: &str = "paperId,externalIds,title,year,venue";
const DETAILS: &str = "paperId,externalIds,title,year,venue,abstract,publicationDate,citationCount,referenceCount,influentialCitationCount,isOpenAccess,openAccessPdf,fieldsOfStudy,publicationTypes,authors.authorId,authors.name";
/// Native papers page; the endpoint may omit both author identity and total.
#[derive(Debug, Deserialize, Serialize)]
pub struct PapersPage {
    /// Native window start.
    pub offset: u32,
    /// Native next window, with no inferred continuation.
    pub next: Option<u32>,
    /// Native total, when supplied.
    pub total: Option<u64>,
    /// Unchanged native paper order.
    pub data: Vec<Paper>,
}
impl Reply for PapersPage {
    fn check(&self) -> Result<(), BadReply> {
        for row in &self.data { row.clone().detail()?; }
        if self.data.len() > 25
            || self.next.is_some_and(|n| n <= self.offset)
            || self.total.is_some_and(|n| self.data.len() as u64 > n)
        {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
/// Fetch exactly the requested native window without enrichment.
pub fn papers(
    leg: &Leg<'_>,
    id: &SemanticScholarAuthorId,
    q: &AuthorPapersQuery,
) -> Result<PapersPage, RequestError> {
    let endpoint = SPEC.get("").with_path(format!("/author/{id}/papers"))?
        .fresh(Freshness::Days(1));
    let fields = if q.section().is_some() { DETAILS } else { COMPACT };
    let (page, _) = leg.json_received_checked::<PapersPage>(
        &endpoint,
        &[("fields", Arg::Text(fields)),
          ("offset", Arg::Num(u64::from(q.offset()))),
          ("limit", Arg::Num(u64::from(q.limit())))],
        |page| {
            if page.offset == q.offset() && page.data.len() <= q.limit() as usize
                
            { Ok(()) } else { Err(BadReply) }
        },
    )?;
    Ok(page)
}
