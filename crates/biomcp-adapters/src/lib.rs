//! Checked source replies mapped into shared human gene values.
pub mod hgnc;
mod mapped;
pub mod mygene;
mod quotas;
/// Keyed OncoKB gene annotations.
pub mod oncokb;
mod raw;
pub use mapped::Mapped;

pub mod gnomad;
