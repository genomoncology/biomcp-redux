//! Publisher policy for CPIC and ClinPGx; dispatch belongs to the owning adapters.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
// The captured policy page was a shell, so this names the PM's reviewed product ruling.
const TERMS: Terms = Terms {
    name: "ClinPGx data use policy; noncommercial by PM ruling; captured policy was a JavaScript shell",
    url: "https://www.clinpgx.org/page/dataUsagePolicy",
    usage: Use::Noncommercial,
};
/// CPIC remains a separate canonical source with the shared publisher policy.
pub static CPIC: SourceSpec = SourceSpec {
    source: Source::Cpic,
    base: "https://api.cpicpgx.org/v1",
    terms: TERMS,
    quota: &crate::quotas::CPIC,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
/// ClinPGx replaces the historical PharmGKB spelling without registering an alias.
pub static CLINPGX: SourceSpec = SourceSpec {
    source: Source::ClinPgx,
    base: "https://api.clinpgx.org/v1",
    terms: TERMS,
    quota: &crate::quotas::CLINPGX,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
