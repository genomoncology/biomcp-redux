//! Checked source replies mapped into shared human gene values.
pub mod hgnc;
mod mapped;
pub mod mygene;
/// Keyed OncoKB gene annotations.
pub mod oncokb;
pub mod pubmed;
mod quotas;
mod raw;
pub use mapped::Mapped;

pub mod gnomad;
pub mod uniprot;

pub mod reactome;

pub mod civic;

pub mod clingen;

pub mod hpa;

pub mod europepmc;
