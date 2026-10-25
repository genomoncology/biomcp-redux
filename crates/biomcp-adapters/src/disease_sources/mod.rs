//! Disease service declarations and independently labeled carried content.
/// Shared component policy applied before request, cache or replay admission.
pub mod components;
/// Canonical Monarch service declaration.
pub use crate::monarch::SPEC as MONARCH;
/// Canonical MyDisease service declaration.
pub use crate::mydisease::SPEC as MYDISEASE;
/// Canonical OLS service declaration.
pub use crate::ols::SPEC as OLS;
/// Canonical SEER service declaration.
pub use crate::seer::SPEC as SEER;
