use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        AuthorSearch => ("author-search-query", "biomcp.author-search-query/2"),
        AuthorPapersQuery => ("author-papers-query", "biomcp.author-papers-query/1"),
        AuthorGet => ("author-get-query", "biomcp.author-get-query/2"),
        Envelope<AuthorPage> => ("author-page", "biomcp.author-page/2"),
        Envelope<AuthorCards> => ("author-card", "biomcp.author-card/3"),
        Envelope<AuthorPapers> => ("author-papers", "biomcp.author-papers/1"),
        Envelope<AuthorPaperDetails> => ("author-paper-details", "biomcp.author-paper-details/1"),
        Envelope<OrcidWorks> => ("orcid-works", "biomcp.orcid-works/2")
    }
}
