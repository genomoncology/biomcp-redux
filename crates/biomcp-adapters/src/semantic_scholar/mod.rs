//! Semantic Scholar graph metadata under one shared optional-key quota.
use biodata::Source;
use biomcp_fetch::{Credential, KeyPlace, KeyUse, SecretName, SourceSpec};
use std::time::Duration;
/// Shared policy for search, paper records, graph edges and recommendations.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::SemanticScholar,
    base: "https://api.semanticscholar.org/graph/v1",
    key: KeyUse::Optional(Credential {
        secret: SecretName::SemanticScholarApiKey,
        place: KeyPlace::Header("x-api-key"),
    }),
    quota: &crate::quotas::SEMANTIC_SCHOLAR,
    deadline: Duration::from_secs(15),
};

mod search;
pub use search::{Paper, Search, search};

mod graph;
pub use graph::{Edge, Edges, PaperSummary, edges, summary};
