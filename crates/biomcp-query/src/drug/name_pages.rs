//! Source-specific readers that gather candidates for drug name resolution.
#[path = "name_pages/fallback.rs"]
mod fallback;
pub(super) use fallback::{discover, ema};
#[path = "name_pages/native.rs"]
mod native;
pub(super) use native::{chembl, fda, label, mychem};
