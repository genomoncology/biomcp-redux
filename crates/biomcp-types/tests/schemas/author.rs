use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        AuthorSearch => ("author-search-query", "biomcp.author-search-query/3", query),
        AuthorPapersQuery => ("author-papers-query", "biomcp.author-papers-query/2", query),
        AuthorGet => ("author-get-query", "biomcp.author-get-query/2", query),
    }
}
