//! Disease service declarations and independently labeled carried content.
/// Shared component policy applied before request, cache or replay admission.
pub mod components;
/// Canonical OLS service declaration.
pub use crate::ols::SPEC as OLS;
/// Canonical MyDisease service declaration.
pub use crate::mydisease::SPEC as MYDISEASE;
/// Canonical Monarch service declaration.
pub use crate::monarch::SPEC as MONARCH;
/// Canonical SEER service declaration.
pub use crate::seer::SPEC as SEER;
