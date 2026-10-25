//! Canonical diagnostic publisher declaration; runtime dispatch belongs to later slices.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// One publisher declaration for health, outcomes and admission before any I/O.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::FdaPma,
    base: "https://api.fda.gov/device/pma.json",
    terms: Terms {
        name: "openFDA terms and license; third-party exceptions",
        url: "https://open.fda.gov/terms/",
        usage: Use::Open,
    },
    quota: &crate::quotas::OPENFDA,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
