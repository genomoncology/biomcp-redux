//! Stored FDA companion diagnostic tables with source-owned association selectors.
mod bundle;
mod html;
mod identity;
mod raw;
mod search;
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
pub(crate) use bundle::state as bundle_state;
pub use bundle::sync;
pub use search::{Search, search};
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::FdaCdx,
    base: "https://www.fda.gov",
    terms: Terms {
        name: "FDA website policies; third-party exceptions retained",
        url: "https://www.fda.gov/about-fda/about-website/website-policies",
        usage: Use::Open,
    },
    quota: &crate::quotas::FDA_WEBSITE,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};

fn bad() -> biomcp_fetch::RequestError {
    biomcp_fetch::SourceError {
        source: Source::FdaCdx,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}

/// Small read-only publisher probe; never an export or synchronization.
pub(crate) const HEALTH_PATH: &str = "/about-fda/about-website/website-policies";
