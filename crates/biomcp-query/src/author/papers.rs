use crate::{Ctx, Outcome};
use biomcp_types::AuthorPapersQuery;
/// Route one native author papers selector to its provider.
pub fn papers(cx: &Ctx<'_>, q: &AuthorPapersQuery) -> Outcome {
    super::semantic_scholar::papers(cx, q)
}
