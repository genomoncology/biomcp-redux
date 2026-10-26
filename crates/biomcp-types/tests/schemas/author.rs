use super::{Contract, generate};
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        Envelope<AuthorPage> => ("author-page", "biomcp.author_page/1"),
        Envelope<AuthorCards> => ("author-card", "biomcp.author_card/1"),
        Envelope<AuthorPapers> => ("author-papers", "biomcp.author_papers/1"),
        Envelope<AuthorPaperDetails> => ("author-paper-details", "biomcp.author_paper_details/1"),
        Envelope<OrcidWorks> => ("orcid-works", "biomcp.orcid_works/1")
    }
}
