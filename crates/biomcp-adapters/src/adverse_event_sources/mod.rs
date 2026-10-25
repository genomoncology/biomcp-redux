//! Adverse event services retain independent third-party terminology policy.
/// Component declarations and whole-leg admission before I/O.
pub mod components;
/// Canonical FAERS service declaration.
pub use crate::faers::SPEC as FAERS;
/// Canonical MAUDE service declaration.
pub use crate::maude::SPEC as MAUDE;

// Both report services share the publisher's exceptions, without granting component rights.
pub(crate) const FDA_TERMS: biomcp_fetch::Terms = biomcp_fetch::Terms {
    name: "openFDA terms and license; third-party exceptions",
    url: "https://open.fda.gov/terms/",
    usage: biomcp_types::Use::Open,
};
