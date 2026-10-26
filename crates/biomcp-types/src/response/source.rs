use crate::{Reason, Source};
use serde::{Deserialize, Serialize};
biodata::wire_enum! {
/// State of a source actually asked by the command.
SourceState {
    /// Checked data is current.
    Ok => "ok",
    /// Checked stale data answered.
    Stale => "stale",
    /// Another process is synchronizing the bundle.
    Syncing => "syncing",
    /// The source failed.
    Failed => "failed",
    /// User settings prevented the source request.
    Skipped => "skipped",
}
}
/// One source's outcome, without provider text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SourceOutcome {
    /// Asked source.
    pub source: Source,
    /// Reviewed source use category; publisher terms remain authoritative.
    #[serde(rename = "use")]
    pub usage: crate::Use,
    /// Publisher terms name from the source registry.
    pub terms: Box<str>,
    /// Publisher page documenting the terms.
    pub terms_url: Box<str>,
    /// Retained state.
    pub state: SourceState,
    /// Failure or stale-fallback reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    /// Exact provider search term.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<crate::Text>,
    /// Provider matching count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    /// Retrieved identity depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetched: Option<u32>,
    /// Provider query notices that do not indicate failure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notices: Vec<SourceNotice>,
}

/// A provider query interpretation notice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SourceNotice {
    /// A checked provider answer survived a local cache failure.
    CacheProblem {
        /// Bounded local failure without paths or database diagnostics.
        reason: CacheProblem,
    },
    /// The source still has graph pages after the bounded fetch window.
    GraphTruncated {
        /// Default-source command offering another graph view.
        fallback: crate::Text,
    },
    /// PubMed did not index these words.
    NotFoundWords {
        /// Whole source words.
        words: Box<[crate::Text]>,
    },
}

biodata::wire_enum! {
/// A local cache failure that leaves the provider outcome intact.
CacheProblem {
    /// The cache connection or file was busy.
    Busy => "busy",
    /// The cache could not store or remove a reply.
    Database => "database",
    /// Cache ownership or schema validation failed.
    Version => "version",
}
}
