//! Keep explicit snapshots stable while replacing legacy implicit literal-name pools.
use crate::{Ctx, Outcome, store::Pool};
use biodata::Text;
use biomcp_types::{ArticleSearch, SourceNotice};

pub(super) fn reuse(
    cx: &Ctx<'_>,
    query: &ArticleSearch,
    stored: Option<Pool>,
    out: &mut Outcome,
) -> (Option<Pool>, Option<Vec<Text>>) {
    if query.pool().is_some()
        || query.exact_term()
        || stored.as_ref().is_none_or(|pool| {
            pool.sources
                .iter()
                .flat_map(|source| &source.notices)
                .any(|notice| matches!(notice, SourceNotice::DrugNameSearch { .. }))
        })
    {
        return (stored, None);
    }
    let names = crate::drug::search_names::article(cx, query, out);
    let stored = if names.is_empty() { stored } else { None };
    (stored, Some(names))
}
