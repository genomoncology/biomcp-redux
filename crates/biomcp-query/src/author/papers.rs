use crate::{Ctx, Outcome};
use biomcp_types::{AuthorPapersQuery, QueryBody};
/// Dispatch exact provider records without an identity join.
pub fn papers(_cx: &Ctx<'_>, _q: &AuthorPapersQuery) -> Outcome {
    super::empty(QueryBody::AuthorPapers(None))
}
