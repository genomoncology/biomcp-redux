use crate::{Ctx, Outcome};
use biomcp_types::AuthorPapersQuery;
/// Dispatch exact provider records without an identity join.
pub fn papers(cx: &Ctx<'_>, q: &AuthorPapersQuery) -> Outcome {
    match q.id() {
        biodata::AuthorId::Orcid(id) => super::orcid::papers(cx, q, id),
        biodata::AuthorId::SemanticScholar(_) => super::semantic_scholar::papers(cx, q),
    }
}
