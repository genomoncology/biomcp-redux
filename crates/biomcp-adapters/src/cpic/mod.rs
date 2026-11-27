//! Canonical pharmacogenomics publisher declaration; runtime dispatch arrives separately.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Cpic,
    base: "https://api.cpicpgx.org/v1",
    terms: Terms {
        name: "CPIC data use policy",
        url: "https://cpicpgx.org/about/terms-of-use/",
        usage: biomcp_types::Use::Open,
    },
    quota: &crate::quotas::CPIC,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
mod diplotype;
mod pair;
mod phenotypes;
mod raw;
mod read;
mod search;
pub use diplotype::{diplotype, diplotype_examples};
pub use pair::pair;
pub use phenotypes::{PhenotypeList, drug_phenotypes};
pub use read::{Page, card, drug_recommendations, guidelines, recommendations};
pub use search::search;

/// Native gene collection shared by record reads and the bounded health probe.
pub(crate) const GENE_PATH: &str = "/gene";

/// Complete source policy and command declaration.
pub(crate) mod decl;

// PostgREST imatch passes the pattern to PostgreSQL. Keep user text literal.
fn regex_literal(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        if r"\.^$|?*+()[]{}".contains(ch) {
            escaped.push('\');
        }
        escaped.push(ch);
    }
    escaped
}

fn case_insensitive(value: &str) -> String {
    format!("imatch.^{}$", regex_literal(value))
}
