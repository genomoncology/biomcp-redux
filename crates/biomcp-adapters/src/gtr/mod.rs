//! Canonical diagnostic publisher declaration; runtime dispatch belongs to later slices.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::Gtr,
    base: "https://ftp.ncbi.nlm.nih.gov/pub/GTR",
    terms: Terms {
        name: "NCBI usage policies; submitter and third-party rights retained",
        url: "https://www.ncbi.nlm.nih.gov/home/about/policies/",
        usage: Use::Open,
    },
    quota: &crate::quotas::GTR,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
