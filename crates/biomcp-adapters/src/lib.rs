//! Checked source replies mapped into shared human gene values.
pub mod hgnc;
pub mod pubmed;
mod mapped;
pub mod mygene;
/// Keyed OncoKB gene annotations.
pub mod oncokb;
mod quotas;
mod raw;
pub use mapped::Mapped;

pub mod gnomad;
pub mod uniprot;

pub mod reactome;

pub mod civic;

pub mod clingen;

pub mod hpa;
