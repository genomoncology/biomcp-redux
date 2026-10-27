//! Canonical pharmacogenomics publisher declaration; runtime dispatch arrives separately.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::Source;
use std::time::Duration;
/// Publisher policy for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    source: Source::ClinPgx,
    base: "https://api.clinpgx.org/v1",
    terms: Terms {
        name: crate::pgx_sources::TERMS.name,
        url: crate::pgx_sources::TERMS.url,
        usage: crate::pgx_sources::TERMS.usage,
    },
    quota: &crate::quotas::CLINPGX,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};

mod read;
pub use read::{annotation, clinical, references};

/// Small read-only publisher probe; never an export or synchronization.
pub(crate) const HEALTH_PATH: &str = "/data/clinicalAnnotation";
