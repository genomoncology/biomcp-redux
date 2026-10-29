use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        AuthorSearch => ("author-search-query", "biomcp.author-search-query/3", query),
        AuthorPapersQuery => ("author-papers-query", "biomcp.author-papers-query/2", query),
        AuthorGet => ("author-get-query", "biomcp.author-get-query/2", query),
        Envelope<AuthorPage> => ("author-page", "biomcp.author-page/4"),
        Envelope<AuthorCards> => ("author-card", "biomcp.author-card/5"),
        Envelope<AuthorPapers> => ("author-papers", "biomcp.author-papers/3"),
        Envelope<AuthorPaperDetails> => ("author-paper-details", "biomcp.author-paper-details/3"),
        Envelope<OrcidWorks> => ("orcid-works", "biomcp.orcid-works/5")
    }
}
