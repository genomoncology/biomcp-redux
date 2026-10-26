//! Literal native author candidates and request-checked exact provider cards.
mod raw;
mod papers;
pub use papers::{PapersPage, papers};
use super::SPEC;
use biodata::{AuthorCard, SemanticScholarAuthorId};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_types::AuthorSearch;
pub use raw::Page;
const FIELDS: &str = "authorId,name,affiliations,paperCount,citationCount,hIndex,url";
/// Read one unchanged source query in native order.
pub fn search(leg: &Leg<'_>, q: &AuthorSearch) -> Result<Page, RequestError> {
    let endpoint = SPEC.get("/author/search").fresh(Freshness::Days(1));
    let (page, _) = leg.json_received_checked::<Page>(
        &endpoint,
        &[
            ("query", Arg::Text(q.query())),
            ("fields", Arg::Text(FIELDS)),
            ("offset", Arg::Num(u64::from(q.offset()))),
            ("limit", Arg::Num(u64::from(q.limit()))),
        ],
        |page| {
            if page.offset == q.offset() && page.data.len() <= q.limit() as usize {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    Ok(page)
}
/// Reject a mismatched returned identity before cache admission and on cache reads.
pub fn card(leg: &Leg<'_>, id: &SemanticScholarAuthorId) -> Result<AuthorCard, RequestError> {
    let endpoint = SPEC
        .get("")
        .with_path(format!("/author/{id}"))?
        .fresh(Freshness::Days(1));
    let (row, _) = leg.json_received_checked::<raw::Record>(
        &endpoint,
        &[("fields", Arg::Text(FIELDS))],
        |row| {
            if &row.author_id == id {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    Ok(row.card())
}
