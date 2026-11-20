//! FDA Drug Safety Communications: FDA's index page, a shipped seed of what each page links,
//! and one communication's page read live.
pub(crate) mod decl;
mod html;
mod page;
mod read;
mod seed;
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
pub use read::{Communications, communication, communications, seed_date};
use std::time::Duration;

/// FDA's website, which serves the index and every page.
pub(crate) const SITE: &str = "https://www.fda.gov";
/// FDA's index of Drug Safety Communications.
pub(crate) const INDEX_PATH: &str =
    "/drugs/drug-safety-and-availability/drug-safety-communications";

/// FDA's website policies: "Unless otherwise noted, the contents of the FDA website
/// (www.fda.gov) — both text and graphics — are not copyrighted. They are in the public domain
/// and may be republished, reprinted and otherwise used freely by anyone without the need to
/// obtain permission from FDA." The client names itself, as FDA's robots.txt expects.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: Some(concat!(
        "biomcp/",
        env!("CARGO_PKG_VERSION"),
        " (+https://github.com/genomoncology/biomcp)"
    )),
    source: Source::FdaDsc,
    base: "https://www.fda.gov",
    terms: Terms {
        name: "FDA website policies: www.fda.gov content is public domain unless otherwise noted",
        url: "https://www.fda.gov/about-fda/about-website/website-policies",
        usage: Use::Open,
    },
    quota: &crate::quotas::FDA_DSC,
    key: KeyUse::None,
    // A get may read the index and then the page, 30 seconds apart.
    deadline: Duration::from_secs(45),
    command_budget: Some(Duration::from_secs(45)),
};
